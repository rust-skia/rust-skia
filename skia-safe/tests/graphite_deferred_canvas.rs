//! `Recorder::make_deferred_canvas` returns `None` when Graphite cannot create the canvas's device.
//! Skia's `makeDeferredCanvas` did not check that result and built a canvas on a null device.

#![cfg(all(
    feature = "vulkan",
    feature = "graphite",
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

mod common;

use common::vulkan::Vulkan;
use skia_safe::gpu::graphite;
use skia_safe::{AlphaType, ColorType, ImageInfo};

#[test]
fn make_deferred_canvas_returns_none_for_an_invalid_texture_info() {
    let Some(vulkan) = Vulkan::new() else {
        eprintln!("skipped: no Vulkan device available");
        return;
    };
    let Some(mut context) = vulkan.graphite_context() else {
        eprintln!("skipped: no Vulkan context available");
        return;
    };
    let mut recorder = context.make_recorder(None).unwrap();
    let image_info = ImageInfo::new((32, 32), ColorType::RGBA8888, AlphaType::Premul, None);

    assert!(
        recorder
            .make_deferred_canvas(&image_info, &graphite::TextureInfo::new())
            .is_none()
    );
}
