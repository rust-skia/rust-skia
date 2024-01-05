#[cfg(not(target_os = "macos"))]
fn main() {
    println!("This example is only supported on macos")
}

#[cfg(all(target_os = "macos", not(feature = "metal")))]
fn main() {
    println!("To run this example, invoke cargo with --features \"metal\".")
}

#[cfg(all(target_os = "macos", feature = "metal"))]
use skia_safe::{scalar, Canvas, Color4f, ColorType, Paint, Point, Rect, Size};

#[cfg(all(target_os = "macos", feature = "metal"))]
fn main() {
    use cocoa::{appkit::NSView, base::id as cocoa_id};
    use core_graphics_types::geometry::CGSize;
    use foreign_types_shared::{ForeignType, ForeignTypeRef};
    use metal_rs::{Device, MTLPixelFormat, MetalLayer};
    use objc::{rc::autoreleasepool, runtime::YES};
    use skia_safe::gpu::{self, mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        raw_window_handle::HasWindowHandle,
        window::WindowBuilder,
    };

    let size = LogicalSize::new(800, 600);

    let events_loop = EventLoop::new().expect("Failed to create event loop");

    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Skia Metal Winit Example".to_string())
        .build(&events_loop)
        .unwrap();

    let window_handle = window
        .window_handle()
        .expect("Failed to retrieve a window handle");

    let raw_window_handle = window_handle.as_raw();

    let device = Device::system_default().expect("no device found");

    let metal_layer = {
        let draw_size = window.inner_size();
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        // Disabling this option allows Skia's Blend Mode to work.
        // More about: https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly
        layer.set_framebuffer_only(false);

        unsafe {
            let view = match raw_window_handle {
                raw_window_handle::RawWindowHandle::AppKit(appkit) => appkit.ns_view.as_ptr(),
                _ => panic!("Wrong window handle type"),
            } as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(layer.as_ref() as *const _ as _);
        }
        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
        layer
    };

    let command_queue = device.new_command_queue();

    let backend = unsafe {
        mtl::BackendContext::new(
            device.as_ptr() as mtl::Handle,
            command_queue.as_ptr() as mtl::Handle,
            std::ptr::null(),
        )
    };

    let mut context = DirectContext::new_metal(&backend, None).unwrap();

    events_loop
        .run(move |event, window_target| {
            autoreleasepool(|| {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => window_target.exit(),
                        WindowEvent::Resized(size) => {
                            metal_layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                            window.request_redraw()
                        }
                        WindowEvent::RedrawRequested => {
                            if let Some(drawable) = metal_layer.next_drawable() {
                                let drawable_size = {
                                    let size = metal_layer.drawable_size();
                                    Size::new(size.width as scalar, size.height as scalar)
                                };

                                let mut surface = unsafe {
                                    let texture_info = mtl::TextureInfo::new(
                                        drawable.texture().as_ptr() as mtl::Handle,
                                    );

                                    let backend_render_target = BackendRenderTarget::new_metal(
                                        (drawable_size.width as i32, drawable_size.height as i32),
                                        &texture_info,
                                    );

                                    gpu::surfaces::wrap_backend_render_target(
                                        &mut context,
                                        &backend_render_target,
                                        SurfaceOrigin::TopLeft,
                                        ColorType::BGRA8888,
                                        None,
                                        None,
                                    )
                                    .unwrap()
                                };

                                draw(surface.canvas());

                                context.flush_and_submit();
                                drop(surface);

                                let command_buffer = command_queue.new_command_buffer();
                                command_buffer.present_drawable(drawable);
                                command_buffer.commit();
                            }
                        }
                        _ => (),
                    }
                }
            });
        })
        .expect("run() failed");
}

/// Renders a rectangle that occupies exactly half of the canvas
#[cfg(all(target_os = "macos", feature = "metal"))]
fn draw(canvas: &Canvas) {
    let canvas_size = Size::from(canvas.base_layer_size());

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
}
