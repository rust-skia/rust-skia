use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::Canvas;
use std::path::Path;

pub enum SVG {}

impl DrawingDriver for SVG {
    const NAME: &'static str = "svg";

    fn draw_image(size: (i32, i32), path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        use skia_safe::Rect;
        let mut canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
        func(&mut canvas);
        let data = canvas.end();
        artifact::write_file(data.as_bytes(), path, name, "svg");
    }
}
