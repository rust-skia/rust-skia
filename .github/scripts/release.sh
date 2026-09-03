#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

die() {
    echo "release: $*" >&2
    exit 1
}

package_version() {
    awk '
        /^\[package\]$/ { package = 1; next }
        package && /^version[[:space:]]*=/ {
            value = $0
            sub(/^[^"]*"/, "", value)
            sub(/".*/, "", value)
            print value
            exit
        }
    ' "$1"
}

svg_dependency_version() {
    sed -nE 's/^skia-svg-macros.*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' \
        skia-safe/Cargo.toml
}

bindings_dependency_version() {
    sed -nE 's/^skia-bindings.*version[[:space:]]*=[[:space:]]*"=([^"]+)".*/\1/p' \
        skia-safe/Cargo.toml
}

svg_check() {
    local previous="$1"
    local current previous_svg dependency
    git rev-parse --verify "$previous^{commit}" >/dev/null
    current="$(package_version skia-svg-macros/Cargo.toml)"
    previous_svg="$(package_version <(git show "$previous:skia-svg-macros/Cargo.toml"))"
    dependency="$(svg_dependency_version)"
    [[ "$current" == "$dependency" ]] ||
        die "skia-safe requires $dependency but skia-svg-macros is $current"

    if git diff --quiet "$previous"..HEAD -- skia-svg-macros; then
        echo "skia-svg-macros: unchanged since $previous; no publication needed"
        return
    fi
    [[ "$current" != "$previous_svg" ]] ||
        die "skia-svg-macros changed but its version is still $current"
    echo "skia-svg-macros: publish $current before skia-safe (previously $previous_svg)"
}

svg_dry_run() {
    local previous="$1"
    svg_check "$previous"
    if ! git diff --quiet "$previous"..HEAD -- skia-svg-macros; then
        (cd skia-svg-macros && cargo publish --dry-run)
    fi
}

branches() {
    local remote="${1:-upstream}"
    local master release
    git fetch "$remote" master release
    master="$(git rev-parse "refs/remotes/$remote/master")"
    release="$(git rev-parse "refs/remotes/$remote/release")"
    [[ "$master" == "$release" ]] ||
        die "$remote/master (${master:0:12}) and $remote/release (${release:0:12}) differ"
    echo "OK: $remote/master and $remote/release point to $master"
}

tag_check() {
    local remote="$1" version="$2" commit="$3"
    commit="$(git rev-parse "$commit^{commit}")"
    git fetch "$remote" tag "$version"
    [[ "$(git rev-parse "$version^{commit}")" == "$commit" ]] ||
        die "tag $version does not point to $commit"
    echo "OK: $version points to $commit"
}

verify_release() {
    local version="$1" previous="$2" commit="$3" remote="$4"
    local runs assets workflow_file workflow count
    commit="$(git rev-parse "$commit^{commit}")"
    [[ "$(git rev-parse HEAD)" == "$commit" ]] ||
        die "local HEAD is not the release commit $commit"
    branches "$remote"
    [[ "$(git rev-parse "refs/remotes/$remote/master")" == "$commit" ]] ||
        die "$commit is not the release commit"

    runs="$(gh run list --repo rust-skia/rust-skia --branch release \
        --commit "$commit" --limit 30 \
        --json databaseId,name,status,conclusion,url)"
    jq -r '.[] | [.databaseId, .name, .status, .conclusion, .url] | @tsv' <<<"$runs"
    for workflow_file in .github/workflows/*-binaries.yaml; do
        workflow="$(sed -nE "s/^name: '([^']+)'/\1/p" "$workflow_file")"
        jq -e --arg workflow "$workflow" '
            any(.[]; .name == $workflow and .status == "completed" and .conclusion == "success")
        ' <<<"$runs" >/dev/null || die "$workflow did not complete successfully"
    done

    assets="$(gh release view "$version" --repo rust-skia/skia-binaries --json assets,url)"
    count="$(jq '.assets | length' <<<"$assets")"
    [[ "$count" -gt 0 ]] || die "binary release $version has no assets"
    jq -r '.url, (.assets[].name)' <<<"$assets"

    make crate-bindings-binaries
    make crate-bindings-build
    svg_dry_run "$previous"
}

wait_for_crate() {
    local crate="$1" version="$2"
    local attempt
    for attempt in {1..12}; do
        if cargo info --registry crates-io "$crate@$version" >/dev/null 2>&1; then
            echo "Available: $crate@$version"
            return
        fi
        echo "Waiting for $crate@$version ($attempt/12)"
        sleep 5
    done
    die "$crate@$version did not become available within 60 seconds"
}

publish_if_needed() {
    local crate="$1" version="$2"
    shift 2
    if cargo info --registry crates-io "$crate@$version" >/dev/null 2>&1; then
        echo "Already published: $crate@$version"
    else
        "$@"
    fi
    wait_for_crate "$crate" "$version"
}

publish_crates() {
    local version="$1" previous="$2"
    local bindings safe svg
    bindings="$(package_version skia-bindings/Cargo.toml)"
    safe="$(package_version skia-safe/Cargo.toml)"
    svg="$(package_version skia-svg-macros/Cargo.toml)"
    [[ "$bindings" == "$version" && "$safe" == "$version" ]] ||
        die "requested $version, manifests contain bindings=$bindings safe=$safe"
    [[ "$(bindings_dependency_version)" == "$version" ]] ||
        die "skia-safe does not require skia-bindings =$version"

    svg_check "$previous"
    if ! git diff --quiet "$previous"..HEAD -- skia-svg-macros; then
        publish_if_needed skia-svg-macros "$svg" make publish-svg-macros
    fi
    publish_if_needed skia-bindings "$version" make publish-bindings-docs
    publish_if_needed skia-safe "$version" make publish-safe
    make crate-post-release-test RELEASE_VERSION="$version"
}

publish_github() {
    local version="$1" commit="$2" notes="$3" prerelease="$4" remote="$5"
    local args=()
    if [[ "$prerelease" == true ]]; then
        args+=(--prerelease)
    elif [[ "$prerelease" != false ]]; then
        die "prerelease must be true or false"
    fi

    if gh release view "$version" --repo rust-skia/rust-skia >/dev/null 2>&1; then
        echo "GitHub release already exists: $version"
    else
        gh release create "$version" \
            --repo rust-skia/rust-skia \
            --target "$commit" \
            --title "$version" \
            --notes-file "$notes" \
            "${args[@]}"
    fi
    gh release view "$version" --repo rust-skia/rust-skia
    tag_check "$remote" "$version" "$commit"
}

case "${1:-}" in
    verify) verify_release "$2" "$3" "$4" "${5:-upstream}" ;;
    publish-crates) publish_crates "$2" "$3" ;;
    publish-github) publish_github "$2" "$3" "$4" "${5:-false}" "${6:-upstream}" ;;
    *) die "usage: $0 {verify|publish-crates|publish-github} ..." ;;
esac
