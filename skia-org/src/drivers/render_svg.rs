use std::path::Path;

use skia_safe::{surfaces, svg, Canvas, Rect};

use crate::{artifact, DrawingDriver, Driver};

pub struct RenderSvg;

impl DrawingDriver for RenderSvg {
    const DRIVER: Driver = Driver::RenderSvg;

    fn new() -> Self {
        Self
    }

    fn draw_image(
        &mut self,
        size @ (width, height): (i32, i32),
        path: &Path,
        name: &str,
        draw: impl Fn(&mut Canvas),
    ) {
        let mut canvas = svg::Canvas::new(Rect::from_size(size), None);
        draw(&mut canvas);
        let data = canvas.end();
        let svg = data.as_bytes();

        let svg_dom = svg::Dom::from_bytes(svg).unwrap();

        let mut surface = surfaces::raster_n32_premul((width * 2, height * 2)).unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, |canvas| svg_dom.render(canvas));
    }
}
