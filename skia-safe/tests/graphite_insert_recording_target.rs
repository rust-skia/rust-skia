//! A recording made on a deferred canvas replays into the target surface given when it is
//! inserted, offset by the target translation and limited by the target clip.
//!
//! The test creates the target texture itself, because a deferred canvas needs the exact
//! `TextureInfo` of its target, and skia-safe gets one only from a wrapped native texture.

#![cfg(all(target_os = "macos", feature = "metal", feature = "graphite"))]

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{
    MTLCreateSystemDefaultDevice, MTLDevice, MTLPixelFormat, MTLStorageMode, MTLTexture,
    MTLTextureDescriptor, MTLTextureUsage,
};
use skia_safe::gpu::graphite::{self, InsertRecordingInfo, InsertStatus, mtl};
use skia_safe::{AlphaType, Color, ColorType, IRect, ImageInfo, Paint, Rect};

const SIZE: i32 = 32;

#[test]
fn a_deferred_recording_replays_into_the_target_surface_with_translation_and_clip() {
    let Some(device) = MTLCreateSystemDefaultDevice() else {
        eprintln!("skipped: no Metal device available");
        return;
    };
    let queue = device.newCommandQueue().unwrap();
    let backend = unsafe {
        mtl::BackendContext::new(
            Retained::as_ptr(&device) as mtl::Handle,
            Retained::as_ptr(&queue) as mtl::Handle,
        )
    };
    let mut context = mtl::context_factory::make_metal(&backend, None).unwrap();

    let texture = make_texture(&device);
    let backend_texture = unsafe {
        mtl::backend_textures::make_metal((SIZE, SIZE), Retained::as_ptr(&texture) as mtl::Handle)
    };

    // The target starts out white.
    let mut target_recorder = context.make_recorder(None).unwrap();
    let mut target = graphite::surfaces::wrap_backend_texture(
        &mut target_recorder,
        &backend_texture,
        ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap();
    target.canvas().clear(Color::WHITE);
    let mut clear = target_recorder.snap().unwrap();
    assert_eq!(
        context.insert_recording(&InsertRecordingInfo::new(&mut clear)),
        InsertStatus::Success
    );

    // The deferred canvas fills its whole 16×16 area with red.
    let mut recorder = context.make_recorder(None).unwrap();
    let image_info = ImageInfo::new((16, 16), ColorType::RGBA8888, AlphaType::Premul, None);
    recorder
        .make_deferred_canvas(&image_info, &backend_texture.info())
        .unwrap()
        .draw_rect(
            Rect::from_wh(16.0, 16.0),
            &Paint::new(skia_safe::Color4f::from(Color::RED), None),
        );
    let mut recording = recorder.snap().unwrap();

    // Moved to (8, 8), the red area covers (8, 8)..(24, 24). The clip is in the deferred canvas's
    // coordinates and moves with it, so it keeps the area's left half, (8, 8)..(16, 24).
    let status = context.insert_recording(
        &InsertRecordingInfo::new(&mut recording)
            .target_surface(&mut target)
            .target_translation((8, 8))
            .target_clip(IRect::from_xywh(0, 0, 8, 16)),
    );
    assert_eq!(status, InsertStatus::Success);
    assert!(context.submit_and_wait());

    let info = ImageInfo::new((SIZE, SIZE), ColorType::RGBA8888, AlphaType::Premul, None);
    let mut pixels = vec![0u8; info.compute_min_byte_size()];
    assert!(context.read_pixels(
        &mut target,
        &info,
        &mut pixels,
        info.min_row_bytes(),
        (0, 0)
    ));
    let pixel = |x: usize, y: usize| {
        let at = (y * SIZE as usize + x) * 4;
        <[u8; 4]>::try_from(&pixels[at..at + 4]).unwrap()
    };
    const RED: [u8; 4] = [255, 0, 0, 255];
    const WHITE: [u8; 4] = [255, 255, 255, 255];

    assert_eq!(
        pixel(10, 10),
        RED,
        "inside the translated area and the clip"
    );
    assert_eq!(
        pixel(10, 22),
        RED,
        "inside the translated area and the clip"
    );
    assert_eq!(
        pixel(20, 10),
        WHITE,
        "inside the translated area, outside the clip"
    );
    assert_eq!(pixel(4, 4), WHITE, "outside the translated area");
}

fn make_texture(
    device: &ProtocolObject<dyn MTLDevice>,
) -> Retained<ProtocolObject<dyn MTLTexture>> {
    let descriptor = unsafe {
        MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            SIZE as usize,
            SIZE as usize,
            false,
        )
    };
    descriptor.setUsage(MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead);
    descriptor.setStorageMode(MTLStorageMode::Private);
    device.newTextureWithDescriptor(&descriptor).unwrap()
}
