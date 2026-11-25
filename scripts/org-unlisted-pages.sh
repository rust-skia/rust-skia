#!/usr/bin/env bash
# Prints the skia.org pages with fiddles that `skia-org/README.md` lists neither as ported nor as
# skipped, and fails when there are any. Run it from the repository root.
set -euo pipefail

unlisted=0
while IFS= read -r page; do
    if ! grep -qF "\`$page\`" skia-org/README.md; then
        echo "$page"
        unlisted=1
    fi
done < <(cd skia-bindings/skia && grep -rlE 'fiddle-embed-sk|fiddle\.skia\.org/[ci]/' --include='*.md' site | sort)
exit $unlisted
