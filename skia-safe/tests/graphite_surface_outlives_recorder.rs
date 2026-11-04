//! A Graphite surface can outlive the recorder it was created from, because
//! `graphite::surfaces::render_target` ties no lifetime to the recorder. Skia's draws dereference
//! the recorder, so dropping it while a surface still uses it must panic instead of leaving the
//! surface with a deleted recorder.

#![cfg(all(
    feature = "vulkan",
    feature = "graphite",
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

mod common;

use std::panic::{self, AssertUnwindSafe};

use common::vulkan::Vulkan;
use skia_safe::gpu::{self, graphite};
use skia_safe::{AlphaType, Color, Color4f, ColorType, ImageInfo, Paint, Rect};

#[test]
fn dropping_a_recorder_panics_while_one_of_its_surfaces_lives() {
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
    let mut surface = graphite::surfaces::render_target(
        &mut recorder,
        &image_info,
        gpu::Mipmapped::No,
        None,
        None,
    )
    .unwrap();

    let dropped = panic::catch_unwind(AssertUnwindSafe(|| drop(recorder)));
    assert!(dropped.is_err(), "dropping the recorder did not panic");

    // The panic leaves the recorder alive, so the surface can still draw.
    surface.canvas().draw_rect(
        Rect::from_xywh(8.0, 8.0, 16.0, 16.0),
        &Paint::new(Color4f::from(Color::RED), None),
    );
}
