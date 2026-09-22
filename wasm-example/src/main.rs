use std::boxed::Box;

use skia_safe::{
    Color, Surface,
    gpu::{self, DirectContext, gl::FramebufferInfo},
};

use skia_icon::render_frame;

unsafe extern "C" {
    pub fn emscripten_GetProcAddress(
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_void;
}

struct GpuState {
    context: DirectContext,
    framebuffer_info: FramebufferInfo,
}

/// This struct holds the state of the Rust application between JS calls.
///
/// It is created by [init] and passed to the other exported functions. Note that rust-skia data
/// structures are not thread safe, so a state must not be shared between different Web Workers.
pub struct State {
    gpu_state: GpuState,
    surface: Surface,
}

impl State {
    fn new(gpu_state: GpuState, surface: Surface) -> Self {
        State { gpu_state, surface }
    }

    fn set_surface(&mut self, surface: Surface) {
        self.surface = surface;
    }
}

/// Load GL functions pointers from JavaScript so we can call OpenGL functions from Rust.
///
/// This only needs to be done once.
fn init_gl() {
    unsafe {
        gl::load_with(|addr| {
            let addr = std::ffi::CString::new(addr).unwrap();
            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
        });
    }
}

/// Create the GPU state from the JavaScript WebGL context.
///
/// This needs to be done once per WebGL context.
fn create_gpu_state() -> GpuState {
    let interface = skia_safe::gpu::gl::Interface::new_native().unwrap();
    let context = skia_safe::gpu::direct_contexts::make_gl(interface, None).unwrap();
    let framebuffer_info = {
        let mut fboid: gl::types::GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            protected: skia_safe::gpu::Protected::No,
        }
    };

    GpuState {
        context,
        framebuffer_info,
    }
}

/// Create the Skia surface that will be used for rendering.
fn create_surface(gpu_state: &mut GpuState, width: i32, height: i32) -> Surface {
    let backend_render_target =
        gpu::backend_render_targets::make_gl((width, height), 1, 8, gpu_state.framebuffer_info);

    gpu::surfaces::wrap_backend_render_target(
        &mut gpu_state.context,
        &backend_render_target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        skia_safe::ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap()
}

/// Draw the animated logo centered at the given point at half size.
fn render_logo_at(surface: &mut Surface, frame: usize, x: f32, y: f32) {
    let dims = surface.image_info().dimensions();
    let (center_x, center_y) = (dims.width as f32 / 2.0, dims.height as f32 / 2.0);
    let canvas = surface.canvas();
    // Position and scale the logo at the cursor: translate so that scaling the
    // canvas around the origin afterwards pivots at the cursor point
    // (translate = cursor - scale * render_frame's draw center). Applied
    // relative to a save/restore pair so nothing leaks between frames.
    canvas.save();
    canvas.translate((x - center_x / 2.0, y - center_y / 2.0));
    canvas.scale((0.5, 0.5));
    render_frame(frame, 60, 60, canvas);
    canvas.restore();
}

/// Initialize the renderer.
///
/// This is called from JS after the WebGL context has been created.
#[unsafe(no_mangle)]
pub extern "C" fn init(width: i32, height: i32) -> Box<State> {
    let mut gpu_state = create_gpu_state();
    let surface = create_surface(&mut gpu_state, width, height);
    let state = State::new(gpu_state, surface);
    Box::new(state)
}

/// Resize the Skia surface
///
/// This is called from JS when the window is resized.
/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resize_surface(state: *mut State, width: i32, height: i32) {
    let state = unsafe { state.as_mut() }.expect("got an invalid state pointer");
    let surface = create_surface(&mut state.gpu_state, width, height);
    state.set_surface(surface);
}

/// Draw the animated rust-skia logo centered at the given (mouse) coordinates.
///
/// `timestamp_ms` is the `requestAnimationFrame` timestamp in milliseconds; it
/// drives the animation phase.
/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn draw_logo(state: *mut State, x: i32, y: i32, timestamp_ms: f64) {
    let state = unsafe { state.as_mut() }.expect("got an invalid state pointer");
    // The renderer runs at 60 frames/s (60 fps, 60 bpm); derive the frame
    // number from the browser clock so the animation keeps its speed when the
    // tab is throttled.
    let frame = (timestamp_ms * 60.0 / 1000.0) as usize;

    state.surface.canvas().clear(Color::TRANSPARENT);
    render_logo_at(&mut state.surface, frame, x as f32, y as f32);
    // A failed flush means the rendering results are undefined and must be discarded. The surface
    // wraps the default framebuffer, which the browser composites itself, so unlike the native
    // window examples there is no present step to skip here: report the failure instead of
    // dropping it silently, and let the next frame redraw the logo from scratch.
    if let Err(err) = state
        .gpu_state
        .context
        .flush_and_submit_surface(&mut state.surface, None)
    {
        eprintln!("flush_and_submit_surface failed, the frame is undefined: {err}");
    }
}

/// The main function is called by emscripten when the WASM object is created.
fn main() {
    init_gl();
}
