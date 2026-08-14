//! Headless offscreen Graphite/Metal render — FFI soundness verification.
//!
//! Exercises the full Graphite Context/Recorder/Recording lifecycle on an
//! offscreen render target (no window / no drawable), then drops every object.
//!
//! The drop path distinguishes a correct `unique_ptr`-owned binding
//! (`RefHandle` -> C++ `delete` -> real `~Context()/~Recorder()`) from the buggy
//! ref-counted model (`RCHandle` -> `SkRefCntBase::unref()` on a non-ref-counted
//! object, whose C++ destructor never runs -> leak).
//!
//! With an iteration count > 1 it loops the lifecycle so the leak shows up as
//! growing RSS. Measure peak RSS with:
//!   /usr/bin/time -l cargo run ... --example graphite_offscreen --features metal,graphite -- 500
//!
//! Run: `cargo run --example graphite_offscreen --features metal,graphite -- [iters]`

#[cfg(not(all(target_os = "macos", feature = "metal", feature = "graphite")))]
fn main() {
    eprintln!("graphite_offscreen requires: macOS + --features metal,graphite");
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

    let iters: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    // `reuse` => create ONE Context/Recorder outside the loop (isolates whether
    // the residual per-iter growth is tied to Context creation or to the
    // surface/render path).
    let reuse = std::env::args().nth(2).as_deref() == Some("reuse");

    // Metal device + queue created once and shared across iterations.
    let device = MTLCreateSystemDefaultDevice().expect("no system default Metal device");
    let queue = device
        .newCommandQueue()
        .expect("failed to create MTLCommandQueue");
    let device_ptr = Retained::as_ptr(&device) as *mut c_void;
    let queue_ptr = Retained::as_ptr(&queue) as *mut c_void;

    let verbose = iters == 1;

    // In reuse mode, one Context + Recorder live for the whole run.
    let mut shared = if reuse {
        let backend = unsafe { gmtl::BackendContext::new(device_ptr, queue_ptr) };
        let mut context = gmtl::make_context(&backend, None).expect("make_context returned None");
        let recorder = context
            .make_recorder(None)
            .expect("make_recorder returned None");
        Some((backend, context, recorder))
    } else {
        None
    };

    for i in 0..iters {
        // Per-iteration Context + Recorder (default mode). A leaked Context/
        // Recorder (missing C++ destructor) accumulates here.
        let mut fresh = if reuse {
            None
        } else {
            let backend = unsafe { gmtl::BackendContext::new(device_ptr, queue_ptr) };
            let mut context =
                gmtl::make_context(&backend, None).expect("make_context returned None");
            let recorder = context
                .make_recorder(None)
                .expect("make_recorder returned None");
            Some((backend, context, recorder))
        };

        let (context, recorder) = match (&mut shared, &mut fresh) {
            (Some((_, c, r)), _) | (_, Some((_, c, r))) => (c, r),
            _ => unreachable!(),
        };

        let image_info = ImageInfo::new((64, 64), ColorType::RGBA8888, AlphaType::Premul, None);
        let mut surface = graphite::surfaces::render_target(
            recorder,
            &image_info,
            graphite::Mipmapped::No,
            None,
            Some("graphite_offscreen_verify"),
        )
        .expect("render_target returned None");

        let canvas = surface.canvas();
        canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
        canvas.draw_rect(
            Rect::from_xywh(8.0, 8.0, 48.0, 48.0),
            &Paint::new(Color4f::new(0.0, 0.0, 1.0, 1.0), None),
        );

        let mut recording = recorder.snap().expect("recorder.snap() returned None");
        let status = context.insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
        let submitted = context.submit_and_wait();
        if verbose {
            println!(
                "[ok] context+recorder+offscreen ok; insert={status:?} submit_and_wait={submitted}"
            );
        }

        // Drop surface + recording each iter; `fresh` (if any) drops its
        // Context + Recorder here too via the unique_ptr destruction path.
        drop(recording);
        drop(surface);
        drop(fresh);

        if !verbose && (i + 1) % 100 == 0 {
            println!("iter {}/{}", i + 1, iters);
        }
    }

    drop(shared);
    println!("[PASS] {iters} iteration(s) (reuse={reuse}) completed without crash");
}
