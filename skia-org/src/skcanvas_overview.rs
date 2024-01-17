use std::path;

use skia_safe::{
    paint, scalar, BlendMode, Canvas, Color, Font, Paint, Path, RRect, Rect, TextBlob,
};

use crate::{helper::default_typeface, resources, DrawingDriver};

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = path.join("SkCanvas-Overview");
    driver.draw_image_256(&path, "heptagram", draw_heptagram);
    driver.draw_image_256(&path, "rotated-rectangle", draw_rotated_rectangle);
    driver.draw_image_256(&path, "hello-skia", draw_hello_skia);
}

fn draw_heptagram(canvas: &Canvas) {
    const SCALE: scalar = 256.0;
    const R: scalar = 0.45 * SCALE;
    #[allow(clippy::excessive_precision)]
    const TAU: scalar = std::f32::consts::TAU;
    let mut path = Path::default();
    path.move_to((R, 0.0));
    for i in 1..7 {
        let theta = 3.0 * (i as scalar) * TAU / 7.0;
        path.line_to((R * scalar::cos(theta), R * scalar::sin(theta)));
    }

    path.close();
    let mut p = Paint::default();
    p.set_anti_alias(true);
    canvas
        .clear(Color::WHITE)
        .translate((0.5 * SCALE, 0.5 * SCALE))
        .draw_path(&path, &p);
}

fn draw_rotated_rectangle(canvas: &Canvas) {
    canvas.save();
    canvas.translate((128.0, 128.0)).rotate(45.0, None);
    let rect = Rect::from_point_and_size((-90.5, -90.5), (181.0, 181.0));
    let mut paint = Paint::default();
    paint.set_color(Color::BLUE);
    canvas.draw_rect(rect, &paint);
    canvas.restore();
}

fn draw_hello_skia(canvas: &Canvas) {
    let image = resources::color_wheel();

    canvas.draw_color(Color::WHITE, BlendMode::default());

    let mut paint = Paint::default();
    paint
        .set_style(paint::Style::Stroke)
        .set_stroke_width(4.0)
        .set_color(Color::RED);

    let mut rect = Rect::from_point_and_size((50.0, 50.0), (40.0, 60.0));
    canvas.draw_rect(rect, &paint);

    let oval = RRect::new_oval(rect).with_offset((40.0, 60.0));
    paint.set_color(Color::BLUE);
    canvas.draw_rrect(oval, &paint);

    paint.set_color(Color::CYAN);
    canvas.draw_circle((180.0, 50.0), 25.0, &paint);

    rect = rect.with_offset((80.0, 0.0));
    paint.set_color(Color::YELLOW);
    canvas.draw_round_rect(rect, 10.0, 10.0, &paint);

    let mut path = Path::default();
    path.cubic_to((768.0, 0.0), (-512.0, 256.0), (256.0, 256.0));
    paint.set_color(Color::GREEN);
    canvas.draw_path(&path, &paint);

    canvas.draw_image(&image, (128.0, 128.0), Some(&paint));

    let rect2 = Rect::from_point_and_size((0.0, 0.0), (40.0, 60.0));
    canvas.draw_image_rect(&image, None, rect2, &paint);

    let paint2 = Paint::default();

    let text = TextBlob::from_str(
        "Hello, Skia!",
        &Font::from_typeface(default_typeface(), 18.0),
    )
    .unwrap();
    canvas.draw_text_blob(&text, (50, 25), &paint2);
}
