use skia_safe::{Color, Paint, PaintStyle};
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "gl")]
use web_sys::WebGl2RenderingContext;
#[cfg(not(feature = "gl"))]
use web_sys::{CanvasRenderingContext2d, ImageData};

#[cfg(feature = "gl")]
use skia_safe::{
    gpu::{self, gl::FramebufferInfo, DirectContext},
    ColorType, Surface,
};
#[cfg(not(feature = "gl"))]
use skia_safe::{AlphaType, ColorType, ImageInfo};

#[cfg(feature = "gl")]
struct GpuState {
    context: DirectContext,
    framebuffer_info: FramebufferInfo,
}

#[cfg(feature = "gl")]
enum Backend {
    Gpu {
        gpu_state: GpuState,
        surface: Surface,
    },
}

#[cfg(not(feature = "gl"))]
enum Backend {
    Raster {
        pixels: Vec<u8>,
        context: CanvasRenderingContext2d,
    },
}

fn paint() -> Paint {
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);
    paint
}

#[cfg(feature = "gl")]
impl GpuState {
    fn new(gl_ctx: WebGl2RenderingContext) -> Result<Self, JsValue> {
        let id = skia_safe::gpu::gl::register_gl_context(gl_ctx);
        skia_safe::gpu::gl::set_gl_context(id);

        let interface = skia_safe::gpu::gl::Interface::new_web_sys()
            .ok_or_else(|| JsValue::from_str("failed to create WebGL2 GL interface"))?;
        let context = gpu::direct_contexts::make_gl(interface, None)
            .ok_or_else(|| JsValue::from_str("failed to create Skia DirectContext"))?;
        let framebuffer_info = FramebufferInfo {
            fboid: 0_u32,
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            protected: skia_safe::gpu::Protected::No,
        };

        Ok(Self {
            context,
            framebuffer_info,
        })
    }
}

#[cfg(feature = "gl")]
fn create_surface(gpu_state: &mut GpuState, width: i32, height: i32) -> Result<Surface, JsValue> {
    let target =
        gpu::backend_render_targets::make_gl((width, height), 0, 8, gpu_state.framebuffer_info);
    gpu::surfaces::wrap_backend_render_target(
        &mut gpu_state.context,
        &target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .ok_or_else(|| JsValue::from_str("failed to create Skia surface"))
}

#[wasm_bindgen]
pub fn uses_webgl() -> bool {
    cfg!(feature = "gl")
}

#[wasm_bindgen]
pub struct State {
    width: i32,
    height: i32,
    backend: Backend,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, context: JsValue) -> Result<Self, JsValue> {
        let (width, height) = (width as i32, height as i32);

        #[cfg(feature = "gl")]
        {
            let context = context
                .dyn_into::<WebGl2RenderingContext>()
                .map_err(|_| JsValue::from_str("expected a WebGl2RenderingContext"))?;
            let mut gpu_state = GpuState::new(context)?;
            let surface = create_surface(&mut gpu_state, width, height)?;
            Ok(Self {
                width,
                height,
                backend: Backend::Gpu { gpu_state, surface },
            })
        }

        #[cfg(not(feature = "gl"))]
        {
            let context = context
                .dyn_into::<CanvasRenderingContext2d>()
                .map_err(|_| JsValue::from_str("expected a CanvasRenderingContext2d"))?;
            Ok(Self {
                width,
                height,
                backend: Backend::Raster {
                    pixels: vec![0u8; (width * height * 4) as usize],
                    context,
                },
            })
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.width = width as i32;
        self.height = height as i32;

        #[cfg(feature = "gl")]
        {
            let Backend::Gpu { gpu_state, surface } = &mut self.backend;
            *surface = create_surface(gpu_state, self.width, self.height)?;
            Ok(())
        }

        #[cfg(not(feature = "gl"))]
        {
            let Backend::Raster { pixels, .. } = &mut self.backend;
            pixels.resize((self.width * self.height * 4) as usize, 0);
            Ok(())
        }
    }

    pub fn draw(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        let paint = paint();

        #[cfg(feature = "gl")]
        {
            let Backend::Gpu { gpu_state, surface } = &mut self.backend;
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            canvas.draw_circle((x, y), 50.0, &paint);
            gpu_state.context.flush_and_submit_surface(surface, None);
            Ok(())
        }

        #[cfg(not(feature = "gl"))]
        {
            let Backend::Raster { pixels, context } = &mut self.backend;
            let info = ImageInfo::new(
                (self.width, self.height),
                ColorType::RGBA8888,
                AlphaType::Premul,
                None,
            );
            let row_bytes = self.width as usize * 4;
            let canvas = skia_safe::Canvas::from_raster_direct(
                &info,
                pixels.as_mut_slice(),
                Some(row_bytes),
                None,
            )
            .ok_or_else(|| JsValue::from_str("failed to create raster canvas"))?;

            canvas.clear(Color::WHITE);
            canvas.draw_circle((x, y), 50.0, &paint);
            drop(canvas);

            let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(pixels.as_slice()),
                self.width as u32,
                self.height as u32,
            )?;
            context.put_image_data(&image_data, 0.0, 0.0)
        }
    }
}
