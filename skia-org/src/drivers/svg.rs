use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::Canvas;
use std::path::Path;

pub struct Svg;

impl DrawingDriver for Svg {
    const NAME: &'static str = "svg";

    fn new() -> Self {
        Self
    }

    fn draw_image(
        &mut self,
        size: (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        use skia_safe::Rect;
        let mut canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
        func(&mut canvas);
        let data = canvas.end();
        artifact::write_file(data.as_bytes(), path, name, "svg");
    }
}
