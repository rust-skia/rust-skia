use skia_safe::skia::{Canvas, Path, scalar, Paint, Color, Rect, PaintStyle, BlendMode, RRect, Font, Typeface, Image, Data};
use crate::artifact;

pub fn draw() {
    artifact::draw_canvas_256("heptagram", draw_heptagram);
    artifact::draw_canvas_256("rotated-rectangle", draw_rotated_rectangle);
    artifact::draw_canvas_256("hello-skia", draw_hello_skia);
}

fn draw_heptagram(canvas: &mut Canvas) {
    const SCALE: scalar = 256.0;
    const R : scalar = 0.45 * SCALE;
    const TAU : scalar = 6.2831853;
    let mut path = Path::default();
    path.move_to((R, 0.0).into());
    for i in 1..7 {
        let theta = 3.0 * (i as scalar) * TAU / 7.0;
        path.line_to((R * scalar::cos(theta), R * scalar::sin(theta)).into());
    }

    path.close();
    let mut p = Paint::default();
    p.set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.translate((0.5 * SCALE, 0.5 * SCALE).into());
    canvas.draw_path(&path, &p);
}

fn draw_rotated_rectangle(canvas: &mut Canvas) {
    canvas.save();
    canvas.translate((128.0, 128.0).into());
    canvas.rotate(45.0, None);
    // TODO: should we add a function Rect::from_point_and_size() ?
    // also this could be ambiguous, Rects can also be described as two Points!
    let rect = Rect::from_point_and_size((-90.5, -90.5).into(), (181.0, 181.0).into());
    let mut paint = Paint::default();
    paint.set_color(Color::BLUE);
    canvas.draw_rect(&rect, &paint);
    canvas.restore();
}

fn draw_hello_skia(canvas: &mut Canvas) {

    let bytes = include_bytes!("color_wheel.png");
    let data = Data::new_copy(bytes);
    let image = Image::from_encoded(&data, None).unwrap();

    canvas.draw_color(Color::WHITE, BlendMode::default());

    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(4.0);
    paint.set_color(Color::RED);

    let mut rect = Rect::from_point_and_size((50.0, 50.0).into(), (40.0, 60.0).into());
    canvas.draw_rect(rect, &paint);

    let oval = RRect::new_oval(rect).offset((40.0, 60.0).into());
    paint.set_color(Color::BLUE);
    canvas.draw_rrect(&oval, &paint);

    paint.set_color(Color::CYAN);
    canvas.draw_circle((180.0, 50.0).into(), 25.0, &paint);

    rect = rect.with_offset((80.0, 0.0).into());
    paint.set_color(Color::YELLOW);
    canvas.draw_round_rect(rect, 10.0, 10.0, &paint);

    let mut path = Path::default();
    path.cubic_to((768.0, 0.0).into(), (-512.0, 256.0).into(), (256.0, 256.0).into());
    paint.set_color(Color::GREEN);
    canvas.draw_path(&path, &paint);

    canvas.draw_image(&image, (128.0, 128.0).into(), Some(&paint));

    let rect2 = Rect::from_point_and_size((0.0, 0.0).into(), (40.0, 60.0).into());
    canvas.draw_image_rect(&image, None, rect2, &paint);

    let paint2 = Paint::default();

    // TODO: support TextBlobs
    // auto text = SkTextBlob::MakeFromString("Hello, Skia!", SkFont(nullptr, 18));
    // canvas.drawTextBlob(text.get(), 50, 25, paint2);

    let font = Font::from_typeface_with_size(&Typeface::default(), 18.0);
    canvas.draw_str("Hello, Skia!", (50.0, 25.0).into(), &font, &paint2);
}
