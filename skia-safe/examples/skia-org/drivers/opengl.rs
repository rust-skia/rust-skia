use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::{gpu, Budgeted, Canvas, ImageInfo, Surface};
use std::path::Path;

pub enum OpenGL {}

impl DrawingDriver for OpenGL {
    const NAME: &'static str = "opengl";

    fn draw_image(
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        let mut context = gpu::Context::new_gl(None).unwrap();

        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = Surface::new_render_target(
            &mut context,
            Budgeted::YES,
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
