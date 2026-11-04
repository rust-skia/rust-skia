//! A `wgpu` application that renders a rotating cube, whose faces show a texture that Skia draws
//! into every frame: the animated rust-skia logo of the `skia-icon` crate.
//!
//! Skia renders on the application's `wgpu` device through `skia_safe::gpu::graphite::wgpu`
//! (ADR 0003). Skia and the application submit to the same `wgpu` queue, so the cube pass sees the
//! texture that Skia finished drawing earlier in the frame without further synchronization.
//!
//! Run with `cargo run -p skia-safe --features wgpu --example wgpu-window`.

#[cfg(feature = "_wgpu")]
mod app;

#[cfg(feature = "_wgpu")]
fn main() {
    app::run();
}

#[cfg(not(feature = "_wgpu"))]
fn main() {
    println!("To run this example, invoke cargo with --features \"wgpu\".")
}
