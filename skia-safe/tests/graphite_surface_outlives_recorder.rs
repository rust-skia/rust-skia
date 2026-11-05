//! A Graphite surface can outlive the recorder it was created from, because
//! `graphite::surfaces::render_target` ties no lifetime to the recorder. Skia's draws dereference
//! the recorder, so dropping it while a surface still uses it must panic instead of leaving the
//! surface with a deleted recorder.

#![cfg(all(
    feature = "graphite",
    any(all(target_os = "macos", feature = "metal"), feature = "vulkan"),
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

mod common;

use std::panic::{self, AssertUnwindSafe};

use common::graphite::Graphite;
use skia_safe::gpu::{self, graphite};
use skia_safe::{AlphaType, Color, Color4f, ColorType, ImageInfo, Paint, Rect};

#[test]
fn dropping_a_recorder_panics_while_one_of_its_surfaces_lives() {
    let Some(mut graphite) = Graphite::new() else {
        eprintln!("skipped: no Graphite context available");
        return;
    };
    let mut recorder = graphite.context.make_recorder(None).unwrap();
    let mut surface = render_target(&mut recorder);

    let dropped = panic::catch_unwind(AssertUnwindSafe(|| drop(recorder)));
    assert!(dropped.is_err(), "dropping the recorder did not panic");

    // The panic leaks the recorder, so a later draw on the surface doesn't crash. The draw is never
    // submitted.
    surface.canvas().draw_rect(
        Rect::from_xywh(8.0, 8.0, 16.0, 16.0),
        &Paint::new(Color4f::from(Color::RED), None),
    );

    // The leaked recorder still owns GPU objects of the device, and Vulkan does not allow
    // destroying a device that still has them, so the device is leaked too.
    drop(surface);
    std::mem::forget(graphite);
}

/// A dropped surface leaves an empty entry in the recorder's device list until the next snap. The
/// drop check relies on `snap()` removing it, and must not count it as a live surface.
#[test]
fn dropping_a_recorder_after_its_surface_succeeds() {
    let Some(mut graphite) = Graphite::new() else {
        eprintln!("skipped: no Graphite context available");
        return;
    };
    let mut recorder = graphite.context.make_recorder(None).unwrap();
    drop(render_target(&mut recorder));

    drop(recorder);
}

fn render_target(recorder: &mut graphite::Recorder) -> skia_safe::Surface {
    let image_info = ImageInfo::new((32, 32), ColorType::RGBA8888, AlphaType::Premul, None);
    graphite::surfaces::render_target(recorder, &image_info, gpu::Mipmapped::No, None, None)
        .unwrap()
}
