use std::path::PathBuf;
use skia_safe::{Canvas, Path, Paint, paint, Color, Font};
use crate::artifact::DrawingDriver;

pub fn draw<Driver: DrawingDriver>(path: &PathBuf) {
    let path = path.join("SkPath-Overview");

    Driver::draw_image_256(&path, "example1", draw_example1);
    Driver::draw_image_256(&path, "example2", draw_example2);
    Driver::draw_image_256(&path, "example3", draw_example3);
    Driver::draw_image_256(&path, "example4", draw_example4);
    Driver::draw_image_256(&path, "example5", draw_example5);
}

fn draw_example1(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let mut path = Path::default();
    path.move_to((124, 108));
    path.line_to((172, 24));
    path.add_circle((50, 50), 30.0, None);
    path.move_to((36, 148));
    path.quad_to((66, 188), (120, 136));
    canvas.draw_path(&path, &paint);
    paint.set_style(paint::Style::Stroke);
    paint.set_color(Color::BLUE);
    paint.set_stroke_width(3.0);
    canvas.draw_path(&path, &paint);
}

fn draw_example2(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let mut path = Path::default();
    path.move_to((36, 48));
    path.quad_to((66, 88), (120, 36));
    canvas.draw_path(&path, &paint);
    paint.set_style(paint::Style::Stroke);
    paint.set_color(Color::BLUE);
    paint.set_stroke_width(8.0);
    canvas.translate((0, 50));
    canvas.draw_path(&path, &paint);
    paint.set_style(paint::Style::StrokeAndFill);
    paint.set_color(Color::RED);
    canvas.translate((0, 50));
    canvas.draw_path(&path, &paint);
}

fn draw_example3(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    canvas.draw_str("1st contour", (150, 100), &Font::default(), &paint);
    canvas.draw_str("2nd contour", (130, 160), &Font::default(), &paint);
    canvas.draw_str("3rd contour", (40, 30), &Font::default(), &paint);
    paint.set_style(paint::Style::Stroke);
    let mut path = Path::default();
    path.move_to((124, 108));
    path.line_to((172, 24));
    path.move_to((36, 148));
    path.quad_to((66, 188), (120, 136));
    path.close();
    path.conic_to((70, 20), (110, 40), 0.6);
    canvas.draw_path(&path, &paint);
}

fn draw_example4(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(paint::Style::Stroke);
    paint.set_stroke_width(8.0);
    let mut path = Path::default();
    path.move_to((36, 48));
    path.quad_to((66, 88), (120, 36));
    canvas.draw_path(&path, &paint);
    path.close();
    canvas.translate((0, 50));
    canvas.draw_path(&path, &paint);
}

fn draw_example5(canvas: &mut Canvas) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(paint::Style::Stroke);
    paint.set_stroke_width(8.0);
    paint.set_stroke_cap(paint::Cap::Round);
    let mut path = Path::default();
    path.move_to((36, 48));
    path.line_to((36, 48));
    canvas.draw_path(&path, &paint);
    path.reset();
    paint.set_stroke_cap(paint::Cap::Square);
    path.move_to((56, 48));
    path.close();
    canvas.draw_path(&path, &paint);
}
