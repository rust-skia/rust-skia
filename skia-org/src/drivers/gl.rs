use std::path::Path;

use skia_safe::{gpu, Canvas, ImageInfo};

use crate::{artifact, drivers::DrawingDriver, Driver};

pub struct OpenGl {
    context: gpu::DirectContext,
}

impl DrawingDriver for OpenGl {
    const DRIVER: Driver = Driver::OpenGl;

    fn new() -> Self {
        let interface = gpu::gl::Interface::new_native().unwrap();
        Self {
            context: gpu::direct_contexts::make_gl(interface, None).unwrap(),
        }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&Canvas),
    ) {
        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = gpu::surfaces::render_target(
            &mut self.context,
            gpu::Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::BottomLeft,
            None,
            false,
            None,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }
}
