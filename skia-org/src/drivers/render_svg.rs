use std::path::Path;

use skia_safe::{surfaces, svg, Canvas, FontMgr, Rect};

use crate::{artifact, DrawingDriver, Driver};

pub struct RenderSvg {
    font_mgr: FontMgr,
}

impl DrawingDriver for RenderSvg {
    const DRIVER: Driver = Driver::RenderSvg;

    fn new() -> Self {
        Self {
            font_mgr: FontMgr::new(),
        }
    }

    fn draw_image(
        &mut self,
        size @ (width, height): (i32, i32),
        path: &Path,
        name: &str,
        draw: impl Fn(&Canvas),
    ) {
        let canvas = svg::Canvas::new(Rect::from_size(size), None);
        draw(&canvas);
        let data = canvas.end();
        let svg = data.as_bytes();

        let svg_dom = svg::Dom::from_bytes(svg, &self.font_mgr).unwrap();

        let mut surface = surfaces::raster_n32_premul((width * 2, height * 2)).unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, |canvas| svg_dom.render(canvas));
    }
}
