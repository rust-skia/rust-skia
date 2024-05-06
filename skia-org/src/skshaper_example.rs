use std::path;

use skia_safe::{shapers, Canvas, Font, FontMgr, Paint, Point, Shaper};

use crate::{helper::default_typeface, DrawingDriver};

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = path.join("SkShaper-Example");

    driver.draw_image_256(&path, "rtl-shaped", draw_rtl_shaped);
    driver.draw_image_256(&path, "rtl-unshaped", draw_rtl_unshaped);
}

const RTL_TEXT: &str = "العربية";
const TEXT_POS: Point = Point::new(0.0, 64.0);

fn draw_rtl_shaped(canvas: &Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let font = &Font::from_typeface(default_typeface(), 64.0);

    let shaper = Shaper::new(FontMgr::new());
    if let Some((blob, _)) =
        shaper.shape_text_blob(RTL_TEXT, font, false, 10000.0, Point::default())
    {
        canvas.draw_text_blob(&blob, TEXT_POS, &paint);
    }
}

fn draw_rtl_unshaped(canvas: &Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let font = &Font::from_typeface(default_typeface(), 64.0);

    let shaper = shapers::primitive::primitive_text();
    if let Some((blob, _)) =
        shaper.shape_text_blob(RTL_TEXT, font, false, 10000.0, Point::default())
    {
        canvas.draw_text_blob(&blob, TEXT_POS, &paint);
    }
}
