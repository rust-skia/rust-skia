#![allow(dead_code)]

#[cfg(target_os = "android")]
fn main() {
    println!(
        "This example is not supported on Android (https://github.com/rust-windowing/winit/issues/948)."
    )
}

#[cfg(target_os = "emscripten")]
fn main() {
    println!(
        "This example is not supported on Emscripten (https://github.com/rust-windowing/glutin/issues/1349)"
    )
}

#[cfg(target_os = "ios")]
fn main() {
    println!(
        "This example is not supported on iOS (https://github.com/rust-windowing/glutin/issues/1448)"
    )
}

#[cfg(all(
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios"),
    not(feature = "gl")
))]
fn main() {
    println!("To run this example, invoke cargo with --features \"gl\".")
}

#[cfg(all(
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios"),
    feature = "gl"
))]
fn main() {
    use std::{ffi::CString, num::NonZeroU32};

    use gl::types::*;
    use gl_rs as gl;
    use glutin::{
        config::{ConfigTemplateBuilder, GlConfig},
        context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
        display::{GetGlDisplay, GlDisplay},
        prelude::{GlSurface, NotCurrentGlContext},
        surface::{
            Surface as GlutinSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface,
        },
    };
    use glutin_winit::DisplayBuilder;
    use raw_window_handle::HasWindowHandle;
    use winit::{
        application::ApplicationHandler,
        dpi::LogicalSize,
        event::{KeyEvent, Modifiers, WindowEvent},
        event_loop::EventLoop,
        window::{Window, WindowAttributes},
    };

    use skia_safe::{
        Color, ColorType, Surface,
        gpu::{self, SurfaceOrigin, backend_render_targets, gl::FramebufferInfo},
    };

    use skia_icon as renderer;

    let el = EventLoop::new().expect("Failed to create event loop");

    let window_attributes = WindowAttributes::default()
        .with_title("rust-skia-gl-window")
        .with_inner_size(LogicalSize::new(800, 800));

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);

    let display_builder = DisplayBuilder::new().with_window_attributes(window_attributes.into());
    let (window, gl_config) = display_builder
        .build(&el, template, |configs| {
            // Find the config with the minimum number of samples. Usually Skia takes care of
            // anti-aliasing and may not be able to create appropriate Surfaces for samples > 0.
            // See https://github.com/rust-skia/rust-skia/issues/782
            // And https://github.com/rust-skia/rust-skia/issues/764
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();
    println!("Picked a config with {} samples", gl_config.num_samples());
    let window = window.expect("Could not create window with OpenGL context");
    let window_handle = window
        .window_handle()
        .expect("Failed to retrieve RawWindowHandle");
    let raw_window_handle = window_handle.as_raw();

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_config
                    .display()
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    };

    let (width, height): (u32, u32) = window.inner_size().into();

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    );

    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .expect("Could not create gl window surface")
    };

    let gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Could not make GL context current when setting up skia renderer");

    gl::load_with(|s| {
        gl_config
            .display()
            .get_proc_address(CString::new(s).unwrap().as_c_str())
    });
    let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
        if name == "eglGetCurrentDisplay" {
            return std::ptr::null();
        }
        gl_config
            .display()
            .get_proc_address(CString::new(name).unwrap().as_c_str())
    })
    .expect("Could not create interface");

    let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, None)
        .expect("Could not create direct context");

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        }
    };

    fn create_surface(
        window: &Window,
        fb_info: FramebufferInfo,
        gr_context: &mut skia_safe::gpu::DirectContext,
        num_samples: usize,
        stencil_size: usize,
    ) -> Surface {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target =
            backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

        gpu::surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create skia surface")
    }

    let num_samples = gl_config.num_samples() as usize;
    let stencil_size = gl_config.stencil_size() as usize;

    let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size);

    // Guarantee the drop order inside the FnMut closure. `Window` _must_ be dropped after
    // `DirectContext`.
    //
    // <https://github.com/rust-skia/rust-skia/issues/476>
    struct Env {
        surface: Surface,
        gl_surface: GlutinSurface<WindowSurface>,
        gr_context: skia_safe::gpu::DirectContext,
        gl_context: PossiblyCurrentContext,
        window: Window,
    }

    impl Drop for Env {
        fn drop(&mut self) {
            // This fixes a segmentation fault on AMD GPUs, see
            // <https://github.com/rust-skia/rust-skia/pull/1235> and
            // <https://github.com/marc2332/freya/issues/347> for more details.
            self.gr_context.release_resources_and_abandon();
        }
    }

    // Synchronize buffer swaps with the display's video frames (vsync) to get
    // a tearing-free, full refresh-rate animation.
    gl_surface
        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        .expect("Could not set swap interval");

    let env = Env {
        surface,
        gl_surface,
        gl_context,
        gr_context,
        window,
    };

    struct Application {
        env: Env,
        fb_info: FramebufferInfo,
        num_samples: usize,
        stencil_size: usize,
        modifiers: Modifiers,
        frame: usize,
        // Whether the window content is currently not presented (minimized
        // or fully occluded), as reported by winit's `Occluded` event.
        occluded: bool,
    }

    let mut application = Application {
        env,
        fb_info,
        num_samples,
        stencil_size,
        modifiers: Modifiers::default(),
        frame: 0,
        occluded: false,
    };

    impl ApplicationHandler for Application {
        fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
            // Kick off the VBlank loop: request → draw → swap (blocks on
            // vblank) → request → …
            self.env.window.request_redraw();
        }

        fn new_events(
            &mut self,
            _event_loop: &winit::event_loop::ActiveEventLoop,
            _cause: winit::event::StartCause,
        ) {
            // Intentionally does not request redraws: the chain is sustained
            // from `RedrawRequested` (see below).
        }

        fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
            // While the window is minimized, break the redraw chain: winit's
            // Windows backend never delivers `Occluded` events (macOS/iOS/Web
            // only), so winit's own `is_minimized` query is the portable
            // presentability check there. No re-request => the loop sleeps.
            if self.env.window.is_minimized().unwrap_or(false) {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                return;
            }
            // Sustain the VBlank loop while the window is presentable.
            self.env.window.request_redraw()
        }

        fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Occluded(occluded) => {
                    // macOS/iOS/Web signal presentability through occlusion
                    // (AppKit delivers this on minimize via
                    // windowDidChangeOcclusionState). Re-arming on the clear
                    // edge sustains the loop after occlusion ends.
                    self.occluded = occluded;
                    if !occluded {
                        self.env.window.request_redraw()
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    self.env.surface = create_surface(
                        &self.env.window,
                        self.fb_info,
                        &mut self.env.gr_context,
                        self.num_samples,
                        self.stencil_size,
                    );
                    /* First resize the opengl drawable */
                    let (width, height): (u32, u32) = physical_size.into();

                    self.env.gl_surface.resize(
                        &self.env.gl_context,
                        NonZeroU32::new(width.max(1)).unwrap(),
                        NonZeroU32::new(height.max(1)).unwrap(),
                    );
                }
                WindowEvent::ModifiersChanged(new_modifiers) => self.modifiers = new_modifiers,
                WindowEvent::KeyboardInput {
                    event: KeyEvent { logical_key, .. },
                    ..
                } => {
                    if self.modifiers.state().super_key() && logical_key == "q" {
                        event_loop.exit();
                    }
                    self.frame = self.frame.saturating_sub(10);
                    self.env.window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    // The VBlank loop: `swap_buffers` blocks until the next
                    // video frame (vblank), which paces this loop at the
                    // display refresh rate; each completed swap re-requests
                    // the next redraw.
                    //
                    // When the window cannot be presented (minimized/occluded)
                    // the swap no longer blocks — so the loop is broken off
                    // instead: no draw, no swap, no re-request. It is re-armed
                    // by Occluded(false), or by `about_to_wait` once
                    // `is_minimized` reports false again.
                    if self.occluded {
                        return;
                    }
                    self.frame += 1;
                    let canvas = self.env.surface.canvas();
                    canvas.clear(Color::WHITE);
                    // At fps=180/bpm=60 a full revolution takes
                    // 360 / (12 · 60/60/180) = 5400 frames; wrapping there is
                    // seamless, so the gear simply keeps rotating.
                    renderer::render_frame(self.frame % 5400, 180, 60, canvas);
                    self.env.gr_context.flush_and_submit();
                    self.env
                        .gl_surface
                        .swap_buffers(&self.env.gl_context)
                        .unwrap();
                    self.env.window.request_redraw();
                }
                _ => (),
            }
        }
    }

    el.run_app(&mut application).expect("run() failed");
}
