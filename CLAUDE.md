# CLAUDE.md — rust-skia development notes

## Repository structure

- `skia-bindings/` — Low-level C++ bindings. Contains:
  - `skia/` — Git submodule pointing to `rust-skia/skia` fork (tagged e.g. `m148-0.95.1`)
  - `src/bindings.cpp` — C wrapper functions for Skia C++ APIs (parsed by bindgen)
  - `src/shaper.cpp` — C wrappers for SkShaper / modules
  - `Cargo.toml` — `[package.metadata] skia = "m148-X.Y.Z"` must match submodule tag
  - `build_support/` — Build configuration (e.g. `binaries_config.rs` for platform-specific logic)
- `skia-safe/` — Safe Rust wrappers over `skia-bindings`. Mirrors Skia's include structure:
  - `src/core/` → `include/core/`
  - `src/gpu/ganesh/` → `include/gpu/ganesh/`
  - `src/modules/shaper/` → `modules/skshaper/include/`
  - `src/modules/paragraph/` → `modules/skparagraph/include/`
  - etc.

## Binding generation

- Bindgen parses the `.cpp` files in `skia-bindings/src/` to discover `extern "C"` functions and types.
- Touch `skia-bindings/src/bindings.cpp` and run `cargo check -p skia-bindings` to force regeneration.
- Generated bindings land in `target/*/build/skia-bindings-*/out/skia/bindings.rs`.
- C++ `enum class` variants get their `k` prefix stripped automatically by bindgen.
- C++ namespaced types like `SkShapers::CT::LineBreakMode` become `SkShapers_CT_LineBreakMode`.

## Common patterns in skia-safe

- **Enum wrapping:** `pub type Foo = skia_bindings::SkFoo;` + `variant_name!(Foo::SomeVariant);`
- **Struct wrapping:** `pub type Foo = Handle<SkFoo>;` or `pub type Foo = RCHandle<SkFoo>;`
- **native_transmutable!** for types that are bit-for-bit compatible.
- **require_type_equality!** to verify base class assumptions at compile time (e.g. `SkPixelRef_INHERITED` == `SkRefCnt`).
- **variant_name!** to verify a variant exists at compile time (catches bindgen name changes).
- Platform-gated code uses `#[cfg(any(target_os = "macos", target_os = "ios"))]`.
- Functions returning `Option<Self>` via `Self::from_ptr(unsafe { ... })` — returns `None` if the C function returns null (used for platform-optional features like CoreText).

## Skia milestone update checklist

See the [Template: Skia Milestone Update PR](https://github.com/rust-skia/rust-skia/wiki/Template:-Skia-Milestone-Update-PR) wiki page.

Version numbering: Each milestone bump increments the minor version (e.g. 0.95.0 → 0.96.0).

Key diffs to check between milestones (in `skia-bindings/skia/`):
```
git diff OLD_TAG..NEW_TAG -- include/core include/codec include/docs include/effects \
  include/encode include/gpu include/pathops include/svg include/utils include/private/base \
  modules/skparagraph/include modules/skshaper/include modules/svg/include modules/skresources/include
```

Build organization diffs:
```
git diff OLD_TAG..NEW_TAG -- BUILD.gn gn/ modules/skshaper/BUILD.gn modules/skshaper/skshaper.gni \
  modules/paragraph/BUILD.gn modules/paragraph/skparagraph.gni modules/svg/BUILD.gn modules/svg/svg.gni
```

## Release notes

For rust-skia release notes format and authoring rules, use:

- `.github/release-notes-guidelines.md`
