use skia_safe::{
    gpu::{self, gl::FramebufferInfo, DirectContext},
    Color, ColorType, Paint, PaintStyle, Surface,
};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

struct GpuState {
    context: DirectContext,
    framebuffer_info: FramebufferInfo,
}

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
        Ok(GpuState {
            context,
            framebuffer_info,
        })
    }
}

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
pub struct State {
    width: i32,
    height: i32,
    gpu_state: GpuState,
    surface: Surface,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, context: WebGl2RenderingContext) -> Result<State, JsValue> {
        let (width, height) = (width as i32, height as i32);
        let mut gpu_state = GpuState::new(context)?;
        let surface = create_surface(&mut gpu_state, width, height)?;
        Ok(State {
            width,
            height,
            gpu_state,
            surface,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.width = width as i32;
        self.height = height as i32;
        self.surface = create_surface(&mut self.gpu_state, self.width, self.height)?;
        Ok(())
    }

    pub fn draw(&mut self, x: f32, y: f32) {
        let canvas = self.surface.canvas();
        canvas.clear(Color::WHITE);
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        canvas.draw_circle((x, y), 50.0, &paint);
        self.gpu_state
            .context
            .flush_and_submit_surface(&mut self.surface, None);
    }
}
