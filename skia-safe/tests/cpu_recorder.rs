//! Tests for the CPU recorder obtained from a Ganesh recording context.
//!
//! The recorder is created from a real (Metal-backed) `DirectContext`, mirroring what
//! `tests/CPUContextRecorderTest.cpp` does upstream.
#![cfg(all(target_os = "macos", feature = "ganesh", feature = "metal"))]

use objc2::rc::Retained;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

use skia_safe::{
    Bitmap, ColorSpace,
    gpu::{direct_contexts, mtl},
    image::RequiredProperties,
    images::raster_from_bitmap,
    recorder::{Recorder, Type as RecorderType},
};

#[test]
fn cpu_recorder_from_recording_context() {
    let device = MTLCreateSystemDefaultDevice().expect("no system default Metal device");
    let queue = device
        .newCommandQueue()
        .expect("failed to create MTLCommandQueue");
    let backend = unsafe {
        mtl::BackendContext::new(
            Retained::as_ptr(&device) as mtl::Handle,
            Retained::as_ptr(&queue) as mtl::Handle,
        )
    };
    let mut context =
        direct_contexts::make_metal(&backend, None).expect("make_metal returned None");

    let mut recorder = context.make_cpu_recorder();
    assert_eq!(recorder.ty(), RecorderType::CPU);

    let mut bitmap = Bitmap::new();
    assert!(bitmap.try_alloc_n32_pixels((64, 64), true));
    let image = raster_from_bitmap(&bitmap).expect("raster_from_bitmap returned None");

    assert!(image.is_valid(Some(&mut recorder)));

    let converted = image
        .make_color_space(
            Some(&mut recorder),
            ColorSpace::new_srgb_linear(),
            RequiredProperties::default(),
        )
        .expect("make_color_space returned None");
    assert_eq!(converted.width(), 64);
    assert!(!converted.is_texture_backed());
}
