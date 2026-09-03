#!/usr/bin/env bash
# Shared helpers for the rust-skia release image/vector comparison scripts.
# This file is sourced, not executed. It defines functions and expects the
# caller to have set: report, platforms, repo, baseline_sha, release_sha,
# comparison, differences, compared_any, and a `prepare_image` function that
# maps a source file to a rasterized PNG for comparison.

# Resolve the baseline and release commit SHAs from version/commit refs.
resolve_shas() {
    baseline_sha="$(git rev-parse "$previous^{commit}")"
    release_sha="$(git rev-parse "$commit^{commit}")"
}

# Find the most recent successful run of a workflow at a commit.
find_run() {
    local sha="$1" workflow="$2"
    gh run list --repo "$repo" --workflow "$workflow" --commit "$sha" \
        --status success --limit 100 --json databaseId --jq '.[0].databaseId // empty'
}

# Resolve the run to use for a workflow: an explicit override from `runs`
# (space-separated "workflow:run" pairs) if present, otherwise the most recent
# successful run at the commit.
resolve_run() {
    local workflow="$1" run
    run="$(awk -v w="$workflow" -F: '$1 == w { print $2; exit }' <<<"$runs")"
    if [[ -n "$run" ]]; then
        echo "$run"
    else
        find_run "$2" "$workflow"
    fi
}

# Download an artifact from a successful run of a workflow at a commit.
download_images() {
    local sha="$1" output="$2" workflow="$3" artifact="$4" run
    run="$(resolve_run "$workflow" "$sha")"
    [[ -n "$run" ]] || {
        echo "No successful $workflow run found for $sha" >&2
        exit 2
    }
    echo "Downloading $artifact from $workflow run $run ($sha)"
    mkdir -p "$output"
    gh run download "$run" --repo "$repo" --name "$artifact" --dir "$output"
}

# Compare the files of one output format between the baseline and candidate
# trees of one platform. The caller's `prepare_image` maps each source file to
# a PNG for comparison. Returns 0 if a comparison ran, 1 if the format is
# absent from either tree and was skipped.
compare_format() {
    local subdir="$1" ext="$2" tag="$3"
    local bdir="${4:-$report/$tag/baseline/$subdir}"
    local cdir="${5:-$report/$tag/candidate/$subdir}"
    [[ -d "$bdir" && -d "$cdir" ]] || return 1

    find "$bdir" -type f -name "*.$ext" -print | sed "s|^$bdir/||" | sort >"$report/${tag}-${subdir}-baseline.txt"
    find "$cdir" -type f -name "*.$ext" -print | sed "s|^$cdir/||" | sort >"$report/${tag}-${subdir}-candidate.txt"
    comm -12 "$report/${tag}-${subdir}-baseline.txt" "$report/${tag}-${subdir}-candidate.txt" >"$report/${tag}-${subdir}-common.txt"

    while IFS= read -r path; do
        [[ -n "$path" ]] || continue
        printf 'removed\t-\t-\t%s\t%s/%s\n' "$tag" "$subdir" "$path" >>"$comparison/summary.tsv"
        differences=$((differences + 1))
    done < <(comm -23 "$report/${tag}-${subdir}-baseline.txt" "$report/${tag}-${subdir}-candidate.txt")

    while IFS= read -r path; do
        [[ -n "$path" ]] || continue
        printf 'added\t-\t-\t%s\t%s/%s\n' "$tag" "$subdir" "$path" >>"$comparison/summary.tsv"
        differences=$((differences + 1))
    done < <(comm -13 "$report/${tag}-${subdir}-baseline.txt" "$report/${tag}-${subdir}-candidate.txt")

    while IFS= read -r path; do
        [[ -n "$path" ]] || continue
        old="$bdir/$path"
        new="$cdir/$path"
        old_img="$(prepare_image "$old" "$ext" "$tag" "$subdir" "$path")"
        new_img="$(prepare_image "$new" "$ext" "$tag" "$subdir" "$path")"

        old_size="$(magick identify -format '%wx%h' "$old_img")"
        new_size="$(magick identify -format '%wx%h' "$new_img")"
        if [[ "$old_size" != "$new_size" ]]; then
            printf 'size-changed\t-\t%s->%s\t%s\t%s/%s\n' "$old_size" "$new_size" "$tag" "$subdir" "$path" >>"$comparison/summary.tsv"
            differences=$((differences + 1))
            continue
        fi

        diff_path="$comparison/diff/$tag/${path%.$ext}.png"
        review_path="$comparison/review/$tag/${path%.$ext}.png"
        mkdir -p "$(dirname "$diff_path")" "$(dirname "$review_path")"
        set +e
        metric="$(magick compare -metric AE "$old_img" "$new_img" "$diff_path" 2>&1)"
        status=$?
        set -e
        if [[ $status -eq 0 ]]; then
            printf 'matched\t0\t%s\t%s\t%s/%s\n' "$old_size" "$tag" "$subdir" "$path" >>"$comparison/summary.tsv"
            rm -f "$diff_path"
        elif [[ $status -eq 1 ]]; then
            magick "$old_img" "$new_img" "$diff_path" +append "$review_path"
            printf 'changed\t%s\t%s\t%s\t%s/%s\n' "$metric" "$old_size" "$tag" "$subdir" "$path" >>"$comparison/summary.tsv"
            differences=$((differences + 1))
        else
            echo "ImageMagick failed for $tag/$subdir/$path: $metric" >&2
            exit 2
        fi
    done <"$report/${tag}-${subdir}-common.txt"
    return 0
}

# Iterate over every platform, download its artifacts, and run the caller's
# `compare_platform` function for each. The caller's `compare_platform` must
# accept the tag and set compared_any when it compares something.
for_each_platform() {
    for platform in $platforms; do
        workflow="${platform%%:*}"
        artifact="${platform#*:}"
        tag="${artifact#skia-org-images-}"

        mkdir -p "$report/$tag"
        download_images "$release_sha" "$report/$tag/candidate" "$workflow" "$artifact"
        download_images "$baseline_sha" "$report/$tag/baseline" "$workflow" "$artifact"

        compare_platform "$tag"
    done
}
