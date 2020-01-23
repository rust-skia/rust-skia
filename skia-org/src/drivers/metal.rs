use crate::artifact;
use crate::drivers::DrawingDriver;
use foreign_types::ForeignType;
use metal_rs::*;
use skia_safe::gpu;
use skia_safe::{Budgeted, Canvas, ImageInfo, Surface};
use std::ffi;
use std::path::Path;

pub enum Metal {}
use cocoa::foundation::NSAutoreleasePool;

impl DrawingDriver for Metal {
    const NAME: &'static str = "metal";

    fn draw_image(
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        let pool = unsafe { NSAutoreleasePool::new(cocoa::base::nil) };

        let device = Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();

        let mut context = gpu::Context::new_metal(
            device.as_ptr() as *mut ffi::c_void,
            queue.as_ptr() as *mut ffi::c_void,
        )
        .unwrap();

        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = Surface::new_render_target(
            &mut context,
            Budgeted::YES,
            &image_info,
            None,
            gpu::SurfaceOrigin::TopLeft,
            None,
            false,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);

        unsafe {
            let () = msg_send![pool, release];
        }
    }
}
