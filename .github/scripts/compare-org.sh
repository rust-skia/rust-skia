#!/usr/bin/env bash
# Renders the skia-org examples with two raster drivers and prints each image that differs by more
# than `fuzz` per pixel, with its count of differing pixels. The differing images are put side by
# side, driver `a` left and `b` right, in `target/compare-org/sheet.png`. Needs ImageMagick. Run it
# from the repository root.
#
# Usage: compare-org.sh <a> <b> <features> <fuzz>
set -euo pipefail

a=$1
b=$2
features=$3
fuzz=$4
out=target/compare-org

rm -rf "$out"
cargo run -q -p skia-org --features "$features" -- "$out" --driver "$a" --driver "$b"

pairs=()
while IFS= read -r image; do
    if [[ ! -f "$out/$b/$image" ]]; then
        echo "missing in $b: $image"
        continue
    fi
    # `magick compare` exits with 1 when the images differ, so its status is ignored.
    differing=$(magick compare -metric AE -fuzz "$fuzz" "$out/$a/$image" "$out/$b/$image" null: 2>&1 | cut -d' ' -f1 || true)
    if [[ "$differing" != "0" ]]; then
        echo "$differing  $image"
        pairs+=("$out/$a/$image" "$out/$b/$image")
    fi
done < <(cd "$out/$a" && find . -name '*.png' | sed 's|^\./||' | sort)

if (( ${#pairs[@]} )); then
    montage "${pairs[@]}" -tile 2x -geometry +6+6 -background '#ddd' "$out/sheet.png"
    echo "Side by side: $out/sheet.png"
else
    echo "No image differs."
fi
