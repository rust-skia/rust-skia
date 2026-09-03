#!/usr/bin/env bash
set -euo pipefail

previous="${1:?previous version is required}"
commit="${2:?release commit is required}"
report="${3:-/tmp/rust-skia-release-images}"
workflow="${4:-linux-qa.yaml}"
artifact="${5:-skia-org-images-x86_64-unknown-linux-gnu}"
website_fallback="${6:-false}"
repo="rust-skia/rust-skia"

command -v magick >/dev/null || { echo "ImageMagick is required" >&2; exit 2; }
command -v gh >/dev/null || { echo "GitHub CLI is required" >&2; exit 2; }
if [[ "$website_fallback" != true && "$website_fallback" != false ]]; then
    echo "website_fallback must be true or false" >&2
    exit 2
fi
[[ ! -e "$report" || -z "$(find "$report" -mindepth 1 -print -quit)" ]] || {
    echo "Report directory must be new or empty: $report" >&2
    exit 2
}

baseline_sha="$(git rev-parse "$previous^{commit}")"
release_sha="$(git rev-parse "$commit^{commit}")"

find_run() {
    local sha="$1"
    gh run list --repo "$repo" --workflow "$workflow" --commit "$sha" \
        --status success --limit 100 --json databaseId --jq '.[0].databaseId // empty'
}

download_images() {
    local sha="$1" output="$2" run
    run="$(find_run "$sha")"
    [[ -n "$run" ]] || {
        echo "No successful $workflow run found for $sha" >&2
        exit 2
    }
    echo "Downloading $artifact from $workflow run $run ($sha)"
    mkdir -p "$output"
    gh run download "$run" --repo "$repo" --name "$artifact" --dir "$output"
}

mkdir -p "$report"
download_images "$release_sha" "$report/candidate"
candidate="$report/candidate"
[[ -d "$candidate/cpu" ]] && candidate="$candidate/cpu"

if [[ "$website_fallback" == true ]]; then
    echo "Using the published website CPU images as a visual-only baseline"
    git clone --depth 1 https://github.com/rust-skia/rust-skia.github.io "$report/site"
    baseline="$report/site/skia-org/cpu"
else
    download_images "$baseline_sha" "$report/baseline"
    baseline="$report/baseline"
    [[ -d "$baseline/cpu" ]] && baseline="$baseline/cpu"
fi

[[ -d "$baseline" && -d "$candidate" ]] || {
    echo "Could not locate both image trees" >&2
    exit 2
}

comparison="$report/comparison"
mkdir -p "$comparison/diff" "$comparison/review"
find "$baseline" -type f -name '*.png' -print | sed "s|^$baseline/||" | sort >"$report/baseline.txt"
find "$candidate" -type f -name '*.png' -print | sed "s|^$candidate/||" | sort >"$report/candidate.txt"
comm -12 "$report/baseline.txt" "$report/candidate.txt" >"$report/common.txt"

printf 'status\tpixels\tdimensions\tpath\n' >"$comparison/summary.tsv"
differences=0

while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    printf 'removed\t-\t-\t%s\n' "$path" >>"$comparison/summary.tsv"
    differences=$((differences + 1))
done < <(comm -23 "$report/baseline.txt" "$report/candidate.txt")

while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    printf 'added\t-\t-\t%s\n' "$path" >>"$comparison/summary.tsv"
    differences=$((differences + 1))
done < <(comm -13 "$report/baseline.txt" "$report/candidate.txt")

while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    old="$baseline/$path"
    new="$candidate/$path"
    old_size="$(magick identify -format '%wx%h' "$old")"
    new_size="$(magick identify -format '%wx%h' "$new")"
    if [[ "$old_size" != "$new_size" ]]; then
        printf 'size-changed\t-\t%s->%s\t%s\n' "$old_size" "$new_size" "$path" >>"$comparison/summary.tsv"
        differences=$((differences + 1))
        continue
    fi

    mkdir -p "$comparison/diff/$(dirname "$path")" "$comparison/review/$(dirname "$path")"
    set +e
    metric="$(magick compare -metric AE "$old" "$new" "$comparison/diff/$path" 2>&1)"
    status=$?
    set -e
    if [[ $status -eq 0 ]]; then
        printf 'matched\t0\t%s\t%s\n' "$old_size" "$path" >>"$comparison/summary.tsv"
        rm -f "$comparison/diff/$path"
    elif [[ $status -eq 1 ]]; then
        magick "$old" "$new" "$comparison/diff/$path" +append "$comparison/review/$path"
        printf 'changed\t%s\t%s\t%s\n' "$metric" "$old_size" "$path" >>"$comparison/summary.tsv"
        differences=$((differences + 1))
    else
        echo "ImageMagick failed for $path: $metric" >&2
        exit 2
    fi
done <"$report/common.txt"

echo "Differences requiring review: $differences"
echo "Summary: $comparison/summary.tsv"
[[ $differences -eq 0 ]] || exit 1