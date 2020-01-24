use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::{Canvas, Surface};
use std::path::Path;

pub enum CPU {}

impl DrawingDriver for CPU {
    const NAME: &'static str = "cpu";

    fn draw_image(
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        let mut surface = Surface::new_raster_n32_premul((width * 2, height * 2)).unwrap();
        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }
}
