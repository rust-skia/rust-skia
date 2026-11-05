//! Renders with Graphite on a `wgpu` device. Skia's Dawn backend runs on `wgpu` through
//! `skia-wgpu` (ADR 0003).
//!
//! Graphite cannot encode an image that lives on the GPU, so the driver copies each rendered
//! texture into a buffer with `wgpu`, wraps the pixels in a raster image, and encodes that.

use std::path::Path;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use skia_safe::gpu::graphite;
use skia_safe::image::RequiredProperties;
use skia_safe::wgpu;
use skia_safe::{AlphaType, Canvas, ColorType, Data, EncodedImageFormat, Image, ImageInfo, images};

use crate::Driver;
use crate::artifact;
use crate::drivers::DrawingDriver;

/// The texture format and the Skia color type that describes it.
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const COLOR_TYPE: ColorType = ColorType::RGBA8888;
const BYTES_PER_PIXEL: u32 = 4;

pub struct Wgpu {
    // Ordered for drop order: the recorder and the context go before the device.
    recorder: graphite::Recorder,
    context: graphite::Context,
    queue: wgpu::Queue,
    device: wgpu::Device,
}

impl DrawingDriver for Wgpu {
    const DRIVER: Driver = Driver::Wgpu;

    fn new() -> Self {
        let instance = wgpu::Instance::default();
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
            .expect("no wgpu adapter");
        println!("wgpu adapter: {:?}", adapter.get_info());
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
            .expect("no wgpu device");
        let mut context = graphite::wgpu::context_factory::make_wgpu(&device, &queue, None)
            .expect("no Graphite context for the wgpu device");
        let mut options = graphite::RecorderOptions::new();
        options.set_image_provider(UploadingImageProvider);
        let recorder = context
            .make_recorder(Some(&options))
            .expect("no Graphite recorder");

        Self {
            recorder,
            context,
            queue,
            device,
        }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&Canvas),
    ) {
        // The examples render at twice their size, like those of the other drivers.
        let size = wgpu::Extent3d {
            width: (width * 2) as u32,
            height: (height * 2) as u32,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let mut surface = graphite::wgpu::surfaces::wrap_texture(
            &mut self.recorder,
            &texture,
            COLOR_TYPE,
            None,
            None,
        )
        .expect("no surface for the wgpu texture");
        let canvas = surface.canvas();
        canvas.scale((2.0, 2.0));
        func(canvas);
        drop(surface);

        let mut recording = self.recorder.snap().expect("no Graphite recording");
        self.context
            .insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
        assert!(
            self.context.submit_and_wait(),
            "submitting to Graphite failed"
        );

        let (pixels, bytes_per_row) = read_texture(&self.device, &self.queue, &texture);
        let info = ImageInfo::new(
            (size.width as i32, size.height as i32),
            COLOR_TYPE,
            AlphaType::Premul,
            None,
        );
        let image =
            images::raster_from_data(&info, Data::new_copy(&pixels), bytes_per_row as usize)
                .expect("no raster image for the texture's pixels");
        let data = image
            .encode(None, EncodedImageFormat::PNG, None)
            .expect("encoding the image failed");
        artifact::write_file(data.as_bytes(), path, name, "png");
    }
}

/// Uploads every raster image that an example draws, because Graphite drops the draw of an image
/// that does not live on the GPU. The examples draw few, small images, so the provider uploads
/// again on every draw instead of caching the textures.
struct UploadingImageProvider;

impl graphite::ImageProvider for UploadingImageProvider {
    fn find_or_create(
        &self,
        recorder: &graphite::BorrowedRecorder,
        image: &Image,
        required: RequiredProperties,
    ) -> Option<Image> {
        recorder.texture_from_image(image, required)
    }
}

/// Copies the texture into a buffer and maps it. Returns the pixels and the bytes per row, which
/// `wgpu` pads to a multiple of [`wgpu::COPY_BYTES_PER_ROW_ALIGNMENT`].
fn read_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
) -> (Vec<u8>, u32) {
    let size = texture.size();
    let bytes_per_row =
        (size.width * BYTES_PER_PIXEL).next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: u64::from(bytes_per_row * size.height),
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
        size,
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
    let pixels = buffer
        .get_mapped_range(..)
        .expect("reading the mapped buffer failed")
        .to_vec();
    (pixels, bytes_per_row)
}

/// Resolves a `wgpu` future that is ready at once. `wgpu`'s native backends resolve adapter and
/// device requests when they are made, so nothing would ever wake a pending one.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut context = Context::from_waker(Waker::noop());
    match pin!(future).poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("a wgpu request that the driver waits on is not ready"),
    }
}
