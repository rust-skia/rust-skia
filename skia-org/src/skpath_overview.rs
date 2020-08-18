use crate::DrawingDriver;
use skia_safe::{paint, Canvas, Color, Font, Paint, Path};
use std::path;

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = path.join("SkPath-Overview");

    driver.draw_image_256(&path, "example1", draw_example1);
    driver.draw_image_256(&path, "example2", draw_example2);
    driver.draw_image_256(&path, "example3", draw_example3);
    driver.draw_image_256(&path, "example4", draw_example4);
    driver.draw_image_256(&path, "example5", draw_example5);
}

fn draw_example1(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let mut path = Path::default();
    path.move_to((124, 108))
        .line_to((172, 24))
        .add_circle((50, 50), 30.0, None)
        .move_to((36, 148))
        .quad_to((66, 188), (120, 136));
    canvas.draw_path(&path, &paint);
    paint
        .set_style(paint::Style::Stroke)
        .set_color(Color::BLUE)
        .set_stroke_width(3.0);
    canvas.draw_path(&path, &paint);
}

fn draw_example2(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let mut path = Path::default();
    path.move_to((36, 48)).quad_to((66, 88), (120, 36));
    canvas.draw_path(&path, &paint);
    paint
        .set_style(paint::Style::Stroke)
        .set_color(Color::BLUE)
        .set_stroke_width(8.0);
    canvas.translate((0, 50)).draw_path(&path, &paint);
    paint
        .set_style(paint::Style::StrokeAndFill)
        .set_color(Color::RED);
    canvas.translate((0, 50));
    canvas.draw_path(&path, &paint);
}

fn draw_example3(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    canvas
        .draw_str("1st contour", (150, 100), &Font::default(), &paint)
        .draw_str("2nd contour", (130, 160), &Font::default(), &paint)
        .draw_str("3rd contour", (40, 30), &Font::default(), &paint);
    paint.set_style(paint::Style::Stroke);
    let mut path = Path::default();
    path.move_to((124, 108))
        .line_to((172, 24))
        .move_to((36, 148))
        .quad_to((66, 188), (120, 136))
        .close()
        .conic_to((70, 20), (110, 40), 0.6);
    canvas.draw_path(&path, &paint);
}

fn draw_example4(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint
        .set_anti_alias(true)
        .set_style(paint::Style::Stroke)
        .set_stroke_width(8.0);
    let mut path = Path::default();
    path.move_to((36, 48)).quad_to((66, 88), (120, 36));
    canvas.draw_path(&path, &paint);
    path.close();
    canvas.translate((0, 50));
    canvas.draw_path(&path, &paint);
}

fn draw_example5(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint
        .set_anti_alias(true)
        .set_style(paint::Style::Stroke)
        .set_stroke_width(8.0)
        .set_stroke_cap(paint::Cap::Round);
    let mut path = Path::default();
    path.move_to((36, 48)).line_to((36, 48));
    canvas.draw_path(&path, &paint);
    path.reset();
    paint.set_stroke_cap(paint::Cap::Square);
    path.move_to((56, 48)).close();
    canvas.draw_path(&path, &paint);
}
