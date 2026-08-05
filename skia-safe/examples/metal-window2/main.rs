#![allow(deprecated)]

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("This example is only supported on macos")
}

#[cfg(all(target_os = "macos", not(feature = "metal")))]
fn main() {
    println!("To run this example, invoke cargo with --features \"metal\".")
}

#[cfg(all(target_os = "macos", feature = "metal", not(feature = "graphite")))]
fn main() {
    println!("To run this example, invoke cargo with --features \"graphite\".")
}

#[cfg(all(target_os = "macos", feature = "metal", feature = "graphite"))]
fn main() {
    use objc2::{
        rc::{Retained, autoreleasepool},
        runtime::ProtocolObject,
    };
    use objc2_core_foundation::CGSize;
    use objc2_metal::{MTLCommandBuffer, MTLCommandQueue};
    use objc2_quartz_core::CAMetalDrawable;

    use winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::{ActiveEventLoop, EventLoop},
        window::WindowId,
    };

    use skia_safe::{
        ColorType,
        graphite::{self, mtl as graphite_mtl},
    };

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    struct Application {
        context: Option<window::Context>,
    }

    let mut application = Application { context: None };

    impl ApplicationHandler for Application {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            assert!(self.context.is_none());
            self.context = Some(window::Context::new(event_loop))
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: WindowId,
            event: WindowEvent,
        ) {
            let context = &mut self.context.as_mut().unwrap();
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(size) => {
                    context
                        .metal_layer
                        .setDrawableSize(CGSize::new(size.width as f64, size.height as f64));
                    context.window.request_redraw()
                }
                WindowEvent::RedrawRequested => {
                    if let Some(drawable) = context.metal_layer.nextDrawable() {
                        let (drawable_width, drawable_height) = {
                            let size = context.metal_layer.drawableSize();
                            (size.width as i32, size.height as i32)
                        };

                        // Create recorder for this frame
                        let recorder_options = graphite::RecorderOptions::default();
                        let mut recorder = context
                            .skia
                            .make_recorder(Some(&recorder_options))
                            .expect("Failed to create recorder");

                        // Create backend texture from Metal drawable
                        let backend_texture = unsafe {
                            graphite_mtl::make_backend_texture(
                                (drawable_width, drawable_height),
                                Retained::as_ptr(&drawable.texture()) as graphite_mtl::Handle,
                            )
                        };

                        // Create surface from backend texture
                        let mut surface = graphite::surfaces::wrap_backend_texture(
                            &mut recorder,
                            &backend_texture,
                            ColorType::BGRA8888,
                            None,
                            None,
                        )
                        .expect("Failed to create surface");

                        // Draw on the surface
                        window::draw(surface.canvas());

                        // Finish recording
                        let mut recording = recorder.snap().expect("Failed to snap recording");

                        // Insert recording into context
                        let insert_info = graphite::InsertRecordingInfo::new(&mut recording);
                        context.skia.insert_recording(&insert_info);
                        // `insert_recording` only borrows the recording (it copies what it
                        // needs synchronously), so dropping it here at end of scope is correct.

                        // Submit work to GPU
                        context.skia.submit(None);

                        // Present drawable
                        let command_buffer = context
                            .command_queue
                            .commandBuffer()
                            .expect("unable to get command buffer");
                        let drawable: Retained<ProtocolObject<dyn objc2_metal::MTLDrawable>> =
                            (&drawable).into();
                        command_buffer.presentDrawable(&drawable);
                        command_buffer.commit();
                    }

                    // request redraw to continuously render frames (remove if you only want to render on demand)
                    context.window.request_redraw();
                }
                _ => (),
            }
        }
    }

    autoreleasepool(|_| {
        event_loop.run_app(&mut application).expect("run() failed");
    })
}

#[cfg(all(target_os = "macos", feature = "metal", feature = "graphite"))]
mod window {
    use objc2::{rc::Retained, runtime::ProtocolObject};
    use objc2_app_kit::NSView;
    use objc2_core_foundation::CGSize;
    use objc2_metal::{MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice};
    use objc2_quartz_core::CAMetalLayer;
    use skia_safe::{
        Canvas, Color4f, Paint, Point, Rect,
        graphite::{self, Context as GraphiteContext, mtl as graphite_mtl},
    };
    use winit::{
        dpi::{LogicalSize, Size},
        event_loop::ActiveEventLoop,
        raw_window_handle::HasWindowHandle,
        window::{Window, WindowAttributes},
    };

    pub struct Context {
        pub window: Window,
        pub metal_layer: Retained<CAMetalLayer>,
        pub command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
        pub skia: GraphiteContext,
    }

    impl Context {
        pub fn new(event_loop: &ActiveEventLoop) -> Self {
            let size = LogicalSize::new(800, 600);
            let mut window_attributes = WindowAttributes::default();
            window_attributes.inner_size = Some(Size::new(size));
            window_attributes.title = "Skia Graphite Metal Winit Example".to_string();

            let window = event_loop
                .create_window(window_attributes)
                .expect("Failed to create Window");

            let device = MTLCreateSystemDefaultDevice().expect("no device found");

            let metal_layer = {
                let layer = CAMetalLayer::new();
                layer.setDevice(Some(&device));
                layer.setPixelFormat(objc2_metal::MTLPixelFormat::BGRA8Unorm);
                layer.setPresentsWithTransaction(false);
                // Disabling this option allows Skia's Blend Mode to work.
                // More about: https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly
                layer.setFramebufferOnly(false);
                layer.setDrawableSize(CGSize::new(size.width as f64, size.height as f64));

                let view_ptr = match window.window_handle().unwrap().as_raw() {
                    raw_window_handle::RawWindowHandle::AppKit(appkit) => {
                        appkit.ns_view.as_ptr() as *mut NSView
                    }
                    _ => panic!("Wrong window handle type"),
                };
                let view = unsafe { view_ptr.as_ref().unwrap() };

                view.setWantsLayer(true);
                view.setLayer(Some(&layer.clone().into_super()));

                layer
            };

            let command_queue = device
                .newCommandQueue()
                .expect("unable to get command queue");

            // Create Graphite Metal backend context
            let backend = unsafe {
                graphite_mtl::BackendContext::new(
                    Retained::as_ptr(&device) as graphite_mtl::Handle,
                    Retained::as_ptr(&command_queue) as graphite_mtl::Handle,
                )
            };

            // Create Graphite context
            let context_options = graphite::ContextOptions::default();
            let skia_context = graphite_mtl::make_context(&backend, Some(&context_options))
                .expect("Failed to create Graphite context");

            Self {
                window,
                metal_layer,
                command_queue,
                skia: skia_context,
            }
        }
    }

    /// Renders a rectangle that occupies exactly half of the canvas
    pub fn draw(canvas: &Canvas) {
        let canvas_size = skia_safe::Size::from(canvas.base_layer_size());

        canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));

        let rect_size = canvas_size / 2.0;
        let rect = Rect::from_point_and_size(
            Point::new(
                (canvas_size.width - rect_size.width) / 2.0,
                (canvas_size.height - rect_size.height) / 2.0,
            ),
            rect_size,
        );

        canvas.draw_rect(rect, &Paint::new(Color4f::new(0.0, 0.0, 1.0, 1.0), None));
        canvas.draw_circle(
            Point::new(
                (canvas_size.width - rect_size.width) / 2.0,
                (canvas_size.height - rect_size.height) / 2.0,
            ),
            200.0,
            &Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None),
        );
    }
}
