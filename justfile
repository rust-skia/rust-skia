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

