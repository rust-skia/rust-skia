use skia_safe::{scalar, Canvas, Color4f, ColorType, Paint, Point, Rect, Size, Surface};

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("This example is only supported on macos")
}

#[cfg(all(target_os = "macos", not(feature = "metal")))]
fn main() {
    println!("To run this example, invoke cargo with --features \"metal\".")
}

#[cfg(all(target_os = "macos", feature = "metal"))]
fn main() {
    use cocoa::{appkit::NSView, base::id as cocoa_id};

    use core_graphics_types::geometry::CGSize;
    use std::mem;

    use foreign_types_shared::{ForeignType, ForeignTypeRef};
    use metal_rs::{Device, MTLPixelFormat, MetalLayer};
    use objc::{rc::autoreleasepool, runtime::YES};

    use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};

    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::macos::WindowExtMacOS,
        window::WindowBuilder,
    };

    let size = LogicalSize::new(800, 600);

    let events_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Skia Metal Winit Example".to_string())
        .build(&events_loop)
        .unwrap();

    let device = Device::system_default().expect("no device found");

    let metal_layer = {
        let draw_size = window.inner_size();
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            let view = window.ns_view() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(mem::transmute(layer.as_ref()));
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

    events_loop.run(move |event, _, control_flow| {
        autoreleasepool(|| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        metal_layer
                            .set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
                        window.request_redraw()
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    if let Some(drawable) = metal_layer.next_drawable() {
                        let drawable_size = {
                            let size = metal_layer.drawable_size();
                            Size::new(size.width as scalar, size.height as scalar)
                        };

                        let mut surface = unsafe {
                            let texture_info =
                                mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);

                            let backend_render_target = BackendRenderTarget::new_metal(
                                (drawable_size.width as i32, drawable_size.height as i32),
                                1,
                                &texture_info,
                            );

                            Surface::from_backend_render_target(
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

                        surface.flush_and_submit();
                        drop(surface);

                        let command_buffer = command_queue.new_command_buffer();
                        command_buffer.present_drawable(drawable);
                        command_buffer.commit();
                    }
                }
                _ => {}
            }
        });
    });
}

/// Renders a rectangle that occupies exactly half of the canvas
fn draw(canvas: &mut Canvas) {
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
