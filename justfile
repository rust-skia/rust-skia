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

    actual_tag="$(git -C skia-bindings/skia tag --points-at HEAD | grep -E '^m[0-9]+-' | head -n1 || true)"
    if [[ -z "$actual_tag" ]]; then
        echo "No milestone tag found at skia-bindings/skia HEAD ($(git -C skia-bindings/skia rev-parse --short HEAD))" >&2
        exit 1
    fi

    if [[ "$actual_tag" != "$expected_tag" ]]; then
        echo "Mismatch: skia-bindings/Cargo.toml expects '$expected_tag' but skia submodule is at '$actual_tag'" >&2
        exit 1
    fi

    echo "OK: skia submodule tag matches metadata tag ($expected_tag)"

# Verify the release commit, workflows/assets, binary downloads, and source builds.
release-verify version previous commit remote="upstream":
    bash .github/scripts/release.sh verify "{{ version }}" "{{ previous }}" "{{ commit }}" "{{ remote }}"

# Download matching QA artifacts and compare all generated images.
release-verify-images previous commit report="/tmp/rust-skia-release-images" workflow="linux-qa.yaml" artifact="skia-org-images-x86_64-unknown-linux-gnu" website_fallback="false":
    bash .github/skills/rust-skia-release-verification/scripts/compare-images.sh "{{ previous }}" "{{ commit }}" "{{ report }}" "{{ workflow }}" "{{ artifact }}" "{{ website_fallback }}"

# Publish required crates in dependency order, verify them, and run smoke tests.
release-publish-crates version previous:
    bash .github/scripts/release.sh publish-crates "{{ version }}" "{{ previous }}"

# Create or resume the GitHub release and verify its tag.
release-publish-github version commit notes prerelease="false" remote="upstream":
    bash .github/scripts/release.sh publish-github "{{ version }}" "{{ commit }}" "{{ notes }}" "{{ prerelease }}" "{{ remote }}"
