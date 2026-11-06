//! Renders with Graphite into a texture of a `wgpu` device, and reads the pixels back with `wgpu`
//! itself. Skia's Dawn backend runs on the `wgpu` device through `skia-wgpu` (ADR 0003).

#![cfg(feature = "_wgpu")]

use std::pin::pin;
use std::task::{Context, Poll, Waker};
use std::thread;

use skia_safe::gpu::graphite;
use skia_safe::gradient;
use skia_safe::images;
use skia_safe::wgpu;
use skia_safe::{
    AlphaType, Canvas, Color, Color4f, ColorType, Data, Font, FontMgr, FontStyle, Image, ImageInfo,
    Paint, Rect, TileMode,
};

const SIZE: u32 = 64;

/// Draws content for which Skia generates different shaders, and an image that
/// Skia uploads through a mapped buffer.
#[test]
fn skia_renders_antialiasing_gradients_text_and_images() {
    let Some(pixels) = render(DeviceFeatures::Default, |canvas, recorder, _| {
        canvas.clear(Color::WHITE);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        let colors = [Color4f::from(Color::RED), Color4f::from(Color::GREEN)];
        let gradient = gradient::Gradient::new(
            gradient::Colors::new_evenly_spaced(&colors, TileMode::Clamp, None),
            gradient::Interpolation::default(),
        );
        paint.set_shader(gradient::shaders::linear_gradient(
            ((0.0, 0.0), (SIZE as f32, 0.0)),
            &gradient,
            None,
        ));
        canvas.draw_rect(Rect::from_xywh(0.0, 0.0, SIZE as f32, 16.0), &paint);

        let mut paint = Paint::new(Color4f::from(Color::BLACK), None);
        paint.set_anti_alias(true);
        canvas.draw_circle((16.0, 32.0), 10.0, &paint);
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_stroke_width(2.0);
        canvas.draw_round_rect(Rect::from_xywh(34.0, 22.0, 24.0, 20.0), 4.0, 4.0, &paint);

        let font = Font::new(
            FontMgr::new()
                .legacy_make_typeface(None, FontStyle::default())
                .unwrap(),
            12.0,
        );
        canvas.draw_str(
            "Skia",
            (4.0, 60.0),
            &font,
            &Paint::new(Color4f::from(Color::BLACK), None),
        );

        // Graphite draws only images that live on the GPU, so the raster image is uploaded first.
        let image = graphite::images::texture_from_image(recorder, &checkerboard())
            .expect("uploading the image failed");
        canvas.draw_image(&image, (48.0, 48.0), None);
    }) else {
        return;
    };
    // The gradient runs from red on the left to green on the right.
    let left = pixel(&pixels, 1, 8);
    let right = pixel(&pixels, SIZE - 2, 8);
    assert!(
        left[0] > 200 && left[1] < 60,
        "the gradient does not start red: {left:?}"
    );
    assert!(
        right[1] > 100 && right[0] < 60,
        "the gradient does not end green: {right:?}"
    );
    assert_eq!(
        pixel(&pixels, 16, 32),
        [0, 0, 0, 255],
        "the circle is not black"
    );
    // The checkerboard's top left square is black, the one next to it white.
    assert_eq!(
        pixel(&pixels, 49, 49),
        [0, 0, 0, 255],
        "the image is not drawn"
    );
    assert_eq!(
        pixel(&pixels, 53, 49),
        [255, 255, 255, 255],
        "the image is not drawn"
    );
}

/// Draws on a device with every feature and limit of the adapter, as applications request it.
/// The circle and the rect need different pipelines, so the test also covers Skia's immediates
/// across a pipeline switch (`skia-wgpu/src/v30/README.md`).
#[test]
fn skia_renders_on_a_device_with_every_adapter_feature() {
    let Some(pixels) = render(DeviceFeatures::EveryAdapterFeature, |canvas, _, _| {
        canvas.clear(Color::WHITE);
        let mut paint = Paint::new(Color4f::from(Color::BLUE), None);
        paint.set_anti_alias(true);
        canvas.draw_circle((16.0, 16.0), 10.0, &paint);

        // A gradient of a single color, so that its pixels are exact.
        let colors = [Color4f::from(Color::RED), Color4f::from(Color::RED)];
        let gradient = gradient::Gradient::new(
            gradient::Colors::new_evenly_spaced(&colors, TileMode::Clamp, None),
            gradient::Interpolation::default(),
        );
        let mut paint = Paint::default();
        paint.set_shader(gradient::shaders::linear_gradient(
            ((0.0, 0.0), (SIZE as f32, 0.0)),
            &gradient,
            None,
        ));
        canvas.draw_rect(Rect::from_xywh(36.0, 36.0, 20.0, 20.0), &paint);
    }) else {
        return;
    };
    assert_eq!(
        pixel(&pixels, 16, 16),
        [0, 0, 255, 255],
        "the circle is not blue"
    );
    assert_eq!(
        pixel(&pixels, 46, 46),
        [255, 0, 0, 255],
        "the rect is not red"
    );
    assert_eq!(
        pixel(&pixels, 2, 2),
        [255, 255, 255, 255],
        "the background is not white"
    );
}

/// Draws a texture of the application as an image, after the application dropped its texture.
#[test]
fn skia_draws_a_wgpu_texture_as_an_image() {
    let Some(pixels) = render(
        DeviceFeatures::Default,
        |canvas, recorder, (device, queue)| {
            canvas.clear(Color::WHITE);
            let texture = solid_texture(device, queue, [0, 0, 255, 255]);
            let image = graphite::wgpu::images::wrap_texture(
                recorder,
                &texture,
                ColorType::RGBA8888,
                AlphaType::Premul,
                None,
            )
            .expect("no image for the wgpu texture");
            drop(texture);
            canvas.draw_image_rect(
                &image,
                None,
                Rect::from_xywh(16.0, 16.0, 32.0, 32.0),
                &Paint::default(),
            );
        },
    ) else {
        return;
    };
    assert_eq!(
        pixel(&pixels, 32, 32),
        [0, 0, 255, 255],
        "the texture is not drawn"
    );
    assert_eq!(
        pixel(&pixels, 4, 4),
        [255, 255, 255, 255],
        "the texture is drawn outside of its rectangle"
    );
}

/// A 4 x 4 texture of one color, which Skia can sample.
fn solid_texture(device: &wgpu::Device, queue: &wgpu::Queue, pixel: [u8; 4]) -> wgpu::Texture {
    let size = wgpu::Extent3d {
        width: 4,
        height: 4,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("solid texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        texture.as_image_copy(),
        &pixel.repeat(16),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(16),
            rows_per_image: None,
        },
        size,
    );
    texture
}

/// Which features and limits `render` requests for its device.
enum DeviceFeatures {
    /// The defaults of `wgpu::DeviceDescriptor`.
    Default,
    /// Every feature and limit of the adapter, except the experimental features.
    EveryAdapterFeature,
}

/// Draws with Graphite into a texture of a new `wgpu` device and returns its RGBA pixels.
///
/// Returns `None` and prints a note on a machine without a `wgpu` adapter, such as a CI runner
/// without a GPU. With `EveryAdapterFeature`, it does the same when the adapter lacks
/// `IMMEDIATES`, because Skia then never uses immediates. The test then passes without checking
/// anything.
fn render(
    features: DeviceFeatures,
    draw: impl FnOnce(&Canvas, &mut graphite::Recorder, (&wgpu::Device, &wgpu::Queue)),
) -> Option<Vec<u8>> {
    let instance = wgpu::Instance::default();
    let Ok(adapter) = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
    else {
        eprintln!("skipped: no wgpu adapter available");
        return None;
    };
    let descriptor = match features {
        DeviceFeatures::Default => wgpu::DeviceDescriptor::default(),
        DeviceFeatures::EveryAdapterFeature => {
            if !adapter.features().contains(wgpu::Features::IMMEDIATES) {
                eprintln!("skipped: the wgpu adapter lacks IMMEDIATES");
                return None;
            }
            wgpu::DeviceDescriptor {
                required_features: adapter.features() - wgpu::Features::all_experimental_mask(),
                required_limits: adapter.limits(),
                ..Default::default()
            }
        }
    };
    let (device, queue) = block_on(adapter.request_device(&descriptor)).expect("no wgpu device");
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("skia target"),
        size: wgpu::Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let mut context = graphite::wgpu::context_factory::make_wgpu(&device, &queue, None)
        .expect("no Graphite context for the wgpu device");
    let mut recorder = context.make_recorder(None).expect("no recorder");
    let mut surface = graphite::wgpu::surfaces::wrap_texture(
        &mut recorder,
        &texture,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("no surface for the wgpu texture");
    draw(surface.canvas(), &mut recorder, (&device, &queue));
    drop(surface);

    let mut recording = recorder.snap().expect("no recording");
    context.insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
    assert!(context.submit_and_wait(), "submitting to Graphite failed");

    Some(read_texture(&device, &queue, &texture))
}

fn pixel(pixels: &[u8], x: u32, y: u32) -> [u8; 4] {
    let i = ((y * SIZE + x) * 4) as usize;
    [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
}

/// A 16 x 16 raster image of 4 x 4 black and white squares.
fn checkerboard() -> Image {
    let mut pixels = Vec::new();
    for y in 0..16 {
        for x in 0..16 {
            let black = (x / 4 + y / 4) % 2 == 0;
            let value = if black { 0 } else { 255 };
            pixels.extend_from_slice(&[value, value, value, 255]);
        }
    }
    let info = ImageInfo::new((16, 16), ColorType::RGBA8888, AlphaType::Premul, None);
    images::raster_from_data(&info, Data::new_copy(&pixels), 16 * 4).unwrap()
}

/// Copies the texture into a buffer and maps it, with `wgpu` alone.
fn read_texture(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture) -> Vec<u8> {
    // `SIZE * 4` is a multiple of the 256 bytes that a texture copy requires per row.
    let bytes_per_row = SIZE * 4;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: u64::from(bytes_per_row * SIZE),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: None,
            },
        },
        texture.size(),
    );
    queue.submit([encoder.finish()]);
    buffer.map_async(wgpu::MapMode::Read, .., |result| {
        result.expect("mapping the readback buffer failed")
    });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        })
        .expect("polling the device failed");
    buffer
        .get_mapped_range(..)
        .expect("reading the mapped buffer failed")
        .to_vec()
}

/// `wgpu`'s native backends resolve adapter and device requests without polling.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        thread::yield_now();
    }
}
