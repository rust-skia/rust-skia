#!/usr/bin/env bash
set -euo pipefail

# Advisory comparison of the SVG and PDF outputs of the skia-org example between
# two commits, across every platform that produces QA artifacts. Unlike
# compare-images.sh (the authoritative CPU pixel oracle), these files are
# rasterized with third-party tools (rsvg-convert, Ghostscript) that are not
# Skia, so a difference here only indicates the vector content changed as seen
# through a foreign renderer. Treat results as advisory.
#
# Exit status: 0 = exact match, 1 = differences to review, 2 = could not run.

previous="${1:?previous version is required}"
commit="${2:?release commit is required}"
formats="${3:-svg pdf}"
report="${4:-/tmp/rust-skia-release-vector}"
# Space-separated list of "workflow:artifact" pairs, one per platform that
# generates QA artifacts. Override to check a subset.
platforms="${5:-linux-qa.yaml:skia-org-images-x86_64-unknown-linux-gnu macos-qa.yaml:skia-org-images-aarch64-apple-darwin windows-qa.yaml:skia-org-images-x86_64-pc-windows-msvc windows-arm-qa.yaml:skia-org-images-aarch64-pc-windows-msvc}"
repo="rust-skia/rust-skia"

command -v magick >/dev/null || { echo "ImageMagick is required" >&2; exit 2; }
command -v gh >/dev/null || { echo "GitHub CLI is required" >&2; exit 2; }
command -v rsvg-convert >/dev/null || { echo "librsvg (rsvg-convert) is required for SVG comparison" >&2; exit 2; }
command -v gs >/dev/null || { echo "Ghostscript (gs) is required for PDF comparison" >&2; exit 2; }
[[ ! -e "$report" || -z "$(find "$report" -mindepth 1 -print -quit)" ]] || {
    echo "Report directory must be new or empty: $report" >&2
    exit 2
}

# shellcheck source=compare-common.sh
source "$(dirname "$0")/compare-common.sh"
resolve_shas

comparison="$report/comparison"
mkdir -p "$comparison/diff" "$comparison/review" "$comparison/tmp"
printf 'status\tpixels\tdimensions\tplatform\tpath\n' >"$comparison/summary.tsv"
differences=0
compared_any=0

# Rasterize an SVG or PDF to a PNG for pixel comparison.
prepare_image() {
    local src="$1" ext="$2" tag="$3" subdir="$4" path="$5"
    local dst="$comparison/tmp/${tag}_${subdir}_${path//\//_}.png"
    case "$ext" in
        svg) rsvg-convert -o "$dst" "$src" ;;
        pdf) gs -q -dNOPAUSE -dBATCH -sDEVICE=pngalpha -r150 -sOutputFile="$dst" "$src" ;;
    esac
    echo "$dst"
}

compare_platform() {
    local tag="$1"
    for fmt in $formats; do
        if compare_format "$fmt" "$fmt" "$tag"; then
            compared_any=1
        fi
    done
}

for_each_platform

[[ $compared_any -eq 1 ]] || {
    echo "Could not locate any comparable vector trees" >&2
    exit 2
}

echo "Differences requiring review: $differences"
echo "Summary: $comparison/summary.tsv"
[[ $differences -eq 0 ]] || exit 1
