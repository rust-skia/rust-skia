#!/usr/bin/env bash
#
# build-shared-skia.sh — Build a shared libskia.so + installed headers
# for use with rust-skia's SKIA_SHARED_LIB_PATH / SKIA_HEADER_DIR mode.
#
# This script downloads the correct Skia source (from the rust-skia fork),
# configures it with GN using is_component_build=true so that ninja
# produces .so files directly with proper symbol visibility (SK_API
# exported), and installs headers + libraries + defines to a
# user-specified prefix.
#
# Usage:
#   ./build-shared-skia.sh [--prefix /usr/local] [--jobs N] [--features FEATURES]
#
# Options:
#   --prefix DIR    Install destination (default: ./skia-install)
#   --jobs N        Parallel build jobs (default: nproc)
#   --features F    Comma-separated list of features to enable.
#                   Supported: textlayout, svg, gl, vulkan
#                   Default: textlayout,svg,gl,vulkan
#   --skia-dir DIR  Use an existing Skia source checkout instead of cloning
#   --help          Show this help
#
# Requirements:
#   - clang / clang++ (Skia's GN build system requires clang)
#   - ninja (or ninja-build)
#   - python3
#   - git
#   - System dev packages: libfreetype-dev, libfontconfig-dev, zlib,
#     libpng-dev, libjpeg-dev, libexpat-dev
#   - For textlayout: libicu-dev, libharfbuzz-dev
#
# After a successful build the script prints the environment variables
# and cargo command needed to build skia-safe against the produced library.

set -euo pipefail

# ── Defaults ──────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PREFIX="$(pwd)/skia-install"
JOBS="$(nproc 2>/dev/null || echo 4)"
FEATURES="textlayout,svg,gl,vulkan"
SKIA_DIR=""

# Skia version that matches the skia-bindings crate in this tree.
# (See skia-bindings/Cargo.toml → [package.metadata] skia = "...")
SKIA_TAG="m147-0.94.4"
SKIA_REPO="https://github.com/rust-skia/skia.git"

# ── Parse arguments ───────────────────────────────────────────────────

usage() {
    sed -n '2,/^$/s/^#//p' "$0"
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --prefix)   PREFIX="$2";   shift 2 ;;
        --jobs)     JOBS="$2";     shift 2 ;;
        --features) FEATURES="$2"; shift 2 ;;
        --skia-dir) SKIA_DIR="$2"; shift 2 ;;
        --help|-h)  usage ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

PREFIX="$(cd "$(dirname "$PREFIX")" 2>/dev/null && pwd)/$(basename "$PREFIX")" || PREFIX="$(realpath -m "$PREFIX")"

# Parse feature flags into booleans
feat_textlayout=false; feat_svg=false; feat_gl=false; feat_vulkan=false
IFS=',' read -ra _feats <<< "$FEATURES"
for f in "${_feats[@]}"; do
    case "$f" in
        textlayout) feat_textlayout=true ;;
        svg)        feat_svg=true ;;
        gl)         feat_gl=true ;;
        vulkan)     feat_vulkan=true ;;
        *) echo "Unknown feature: $f"; exit 1 ;;
    esac
done

# ── Dependency checks ─────────────────────────────────────────────────

check_cmd() { command -v "$1" &>/dev/null || { echo "ERROR: '$1' not found in PATH."; exit 1; }; }
check_cmd clang
check_cmd clang++
check_cmd python3
check_cmd git

# Accept either "ninja" or "ninja-build"
if command -v ninja &>/dev/null; then
    NINJA=ninja
elif command -v ninja-build &>/dev/null; then
    NINJA=ninja-build
else
    echo "ERROR: 'ninja' not found in PATH."
    exit 1
fi

echo "=== build-shared-skia.sh ==="
echo "  Skia tag:   $SKIA_TAG"
echo "  Features:   $FEATURES"
echo "  Prefix:     $PREFIX"
echo "  Jobs:       $JOBS"
echo ""

# ── Clone / update Skia ──────────────────────────────────────────────

if [[ -n "$SKIA_DIR" ]]; then
    echo ">>> Using existing Skia source: $SKIA_DIR"
else
    SKIA_DIR="$(pwd)/skia-src"
    if [[ -d "$SKIA_DIR/.git" ]]; then
        echo ">>> Skia source already present at $SKIA_DIR"
        cd "$SKIA_DIR"
        current_tag="$(git describe --tags --exact-match 2>/dev/null || true)"
        if [[ "$current_tag" != "$SKIA_TAG" ]]; then
            echo "    Checking out $SKIA_TAG ..."
            git fetch --tags
            git checkout "$SKIA_TAG"
        fi
        cd - >/dev/null
    else
        echo ">>> Cloning Skia ($SKIA_TAG) ..."
        git clone --depth 1 --branch "$SKIA_TAG" "$SKIA_REPO" "$SKIA_DIR"
    fi
fi

# ── Sync third-party deps ────────────────────────────────────────────

echo ">>> Syncing third-party dependencies ..."
cd "$SKIA_DIR"
GIT_SYNC_DEPS_PATH="$SKIA_DIR/DEPS" \
    GIT_SYNC_DEPS_SKIP_EMSDK=1 \
    python3 tools/git-sync-deps

# ── Assemble GN args ─────────────────────────────────────────────────

GN_ARGS=(
    "is_official_build=true"
    "is_component_build=true"
    "is_debug=false"

    # Disabled subsystems
    "skia_enable_skottie=false"
    "skia_enable_pdf=false"
    "skia_use_xps=false"
    "skia_use_dng_sdk=false"
    "skia_use_lua=false"
    "skia_use_libwebp_encode=false"
    "skia_use_libwebp_decode=false"

    # Use system libraries
    "skia_use_system_libpng=true"
    "skia_use_system_zlib=true"
    "skia_use_system_libjpeg_turbo=true"
    "skia_use_system_expat=true"
    "skia_use_expat=true"
    "skia_use_freetype=true"
    "skia_use_system_freetype2=true"

    # Compilers
    "cc=\"clang\""
    "cxx=\"clang++\""
)

# Ninja targets to build (is_component_build=true produces .so directly)
NINJA_TARGETS=( skia )

# GPU / Ganesh
if $feat_gl || $feat_vulkan; then
    GN_ARGS+=( "skia_enable_ganesh=true" )
else
    GN_ARGS+=( "skia_enable_ganesh=false" )
fi

if $feat_gl; then
    GN_ARGS+=( "skia_use_gl=true" "skia_use_x11=false" "skia_use_egl=false" )
else
    GN_ARGS+=( "skia_use_gl=false" )
fi

if $feat_vulkan; then
    GN_ARGS+=( "skia_use_vulkan=true" "skia_enable_spirv_validation=false" )
else
    GN_ARGS+=( "skia_use_vulkan=false" )
fi

# SVG
if $feat_svg; then
    GN_ARGS+=( "skia_enable_svg=true" )
    NINJA_TARGETS+=( svg )
else
    GN_ARGS+=( "skia_enable_svg=false" )
fi

# Text layout (paragraph, shaper, unicode)
if $feat_textlayout; then
    GN_ARGS+=(
        "skia_enable_skshaper=true"
        "skia_use_icu=true"
        "skia_use_system_icu=true"
        "skia_use_harfbuzz=true"
        "skia_pdf_subset_harfbuzz=true"
        "skia_use_system_harfbuzz=true"
        "skia_enable_skparagraph=true"
    )
    NINJA_TARGETS+=( skparagraph skshaper skunicode_core skunicode_icu )
else
    GN_ARGS+=( "skia_use_icu=false" "skia_use_harfbuzz=false" )
fi

OUT_DIR="$SKIA_DIR/out/Release"

# ── Configure ─────────────────────────────────────────────────────────

# Remove explicit complete_static_lib assignments that conflict with
# is_component_build=true (shared_library does not accept this variable).
# The set_defaults("component") in BUILDCONFIG.gn already handles the
# static-lib case correctly, so these overrides are only needed there.
echo ">>> Removing complete_static_lib overrides for component build ..."
for f in modules/svg/BUILD.gn modules/skparagraph/BUILD.gn; do
    if [[ -f "$SKIA_DIR/$f" ]]; then
        sed -i '/complete_static_lib\s*=/d' "$SKIA_DIR/$f"
    fi
done

echo ">>> Configuring Skia (GN) ..."
GN_ARGS_STR=""
for a in "${GN_ARGS[@]}"; do GN_ARGS_STR+="$a "; done

"$SKIA_DIR/bin/gn" gen "$OUT_DIR" --args="$GN_ARGS_STR"

# ── Build ─────────────────────────────────────────────────────────────

echo ">>> Building Skia (ninja, ${JOBS} jobs) ..."
$NINJA -j "$JOBS" -C "$OUT_DIR" "${NINJA_TARGETS[@]}"

# With is_component_build=true, ninja produces .so files directly —
# no manual --whole-archive linking step is needed.

# ── Extract preprocessor defines ─────────────────────────────────────

echo ">>> Extracting preprocessor defines ..."

NINJA_FILES=( obj/skia.ninja )

if $feat_gl || $feat_vulkan; then
    NINJA_FILES+=( obj/gpu.ninja )
fi
if $feat_textlayout; then
    NINJA_FILES+=(
        obj/modules/skshaper/skshaper.ninja
        obj/modules/skparagraph/skparagraph.ninja
        obj/modules/skunicode/skunicode_core.ninja
        obj/modules/skunicode/skunicode_icu.ninja
    )
fi
if $feat_svg; then
    NINJA_FILES+=( obj/modules/svg/svg.ninja )
fi

: > "$OUT_DIR/skia-defines.txt"
for f in "${NINJA_FILES[@]}"; do
    fpath="$OUT_DIR/$f"
    if [[ -f "$fpath" ]]; then
        sed -n 's/^defines = //p' "$fpath"
    fi
done | tr ' ' '\n' | sort -u | tr '\n' ' ' > "$OUT_DIR/skia-defines.txt"

echo "  defines: $(wc -w < "$OUT_DIR/skia-defines.txt") unique tokens"

# ── Install ───────────────────────────────────────────────────────────

echo ">>> Installing to $PREFIX ..."

install -d "$PREFIX/lib"

# Shared libraries produced by is_component_build=true
for t in "${NINJA_TARGETS[@]}"; do
    install -m 0755 "$OUT_DIR/lib${t}.so" "$PREFIX/lib/lib${t}.so"
done

# Defines file
install -d "$PREFIX/lib/skia"
install -m 0644 "$OUT_DIR/skia-defines.txt" "$PREFIX/lib/skia/skia-defines.txt"

# ── Headers ───────────────────────────────────────────────────────────
# The layout mirrors the Skia source tree so that include paths like
# "include/core/SkCanvas.h" and "modules/skparagraph/include/Paragraph.h"
# work when SKIA_HEADER_DIR points at $PREFIX/include/skia.

HDIR="$PREFIX/include/skia"
install -d "$HDIR"

# Public headers
cp -a "$SKIA_DIR/include" "$HDIR/"

# Module headers
for mod in skparagraph skshaper skunicode svg skresources; do
    if [[ -d "$SKIA_DIR/modules/$mod/include" ]]; then
        install -d "$HDIR/modules/$mod/include"
        cp -a "$SKIA_DIR/modules/$mod/include/." "$HDIR/modules/$mod/include/"
    fi
done

# skcms module headers (referenced by include/core/SkColorSpace.h)
install -d "$HDIR/modules/skcms/src"
cp -a "$SKIA_DIR/modules/skcms/skcms.h"           "$HDIR/modules/skcms/"
cp -a "$SKIA_DIR/modules/skcms/src/skcms_public.h" "$HDIR/modules/skcms/src/"

# Private headers transitively included by the public/module headers
# and by the skia-bindings C++ wrapper sources.
install -d "$HDIR/src/base"
for h in SkUTF.h SkTLazy.h SkTInternalLList.h SkMathPriv.h; do
    cp -a "$SKIA_DIR/src/base/$h" "$HDIR/src/base/"
done

install -d "$HDIR/src/core"
for h in SkFontDescriptor.h SkChecksum.h SkLRUCache.h SkTHash.h; do
    cp -a "$SKIA_DIR/src/core/$h" "$HDIR/src/core/"
done

install -d "$HDIR/src/gpu/ganesh/gl"
cp -a "$SKIA_DIR/src/gpu/ganesh/gl/GrGLDefines.h" "$HDIR/src/gpu/ganesh/gl/"

install -d "$HDIR/third_party/icu"
cp -a "$SKIA_DIR/third_party/icu/SkLoadICU.h" "$HDIR/third_party/icu/"

# ── Done ──────────────────────────────────────────────────────────────

echo ""
echo "============================================================"
echo " Skia shared library built and installed successfully!"
echo "============================================================"
echo ""
echo "  shared libs: $PREFIX/lib/lib*.so"
echo "  headers:     $PREFIX/include/skia/"
echo "  defines:     $PREFIX/lib/skia/skia-defines.txt"
echo ""
echo "To build skia-safe against this library, run:"
echo ""
echo "  export SKIA_SHARED_LIB_PATH=\"$PREFIX/lib\""
echo "  export SKIA_HEADER_DIR=\"$PREFIX/include/skia\""
echo "  export SKIA_BUILD_DEFINES=\"\$(cat $PREFIX/lib/skia/skia-defines.txt)\""
echo "  export LD_LIBRARY_PATH=\"$PREFIX/lib\${LD_LIBRARY_PATH:+:\$LD_LIBRARY_PATH}\""
echo ""
echo "  cargo build -p skia-safe --release \\"
echo "      --features ${FEATURES} --no-default-features --example hello"
echo ""
