//! Headless offscreen Graphite/Metal render with pixel readback verification.
//!
//! Unlike `graphite_offscreen` (which only proves the lifecycle doesn't crash or
//! leak), this draws a known image and reads the pixels back via
//! `Context::read_pixels` (the synchronous `asyncRescaleAndReadPixels` path) to
//! prove the Graphite pipeline produces *correct* output — the first actual
//! pixel-level verification of the binding, and the path a screenshot / golden
//! capture would use.
//!
//! Run: `cargo run --example graphite_readback --features metal,graphite`

#[cfg(not(all(target_os = "macos", feature = "metal", feature = "graphite")))]
fn main() {
    eprintln!("graphite_readback requires: macOS + --features metal,graphite");
}

#[cfg(all(target_os = "macos", feature = "metal", feature = "graphite"))]
fn main() {
    use std::ffi::c_void;

    use objc2::rc::Retained;
    use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

    use skia_safe::{
        AlphaType, Color4f, ColorType, ImageInfo, Paint, Rect,
        graphite::{self, mtl as gmtl},
    };

    const W: i32 = 64;
    const H: i32 = 64;

    let device = MTLCreateSystemDefaultDevice().expect("no system default Metal device");
    let queue = device
        .newCommandQueue()
        .expect("failed to create MTLCommandQueue");
    let device_ptr = Retained::as_ptr(&device) as *mut c_void;
    let queue_ptr = Retained::as_ptr(&queue) as *mut c_void;

    let backend = unsafe { gmtl::BackendContext::new(device_ptr, queue_ptr) };
    let mut context = gmtl::make_context(&backend, None).expect("make_context returned None");
    let mut recorder = context
        .make_recorder(None)
        .expect("make_recorder returned None");

    let image_info = ImageInfo::new((W, H), ColorType::RGBA8888, AlphaType::Premul, None);
    let mut surface = graphite::surfaces::render_target(
        &mut recorder,
        &image_info,
        graphite::Mipmapped::No,
        None,
        Some("graphite_readback"),
    )
    .expect("render_target returned None");

    // White background, opaque blue square covering (8,8)..(56,56).
    let canvas = surface.canvas();
    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
    canvas.draw_rect(
        Rect::from_xywh(8.0, 8.0, 48.0, 48.0),
        &Paint::new(Color4f::new(0.0, 0.0, 1.0, 1.0), None),
    );

    let mut recording = recorder.snap().expect("recorder.snap() returned None");
    let inserted = context.insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
    let submitted = context.submit_and_wait();
    println!("insert={inserted:?} submit_and_wait={submitted}");

    // Read the whole surface back into a CPU buffer.
    let row_bytes = (W as usize) * 4;
    let mut pixels = vec![0u8; row_bytes * (H as usize)];
    let ok = context.read_pixels(&mut surface, &image_info, &mut pixels, row_bytes, (0, 0));
    assert!(ok, "Context::read_pixels failed");

    let px = |x: i32, y: i32| -> [u8; 4] {
        let o = (y as usize) * row_bytes + (x as usize) * 4;
        [pixels[o], pixels[o + 1], pixels[o + 2], pixels[o + 3]]
    };

    // Corner (0,0) is outside the square -> white; center (32,32) -> blue.
    let corner = px(0, 0);
    let center = px(32, 32);
    println!("corner(0,0)  = {corner:?} (expect ~white 255,255,255,255)");
    println!("center(32,32)= {center:?} (expect ~blue  0,0,255,255)");

    let is_white = corner[0] > 200 && corner[1] > 200 && corner[2] > 200 && corner[3] > 200;
    let is_blue = center[2] > 200 && center[0] < 60 && center[1] < 60 && center[3] > 200;

    // Exercise the fixed surface-adopt path: `as_image` now clones (bumps) the
    // surface ref and transfers it, so the count stays balanced across many
    // calls. Under the old borrowed-pointer adopt bug each call net-decremented
    // the surface refcount, freeing it out from under us -> crash / UAF.
    for _ in 0..200 {
        assert!(
            graphite::surfaces::as_image(&surface).is_some(),
            "as_image returned None"
        );
    }
    println!("[ok] as_image x200 stable (surface refcount balanced)");

    // Drop GPU objects before exiting.
    drop(recording);
    drop(surface);

    if is_white && is_blue {
        println!("[PASS] readback verified: background white, square blue");
    } else {
        eprintln!("[FAIL] readback pixels did not match expected image");
        std::process::exit(1);
    }
}
