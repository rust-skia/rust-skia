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
    context: gpu::Context,
    queue: CommandQueue,
    device: Device,
    pool: Pool,
}

impl DrawingDriver for Metal {
    const NAME: &'static str = "metal";

    fn new() -> Self {
        let pool = Pool(unsafe { NSAutoreleasePool::new(cocoa::base::nil) });

        let device = Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();

        let context = unsafe {
            gpu::Context::new_metal(
                device.as_ptr() as *mut ffi::c_void,
                queue.as_ptr() as *mut ffi::c_void,
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
        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = Surface::new_render_target(
            &mut self.context,
            Budgeted::YES,
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

struct Pool(*mut objc::runtime::Object);

impl Drop for Pool {
    fn drop(&mut self) {
        #[allow(clippy::let_unit_value)]
        unsafe {
            // the unit value here is needed  to type the return of msg_send().
            let () = msg_send![self.0, release];
        }
    }
}
