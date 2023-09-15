use crate::{artifact, drivers::DrawingDriver, Driver};
use skia_safe::Canvas;
use std::path::Path;

pub struct Svg;

impl DrawingDriver for Svg {
    const DRIVER: Driver = Driver::Svg;

    fn new() -> Self {
        Self
    }

    fn draw_image(&mut self, size: (i32, i32), path: &Path, name: &str, func: impl Fn(&Canvas)) {
        use skia_safe::Rect;
        let canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
        func(&canvas);
        let data = canvas.end();
        artifact::write_file(data.as_bytes(), path, name, "svg");
    }
}
