#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

die() {
    echo "release: $*" >&2
    exit 1
}

# The checks that passed, repeated by `summary` so that the end of a long build log shows them.
passed=()

ok() {
    passed+=("$*")
    echo "OK: $*"
}

summary() {
    echo
    echo "Summary:"
    printf 'OK: %s\n' "${passed[@]}"
}

# The packaged skia-bindings crate contains the Skia sources of the submodule checkout, so a
# modified tree or a checkout at a commit other than the one recorded would be published.
checkout_check() {
    [[ -z "$(git status --porcelain)" ]] ||
        die "the working tree has changes or the Skia submodule is not at the recorded commit"
    # The tag check looks at local tags, which `git submodule update` does not always fetch.
    git -C skia-bindings/skia fetch --tags --force --quiet
    just check-skia-submodule-tag
    ok "clean working tree with the Skia submodule at the recorded tag"
}

# The post-release smoke test builds for Emscripten and needs `emcc`. A non-interactive shell
# often lacks the emsdk environment, so it is loaded from `$EMSDK` or `~/emsdk`.
emsdk_check() {
    if ! command -v emcc >/dev/null; then
        local env="${EMSDK:-$HOME/emsdk}/emsdk_env.sh"
        [[ -f "$env" ]] || die "emcc is not on the PATH and $env does not exist"
        set +u
        # shellcheck source=/dev/null
        source "$env" >/dev/null 2>&1
        set -u
        command -v emcc >/dev/null || die "emcc is not on the PATH after sourcing $env"
    fi
    ok "emcc for the Emscripten smoke test: $(command -v emcc)"
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

# The exact version of the dependency named $1 in the manifest $2.
exact_dependency_version() {
    sed -nE "s/^$1.*version[[:space:]]*=[[:space:]]*\"=([^\"]+)\".*/\\1/p" "$2"
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
        ok "skia-svg-macros unchanged since $previous; no publication needed"
        return
    fi
    [[ "$current" != "$previous_svg" ]] ||
        die "skia-svg-macros changed but its version is still $current"
    ok "skia-svg-macros changed; publish $current before skia-safe (previously $previous_svg)"
}

svg_dry_run() {
    local previous="$1"
    svg_check "$previous"
    if ! git diff --quiet "$previous"..HEAD -- skia-svg-macros; then
        (cd skia-svg-macros && cargo publish --dry-run)
        ok "skia-svg-macros publication dry run"
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
    ok "$remote/master and $remote/release point to $master"
}

tag_check() {
    local remote="$1" version="$2" commit="$3"
    commit="$(git rev-parse "$commit^{commit}")"
    git fetch "$remote" tag "$version"
    [[ "$(git rev-parse "$version^{commit}")" == "$commit" ]] ||
        die "tag $version does not point to $commit"
    ok "$version points to $commit"
}

verify_release() {
    local version="$1" previous="$2" commit="$3" remote="$4"
    local runs assets workflow_file workflow count
    commit="$(git rev-parse "$commit^{commit}")"
    [[ "$(git rev-parse HEAD)" == "$commit" ]] ||
        die "local HEAD is not the release commit $commit"
    checkout_check
    emsdk_check
    branches "$remote"
    [[ "$(git rev-parse "refs/remotes/$remote/master")" == "$commit" ]] ||
        die "$commit is not the release commit"
    ok "local HEAD and $remote/master are the release commit $commit"

    runs="$(gh run list --repo rust-skia/rust-skia --branch release \
        --commit "$commit" --limit 30 \
        --json databaseId,name,status,conclusion,url)"
    jq -r '.[] | [.databaseId, .name, .status, .conclusion, .url] | @tsv' <<<"$runs"
    for workflow_file in .github/workflows/*-binaries.yaml; do
        workflow="$(sed -nE "s/^name: '([^']+)'/\1/p" "$workflow_file")"
        jq -e --arg workflow "$workflow" '
            any(.[]; .name == $workflow and .status == "completed" and .conclusion == "success")
        ' <<<"$runs" >/dev/null || die "$workflow did not complete successfully"
        ok "$workflow succeeded"
    done

    assets="$(gh release view "$version" --repo rust-skia/skia-binaries --json assets,url)"
    count="$(jq '.assets | length' <<<"$assets")"
    [[ "$count" -gt 0 ]] || die "binary release $version has no assets"
    jq -r '.url, (.assets[].name)' <<<"$assets"
    ok "binary release $version has $count assets"

    just crate-bindings-binaries
    ok "skia-bindings publication dry runs with downloaded binaries"
    just crate-bindings-build
    ok "skia-bindings package source builds"
    svg_dry_run "$previous"
    summary
}

# The registry is named explicitly, because an unqualified lookup resolves the local workspace
# package and would report an unpublished version as published.
crate_published() {
    cargo info --registry crates-io "$1@$2" >/dev/null 2>&1
}

wait_for_crate() {
    local crate="$1" version="$2"
    local attempt
    for attempt in {1..12}; do
        if crate_published "$crate" "$version"; then
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
    if crate_published "$crate" "$version"; then
        echo "Already published: $crate@$version"
    else
        "$@"
    fi
    wait_for_crate "$crate" "$version"
}

# The skia-bindings package carries the documentation bindings, and generating them builds Skia
# from source. Generating them before the first publication makes a damaged nested dependency
# checkout under skia-bindings/skia/third_party fail while nothing is published yet, instead of
# after skia-svg-macros is already on crates.io. A run that resumes after skia-bindings was
# published does not need them.
bindings_docs_check() {
    local version="$1"
    if crate_published skia-bindings "$version"; then
        echo "Already published: skia-bindings@$version; documentation bindings not needed"
        return
    fi
    rm -f /tmp/bindings.rs
    just bindings-docs || die "generating the documentation bindings failed; nothing was published"
    [[ -s /tmp/bindings.rs ]] || die "just bindings-docs did not produce /tmp/bindings.rs"
    ok "documentation bindings generated in /tmp/bindings.rs"
}

publish_crates() {
    local version="$1" previous="$2"
    local bindings wgpu safe svg
    bindings="$(package_version skia-bindings/Cargo.toml)"
    wgpu="$(package_version skia-wgpu/Cargo.toml)"
    safe="$(package_version skia-safe/Cargo.toml)"
    svg="$(package_version skia-svg-macros/Cargo.toml)"
    [[ "$bindings" == "$version" && "$wgpu" == "$version" && "$safe" == "$version" ]] ||
        die "requested $version, manifests contain bindings=$bindings wgpu=$wgpu safe=$safe"
    [[ "$(exact_dependency_version skia-bindings skia-safe/Cargo.toml)" == "$version" ]] ||
        die "skia-safe does not require skia-bindings =$version"
    [[ "$(exact_dependency_version skia-bindings skia-wgpu/Cargo.toml)" == "$version" ]] ||
        die "skia-wgpu does not require skia-bindings =$version"
    [[ "$(exact_dependency_version skia-wgpu skia-safe/Cargo.toml)" == "$version" ]] ||
        die "skia-safe does not require skia-wgpu =$version"
    # The FORCE_SKIA_* overrides of the calling shell are cleared, so that the check of the
    # documentation bindings and the builds that publish obtain Skia the same way.
    unset FORCE_SKIA_BUILD FORCE_SKIA_BINARIES_DOWNLOAD
    checkout_check
    emsdk_check
    bindings_docs_check "$version"

    svg_check "$previous"
    if ! git diff --quiet "$previous"..HEAD -- skia-svg-macros; then
        publish_if_needed skia-svg-macros "$svg" just publish-svg-macros
    fi
    publish_if_needed skia-bindings "$version" just publish-bindings-docs
    publish_if_needed skia-wgpu "$version" just publish-wgpu
    publish_if_needed skia-safe "$version" just publish-safe
    just crate-post-release-test "$version"
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
