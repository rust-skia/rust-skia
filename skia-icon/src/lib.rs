//! Renders the animated version of the rust-skia icon.
//!
//! Shared by the `icon`, `gl-window`, and `wasm-example` examples. Not
//! published to crates.io (`publish = false`); consumed via path/dev
//! dependencies only.

mod renderer;

pub use renderer::render_frame;
