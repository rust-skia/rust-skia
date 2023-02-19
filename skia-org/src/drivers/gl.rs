use crate::{artifact, drivers::DrawingDriver, Driver};
use skia_safe::{gpu, Canvas, ImageInfo, Surface};
use std::path::Path;

pub struct OpenGl {
    context: gpu::DirectContext,
}

impl DrawingDriver for OpenGl {
    const DRIVER: Driver = Driver::OpenGl;

    fn new() -> Self {
        Self {
            context: gpu::DirectContext::new_gl(None, None).unwrap(),
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
            gpu::Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::BottomLeft,
            None,
            false,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }
}
