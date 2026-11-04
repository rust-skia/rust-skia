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
use skia_safe::{
    AlphaType, Color, Color4f, ColorType, ImageInfo, Paint, Rect, canvas::SaveLayerRec,
    image_filters,
};

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

    // The panic leaves the recorder alive, so the surface can still draw.
    surface.canvas().draw_rect(
        Rect::from_xywh(8.0, 8.0, 16.0, 16.0),
        &Paint::new(Color4f::from(Color::RED), None),
    );
}

/// A layer with an image filter leaves devices that only the recorder still holds until the next
/// snap. The drop check relies on `snap()` removing them, and must not count them as live.
#[test]
fn dropping_a_recorder_after_its_surface_ignores_leftover_layer_devices() {
    let Some(mut graphite) = Graphite::new() else {
        eprintln!("skipped: no Graphite context available");
        return;
    };
    let mut recorder = graphite.context.make_recorder(None).unwrap();
    {
        let mut surface = render_target(&mut recorder);
        let canvas = surface.canvas();
        let mut blur = Paint::default();
        blur.set_image_filter(image_filters::blur((4.0, 4.0), None, None, None));
        canvas.save_layer(&SaveLayerRec::default().paint(&blur));
        canvas.draw_rect(Rect::from_xywh(8.0, 8.0, 16.0, 16.0), &Paint::default());
        canvas.restore();
    }

    drop(recorder);
}

fn render_target(recorder: &mut graphite::Recorder) -> skia_safe::Surface {
    let image_info = ImageInfo::new((32, 32), ColorType::RGBA8888, AlphaType::Premul, None);
    graphite::surfaces::render_target(recorder, &image_info, gpu::Mipmapped::No, None, None)
        .unwrap()
}
