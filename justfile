# Platforms that produce QA artifacts, as "workflow:artifact" pairs.
qa_platforms := "linux-qa.yaml:skia-org-images-x86_64-unknown-linux-gnu macos-qa.yaml:skia-org-images-aarch64-apple-darwin windows-qa.yaml:skia-org-images-x86_64-pc-windows-msvc windows-arm-qa.yaml:skia-org-images-aarch64-pc-windows-msvc"

# Features of the documentation built on macOS.
doc_features_mac := "gl,vulkan,metal,textlayout,svg,skottie,ureq,webp"

# Features that docs.rs builds with. The published documentation bindings use them.
doc_features_docs_rs := "ganesh,gl,textlayout,svg,skottie,ureq,webp,vulkan"

# Features of the local build test.
local_build_features := "gl,vulkan,webp,textlayout"

# A shell command that prints the path of the most recently generated `bindings.rs`.
bindings_latest := 'find target -name "bindings.rs" -exec ls -t {} + | head -n 1'

# List the recipes.
default:
    @just --list --unsorted

code-macos:
    code .vscode/rust-skia-macos.code-workspace

code-graphite:
    code .vscode/rust-skia-graphite.code-workspace

code-macos-gl:
    code .vscode/rust-skia-macos-gl.code-workspace

code-windows:
    code .vscode/rust-skia-windows.code-workspace

check-skia-submodule-tag:
    #!/usr/bin/env bash
    set -euo pipefail

    expected_tag="$(sed -nE 's/^skia = "([^"]+)"/\1/p' skia-bindings/Cargo.toml | head -n1)"
    if [[ -z "$expected_tag" ]]; then
        echo "Could not find [package.metadata].skia in skia-bindings/Cargo.toml" >&2
        exit 1
    fi

    # The recorded tag must be among the tags at the submodule HEAD. Both the current
    # and the previous tag scheme are accepted, so older commits still pass.
    if ! git -C skia-bindings/skia tag --points-at HEAD | grep -Fqx "$expected_tag"; then
        echo "Mismatch: skia-bindings/Cargo.toml expects '$expected_tag' but skia submodule HEAD ($(git -C skia-bindings/skia rev-parse --short HEAD)) is not tagged with it" >&2
        exit 1
    fi

    echo "OK: skia submodule tag matches metadata tag ($expected_tag)"

# Check the third-party commits pinned in Skia's DEPS file against OSV.dev for known vulnerabilities.
audit-skia-deps:
    python3 .github/scripts/audit-skia-deps.py

# Verify the release commit, workflows/assets, binary downloads, and source builds.
release-verify version previous commit remote="upstream":
    bash .github/scripts/release.sh verify "{{ version }}" "{{ previous }}" "{{ commit }}" "{{ remote }}"

# Download matching QA artifacts and compare all generated images across every
# platform that produces QA artifacts.
release-verify-images previous commit report="/tmp/rust-skia-release-images" platforms="" runs="":
    bash .github/skills/rust-skia-release-verification/scripts/compare-images.sh "{{ previous }}" "{{ commit }}" "{{ report }}" "{{ if platforms == "" { qa_platforms } else { platforms } }}" "{{ runs }}"

# Advisory comparison of the SVG and PDF outputs between two commits, across
# every platform that produces QA artifacts.
release-verify-vector previous commit formats="svg pdf" report="/tmp/rust-skia-release-vector" platforms="" runs="":
    bash .github/skills/rust-skia-release-verification/scripts/compare-vector.sh "{{ previous }}" "{{ commit }}" "{{ formats }}" "{{ report }}" "{{ if platforms == "" { qa_platforms } else { platforms } }}" "{{ runs }}"

# Publish required crates in dependency order, verify them, and run smoke tests.
release-publish-crates version previous:
    bash .github/scripts/release.sh publish-crates "{{ version }}" "{{ previous }}"

# Create or resume the GitHub release and verify its tag.
release-publish-github version commit notes prerelease="false" remote="upstream":
    bash .github/scripts/release.sh publish-github "{{ version }}" "{{ commit }}" "{{ notes }}" "{{ prerelease }}" "{{ remote }}"

# Mac Catalyst needs `build-std`, see https://github.com/rust-skia/rust-skia/issues/548.
# Build the Mac Catalyst targets with a nightly toolchain.
macos-qa:
    rustup update nightly
    cargo +nightly build -Z build-std --target x86_64-apple-ios-macabi --release
    cargo +nightly build -Z build-std --target aarch64-apple-ios-macabi --release

# Check that skia-bindings packages and builds, from prebuilt binaries and from source.
crate-tests: crate-bindings-binaries crate-bindings-build

# Dry-run the skia-bindings publication with downloaded binaries, with and without GPU features.
crate-bindings-binaries:
    #!/usr/bin/env bash
    set -euxo pipefail
    export FORCE_SKIA_BINARIES_DOWNLOAD=1
    cd skia-bindings
    cargo publish -vv --dry-run --features "ganesh,gl,vulkan,textlayout,binary-cache"
    cargo publish -vv --dry-run

# Packaging skips verification because the source build modifies the extracted Skia tree.
# Build the packaged skia-bindings crate from source, with and without GPU features.
crate-bindings-build:
    #!/usr/bin/env bash
    set -euxo pipefail
    export FORCE_SKIA_BUILD=1
    for features in "ganesh,gl,vulkan,textlayout" ""; do
        rm -rf target/package/skia-bindings-*
        (cd skia-bindings && cargo package -vv --no-verify)
        (cd target/package && tar xzf skia-bindings-*.crate)
        cargo build -vv --manifest-path target/package/skia-bindings-*/Cargo.toml --features "$features"
    done

# The Emscripten build needs `emcc` on the `PATH`. It covers
# https://github.com/rust-skia/rust-skia/issues/1310.
# Install `skia-safe@=<version>` from crates.io, then run it natively and build it for Emscripten.
crate-post-release-test version:
    rm -rf /tmp/skia-test
    cd /tmp && cargo new skia-test
    cd /tmp/skia-test && cargo add "skia-safe@={{ version }}"
    cd /tmp/skia-test && cargo run
    cd /tmp/skia-test && cargo build --target wasm32-unknown-emscripten

# Without `bindings_docs.rs`, `DOCS_RS=1` builds of downstream crates fail when they copy the
# bindings. See https://github.com/rust-skia/rust-skia/issues/720.
# Publish skia-bindings with the documentation bindings in its tarball.
publish-bindings-docs: bindings-docs
    cp /tmp/bindings.rs skia-bindings/bindings_docs.rs
    cd skia-bindings && cargo publish -vv --no-verify --allow-dirty
    rm skia-bindings/bindings_docs.rs

# Publish skia-svg-macros. release.sh runs this only when the crate changed.
publish-svg-macros:
    cd skia-svg-macros && cargo publish -vv

# Publish skia-safe.
publish-safe:
    cd skia-safe && cargo publish -vv --no-verify --allow-dirty

# Generate the bindings with the docs.rs features and copy them to `/tmp/bindings.rs`.
bindings-docs:
    cargo build -vv --features {{ doc_features_docs_rs }}
    cp "$({{ bindings_latest }})" /tmp/bindings.rs

# Build the skia-safe documentation and push it to rust-skia.github.io.
update-doc:
    cargo clean
    rm -rf rust-skia.github.io
    git clone git@github.com:rust-skia/rust-skia.github.io.git
    cd skia-safe && cargo doc --no-deps --lib --features {{ doc_features_mac }}
    cp -r target/doc rust-skia.github.io/
    cd rust-skia.github.io && git add --all
    cd rust-skia.github.io && git commit -m"Auto-Update of /doc" || true
    cd rust-skia.github.io && git push origin master
    rm -rf rust-skia.github.io

# Build the documentation with the macOS features.
doc:
    cargo doc --no-deps --features {{ doc_features_mac }}

# Run all tests that can run on macOS with the full macOS feature set.
test-macos:
    cargo test -p skia-safe --features "all-macos,ureq" --lib
    cargo test -p skia-safe --features "all-macos,ureq" --tests
    cargo build -p skia-safe --features "all-macos,ureq" --examples

# Regenerate the GitHub workflows from `mk-workflows`.
workflows:
    cargo run -p mk-workflows

# The libraries are passed through `SKIA_BUILD_DEFINES` and `SKIA_LIBRARY_SEARCH_PATH`.
# See https://github.com/rust-skia/rust-skia/pull/527.
# Test a build that links locally built Skia libraries.
test-local-build: prepare-local-build build-local-build

# Build the Skia libraries and copy them to `tmp/`.
prepare-local-build:
    cargo clean
    cargo build --release --features {{ local_build_features }}
    rm -rf tmp/
    mkdir -p tmp/
    find target -name "libsk*.a" -type f -exec cp {} tmp/ \;
    find target -name "libicu.a" -type f -exec cp {} tmp/ \;
    find target -name "skia-defines.txt" -type f -exec cp {} tmp/ \;
    # Windows
    find target -name "sk*.lib" -type f -exec cp {} tmp/ \;
    find target -name "icu.lib" -type f -exec cp {} tmp/ \;
    find target -name "icudtl.dat" -type f -exec cp {} tmp/ \;
    # The bindings are expected to be regenerated in a local build.
    rm tmp/*-bindings.*

# Build against the libraries in `tmp/`.
build-local-build:
    cargo clean
    SKIA_SOURCE_DIR="{{ justfile_directory() }}/skia-bindings/skia" SKIA_BUILD_DEFINES="$(cat tmp/skia-defines.txt)" SKIA_LIBRARY_SEARCH_PATH="{{ justfile_directory() }}/tmp" cargo build --release --no-default-features -vv --features {{ local_build_features }}

# A Skia fork patch stack is every commit on top of the upstream chrome/mXX branch named by
# `[package.metadata].skia`, so edited, added, and dropped patches show up regardless of their
# subject. By default it prints one line per patch, marked `=` (unchanged), `!` (edited), `<`
# (dropped), or `>` (added). Run `just diff-skia ""` to see the patch diffs behind the `!` lines.
# Diff the Skia fork patch stack of the submodule checkout with the one master records.
diff-skia flags="--no-patch":
    #!/usr/bin/env bash
    set -euo pipefail
    milestone() { sed -nE 's/^skia = "m([0-9]+)[.-].*/\1/p'; }
    cd skia-bindings/skia
    master_tip="$(git -C ../.. rev-parse master:skia-bindings/skia)"
    master_base="$(git merge-base "$master_tip" "upstream/chrome/m$(git -C ../.. show master:skia-bindings/Cargo.toml | milestone)")"
    base="$(git merge-base HEAD "upstream/chrome/m$(milestone <../Cargo.toml)")"
    git range-diff {{ flags }} "$master_base..$master_tip" "$base..HEAD"

# The diff also lists what master changed since that release. It needs cargo public-api.
# Diff the public skia-safe API with the latest release on crates.io into `target/skia-safe-api.diff`.
diff-api:
    mkdir -p target
    cargo public-api -p skia-safe -s --features=all-macos diff latest -- >target/skia-safe-api.diff

# Print the path of the most recently generated `bindings.rs`.
locate-bindings:
    @{{ bindings_latest }}

# Open the most recently generated `bindings.rs` in VS Code.
open-bindings:
    code "$({{ bindings_latest }})"
