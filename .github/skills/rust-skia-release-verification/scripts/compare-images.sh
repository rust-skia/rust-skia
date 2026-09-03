#!/usr/bin/env bash
set -euo pipefail

# Authoritative comparison of the CPU-rendered PNG outputs of the skia-org
# example between two commits, across every platform that produces QA
# artifacts. These images are rendered by Skia itself, so a difference is a
# genuine regression.
#
# Exit status: 0 = exact match, 1 = differences to review, 2 = could not run.

previous="${1:?previous version is required}"
commit="${2:?release commit is required}"
report="${3:-/tmp/rust-skia-release-images}"
# Space-separated list of "workflow:artifact" pairs, one per platform that
# generates QA artifacts. Override to check a subset.
platforms="${4:-linux-qa.yaml:skia-org-images-x86_64-unknown-linux-gnu macos-qa.yaml:skia-org-images-aarch64-apple-darwin windows-qa.yaml:skia-org-images-x86_64-pc-windows-msvc windows-arm-qa.yaml:skia-org-images-aarch64-pc-windows-msvc}"
repo="rust-skia/rust-skia"

command -v magick >/dev/null || { echo "ImageMagick is required" >&2; exit 2; }
command -v gh >/dev/null || { echo "GitHub CLI is required" >&2; exit 2; }
[[ ! -e "$report" || -z "$(find "$report" -mindepth 1 -print -quit)" ]] || {
    echo "Report directory must be new or empty: $report" >&2
    exit 2
}

# shellcheck source=compare-common.sh
source "$(dirname "$0")/compare-common.sh"
resolve_shas

comparison="$report/comparison"
mkdir -p "$comparison/diff" "$comparison/review"
printf 'status\tpixels\tdimensions\tplatform\tpath\n' >"$comparison/summary.tsv"
differences=0
compared_any=0

# CPU PNGs are compared directly; no rasterization needed.
prepare_image() {
    echo "$1"
}

compare_platform() {
    local tag="$1"
    local bdir="$report/$tag/baseline/cpu" cdir="$report/$tag/candidate/cpu"

    if [[ -d "$bdir" && -d "$cdir" ]]; then
        compared_any=1
        compare_format "cpu" "png" "$tag" "$bdir" "$cdir"
    else
        echo "Could not locate both image trees for $tag" >&2
        exit 2
    fi
}

for_each_platform

[[ $compared_any -eq 1 ]] || {
    echo "No platforms were compared" >&2
    exit 2
}

echo "Differences requiring review: $differences"
echo "Summary: $comparison/summary.tsv"
[[ $differences -eq 0 ]] || exit 1
