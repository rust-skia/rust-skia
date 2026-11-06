# skia-wgpu

Runs [Skia](https://skia.org/)'s Graphite Dawn backend on the [`wgpu`](https://wgpu.rs) crate, so
that Skia renders on the device, queue, and textures of a `wgpu` application.

Skia's Dawn backend calls the WebGPU C functions of `webgpu.h`. This crate implements the functions
that the backend calls on `wgpu` and exports them under their C names, so neither Dawn nor any other
WebGPU implementation is linked.

Applications do not use this crate directly. They enable the `wgpu` feature of
[skia-safe](https://crates.io/crates/skia-safe), or the `wgpu-NN` feature for the `wgpu` major
version they use, and render through `skia_safe::gpu::graphite::wgpu`.

Each supported `wgpu` major version is a feature `wgpu-NN`, currently `wgpu-30`. A build enables
exactly one of them, and only one version of this crate can be part of a dependency graph.

The design is described in the rust-skia repository, in
[ADR 0003](https://github.com/rust-skia/rust-skia/blob/master/docs/adr/0003-skia-runs-on-wgpu-through-a-webgpu-shim.md)
and
[ADR 0004](https://github.com/rust-skia/rust-skia/blob/master/docs/adr/0004-the-shim-exports-the-webgpu-functions-from-rust.md).
[FEATURES.md](https://github.com/rust-skia/rust-skia/blob/master/skia-wgpu/FEATURES.md) lists
which WebGPU features the shim reports to Skia. The notes of each `wgpu` version, such as
[src/v30/README.md](https://github.com/rust-skia/rust-skia/blob/master/skia-wgpu/src/v30/README.md),
list where that version deviates from WebGPU and how the shim makes up for it.

## License

MIT
