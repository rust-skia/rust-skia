//! Draws the animated rust-skia icon with Graphite on the browser's WebGPU, from a
//! `wasm32-unknown-unknown` module that wasm-bindgen processes.

use skia_safe::gpu::graphite;
use skia_safe::{Color, ColorType, wgpu};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

/// The frame rate and tempo of the logo animation, as in the `gl-window` example of `skia-safe`.
const LOGO_FPS: usize = 180;
const LOGO_BPM: usize = 60;
/// The number of frames after which the animation repeats seamlessly: one revolution of the
/// chain ring, 360 / (12 · 60/60/180).
const LOGO_FRAMES: usize = 5400;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("no window")?;
    let canvas: web_sys::HtmlCanvasElement = window
        .document()
        .and_then(|document| document.get_element_by_id("canvas"))
        .ok_or("no element with the id `canvas`")?
        .dyn_into()?;
    let (width, height) = (canvas.width(), canvas.height());

    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        .map_err(to_js)?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(to_js)?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(to_js)?;
    let config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("the canvas does not support the adapter")?;
    surface.configure(&device, &config);
    let color_type = match config.format {
        wgpu::TextureFormat::Bgra8Unorm => ColorType::BGRA8888,
        _ => ColorType::RGBA8888,
    };

    let mut context = graphite::wgpu::context_factory::make_wgpu(&device, &queue, None)
        .ok_or("no Graphite context for the wgpu device")?;
    let mut recorder = context.make_recorder(None).ok_or("no recorder")?;

    let performance = window.performance().ok_or("no performance clock")?;
    loop {
        next_animation_frame(&window).await?;
        // A page can't wait for the GPU, so the context only learns here which work has finished.
        context.check_async_work_completion();

        let frame = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            _ => continue,
        };
        let mut skia_surface = graphite::wgpu::surfaces::wrap_texture(
            &mut recorder,
            &frame.texture,
            color_type,
            None,
            None,
        )
        .ok_or("no Skia surface for the canvas texture")?;

        let canvas = skia_surface.canvas();
        canvas.clear(Color::WHITE);
        let seconds = performance.now() / 1000.0;
        let logo_frame = (seconds * LOGO_FPS as f64) as usize % LOGO_FRAMES;
        skia_icon::render_frame(logo_frame, LOGO_FPS, LOGO_BPM, canvas);

        let mut recording = recorder.snap().ok_or("no recording")?;
        context.insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
        context.submit(None);
        drop(skia_surface);
        queue.present(frame);
    }
}

/// Resolves at the browser's next animation frame.
async fn next_animation_frame(window: &web_sys::Window) -> Result<(), JsValue> {
    let mut request = Ok(0);
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        request = window.request_animation_frame(&resolve);
    });
    request?;
    wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(())
}

fn to_js(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}
