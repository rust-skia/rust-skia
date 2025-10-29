use std::path::Path;

use foreign_types_shared::ForeignType;
use metal::{CommandQueue, Device};
use objc2::rc::{autoreleasepool, Retained};
use objc2_foundation::NSAutoreleasePool;

use crate::{artifact, drivers::DrawingDriver, Driver};
use skia_safe::{
    gpu::{self, mtl},
    Canvas, ImageInfo,
};

#[allow(dead_code)]
pub struct Metal {
    // note: ordered for drop order
    context: gpu::DirectContext,
    queue: CommandQueue,
    device: Device,
    pool: Retained<NSAutoreleasePool>,
}

impl DrawingDriver for Metal {
    const DRIVER: Driver = Driver::Metal;

    fn new() -> Self {
        let pool = unsafe { NSAutoreleasePool::new() };

        let device = Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();

        let backend = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                queue.as_ptr() as mtl::Handle,
            )
        };

        let context = gpu::direct_contexts::make_metal(&backend, None).unwrap();

        Self {
            context,
            queue,
            device,
            pool,
        }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&Canvas),
    ) {
        autoreleasepool(|_| {
            let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
            let mut surface = gpu::surfaces::render_target(
                &mut self.context,
                gpu::Budgeted::Yes,
                &image_info,
                None,
                gpu::SurfaceOrigin::TopLeft,
                None,
                false,
                None,
            )
            .unwrap();

            artifact::draw_image_on_surface(&mut surface, path, name, func);
        })
    }
}
