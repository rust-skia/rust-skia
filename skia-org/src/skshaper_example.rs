use crate::DrawingDriver;
use skia_safe::{Canvas, Font, Paint, Point, Shaper, Typeface};
use std::path;

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = path.join("SkShaper-Example");

    driver.draw_image_256(&path, "rtl-shaped", draw_rtl_shaped);
    driver.draw_image_256(&path, "rtl-unshaped", draw_rtl_unshaped);
}

const RTL_TEXT: &str = "العربية";
const TEXT_POS: Point = Point::new(0.0, 64.0);

fn draw_rtl_shaped(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let font = &Font::from_typeface(Typeface::default(), 64.0);

    let shaper = Shaper::new(None);
    if let Some((blob, _)) =
        shaper.shape_text_blob(RTL_TEXT, font, false, 10000.0, Point::default())
    {
        canvas.draw_text_blob(&blob, TEXT_POS, &paint);
    }
}

fn draw_rtl_unshaped(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let font = &Font::from_typeface(Typeface::default(), 64.0);

    let shaper = Shaper::new_primitive();
    if let Some((blob, _)) =
        shaper.shape_text_blob(RTL_TEXT, font, false, 10000.0, Point::default())
    {
        canvas.draw_text_blob(&blob, TEXT_POS, &paint);
    }
}
