# AGENTS.md - rust-skia development notes

## Repository structure

- `skia-bindings/` - Low-level C++ bindings. Contains:
  - `skia/` - Git submodule pointing to `rust-skia/skia` fork; the tag follows the scheme
    `m<milestone>.<ordinal>` (see `docs/adr/0002-skia-fork-tags-are-milestone-plus-ordinal.md`)
  - `src/*.cpp` - C wrapper functions for Skia C++ APIs (parsed by bindgen). One file per area, for example `bindings.cpp` (core), `ganesh.cpp`, `graphite.cpp`, `shaper.cpp`, `skottie.cpp`, `svg.cpp`, `vulkan.cpp`.
  - `Cargo.toml` - `[package.metadata] skia = "m<milestone>.<ordinal>"` must match submodule tag
  - `build_support/` - Build configuration (e.g. `binaries_config.rs` for platform-specific logic)
- `skia-safe/` - Safe Rust wrappers over `skia-bindings`. Mirrors Skia's include structure:
  - `src/core/` -> `include/core/`
  - `src/gpu/ganesh/` -> `include/gpu/ganesh/`
  - `src/modules/shaper/` -> `modules/skshaper/include/`
  - `src/modules/paragraph/` -> `modules/skparagraph/include/`
  - etc.
- `skia-wgpu/` - Runs Graphite's Dawn backend on the `wgpu` crate: the WebGPU C functions that the
  backend calls, implemented on `wgpu` (the shim), one module per supported `wgpu` major version
  behind the feature `wgpu-NN` (see `docs/adr/0003-skia-runs-on-wgpu-through-a-webgpu-shim.md`).
  `skia-safe` uses it behind the same features.
- `skia-org/` - Ports of the fiddles on skia.org, rendered per driver. `skia-org/README.md` maps
  each skia.org page to its module and records how to fetch and port a fiddle.
- `mk-workflows/` - Generator for the platform CI workflows in `.github/workflows/`; edit it, then
  run `just workflows`. The macOS deployment target lives here.
- CI Linux toolchain: `ghcr.io/pragmatrix/rust-skia-linux` is built from
  `pragmatrix/rust-skia-images` (`linux/Dockerfile`, checked out at `../rust-skia-images`) and
  published by that repo's manual Docker Image CI workflow. `docker/` here is unofficial.

## Skills

Task skills live in `.agents/skills/`. `.claude/skills/` links to it for Claude Code.
Read the matching `SKILL.md` before a bindings change, a Skia fork tag cut, or a release.
