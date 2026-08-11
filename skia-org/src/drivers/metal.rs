use std::path::Path;

use objc2::rc::Retained;
use objc2::rc::autoreleasepool;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSAutoreleasePool;
use objc2_metal::{MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice};

use crate::Driver;
use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::gpu::{self, mtl};
use skia_safe::{Canvas, ImageInfo};

#[allow(dead_code)]
pub struct Metal {
    // note: ordered for drop order
    context: gpu::DirectContext,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    pool: Retained<NSAutoreleasePool>,
}

impl DrawingDriver for Metal {
    const DRIVER: Driver = Driver::Metal;

    fn new() -> Self {
        let pool = unsafe { NSAutoreleasePool::new() };

        let device = MTLCreateSystemDefaultDevice().expect("no Metal device");
        let queue = device.newCommandQueue().expect("no Metal command queue");

        let backend = unsafe {
            mtl::BackendContext::new(
                Retained::as_ptr(&device) as mtl::Handle,
                Retained::as_ptr(&queue) as mtl::Handle,
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
