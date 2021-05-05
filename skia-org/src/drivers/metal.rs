use crate::artifact;
use crate::drivers::DrawingDriver;
use cocoa::foundation::NSAutoreleasePool;
use foreign_types::ForeignType;
use metal_rs::*;
use skia_safe::gpu;
use skia_safe::{Budgeted, Canvas, ImageInfo, Surface};
use std::ffi;
use std::path::Path;

#[allow(dead_code)]
pub struct Metal {
    // note: ordered for drop order
    context: gpu::DirectContext,
    queue: CommandQueue,
    device: Device,
    pool: AutoreleasePool,
}

impl DrawingDriver for Metal {
    const NAME: &'static str = "metal";

    fn new() -> Self {
        let pool = AutoreleasePool::new();

        let device = Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();

        let context = unsafe {
            gpu::DirectContext::new_metal(
                device.as_ptr() as *mut ffi::c_void,
                queue.as_ptr() as *mut ffi::c_void,
                None,
            )
        }
        .unwrap();

        Self {
            pool,
            device,
            queue,
            context,
        }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        let _image_pool = AutoreleasePool::new();

        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = Surface::new_render_target(
            &mut self.context,
            Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::TopLeft,
            None,
            false,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }
}

struct AutoreleasePool(*mut objc::runtime::Object);

impl AutoreleasePool {
    fn new() -> Self {
        Self(unsafe { NSAutoreleasePool::new(cocoa::base::nil) })
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        #[allow(clippy::let_unit_value)]
        unsafe {
            // the unit value here is needed  to type the return of msg_send().
            let () = msg_send![self.0, release];
        }
    }
}
