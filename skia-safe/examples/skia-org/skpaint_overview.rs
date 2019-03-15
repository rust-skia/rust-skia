use std::path::PathBuf;
use crate::artifact;
use skia_safe::skia::{Canvas, Paint, Color, Rect, PaintStyle, BlendMode, Font, Typeface, TextBlob, Point, ShaderTileMode, AutoCanvasRestore, scalar};
use skia_safe::effects::GradientShader;

pub fn draw(path: &PathBuf) {
    let path = path.join("SkPaint-Overview");

    artifact::draw_canvas_256(&path, "01-three-paints", draw_three_paints);
    artifact::draw_canvas_256(&path, "02-fill-and-stroke", draw_fill_and_stroke);
    artifact::draw_canvas_256(&path, "03-fill-and-stroke", draw_gradient);
    artifact::draw_canvas((576, 640), &path, "04-transfer_modes", draw_transfer_modes);
}

fn draw_three_paints(canvas: &mut Canvas) {

    let (mut paint1, mut paint2, mut paint3) = (Paint::default(), Paint::default(), Paint::default());

    paint1.set_anti_alias(true);
    paint1.set_color(Color::from_rgb(255, 0, 0));
    paint1.set_style(PaintStyle::Fill);

    paint2.set_anti_alias(true);
    paint2.set_color(Color::from_rgb(0, 136, 0));
    paint2.set_style(PaintStyle::Stroke);
    paint2.set_stroke_width(3.0);

    paint3.set_anti_alias(true);
    paint3.set_color(Color::from_rgb(136, 136, 136));

    let blob1 =
        TextBlob::from_str("Skia!", &Font::from_typeface_with_size_scale_and_skew(&Typeface::default(), 64.0, 1.0, 0.0));
    let blob2 =
        TextBlob::from_str("Skia!", &Font::from_typeface_with_size_scale_and_skew(&Typeface::default(), 64.0, 1.5, 0.0));

    canvas.clear(Color::WHITE);
    canvas.draw_text_blob(&blob1, (20.0, 64.0),  &paint1);
    canvas.draw_text_blob(&blob1, (20.0, 144.0), &paint2);
    canvas.draw_text_blob(&blob2, (20.0, 224.0), &paint3);
}

fn draw_fill_and_stroke(canvas: &mut Canvas) {
    let mut fill_paint = Paint::default();
    let mut stroke_paint = Paint::default();
    stroke_paint.set_style(PaintStyle::Stroke);
    stroke_paint.set_stroke_width(3.0);

    canvas.draw_rect(Rect::from_point_and_size((10, 10), (60, 20)), &fill_paint);
    canvas.draw_rect(Rect::from_point_and_size((80, 10), (60, 20)), &stroke_paint);

    stroke_paint.set_stroke_width(5.0);
    canvas.draw_oval(Rect::from_point_and_size((150, 10), (60, 20)), &stroke_paint);

    let blob =
        TextBlob::from_str("SKIA", &Font::from_typeface_with_size(&Typeface::default(), 80.0));

    fill_paint.set_color(Color::from_argb(0xFF, 0xFF, 0x00, 0x00));
    canvas.draw_text_blob(&blob, (20, 120), &fill_paint);

    fill_paint.set_color(Color::from_argb(0xFF, 0x00, 0x00, 0xFF));
    canvas.draw_text_blob(&blob, (20, 220), &fill_paint);
}

fn draw_gradient(canvas: &mut Canvas) {
    let points : (Point, Point) = (
        (0.0, 0.0).into(),
        (256.0, 256.0).into()
    );
    let colors = [Color::BLUE, Color::YELLOW];
    let mut paint = Paint::default();

    paint.set_shader(GradientShader::linear(
        points, colors.as_ref(), None,
        ShaderTileMode::Clamp, Default::default(), None).as_ref());
    canvas.draw_paint(&paint);
}

fn draw_transfer_modes(canvas: &mut Canvas) {

    fn draw_str(c: &mut Canvas, t: &str, x: scalar, y: scalar,
        f: &Font, p: &Paint) {
        c.draw_text_blob(&TextBlob::from_str(t, &f), (x, y), p);
    }

    let modes = [
        BlendMode::Clear,
        BlendMode::Src,
        BlendMode::Dst,
        BlendMode::SrcOver,
        BlendMode::DstOver,
        BlendMode::SrcIn,
        BlendMode::DstIn,
        BlendMode::SrcOut,
        BlendMode::DstOut,
        BlendMode::SrcATop,
        BlendMode::DstATop,
        BlendMode::Xor,
        BlendMode::Plus,
        BlendMode::Modulate,
        BlendMode::Screen,
        BlendMode::Overlay,
        BlendMode::Darken,
        BlendMode::Lighten,
        BlendMode::ColorDodge,
        BlendMode::ColorBurn,
        BlendMode::HardLight,
        BlendMode::SoftLight,
        BlendMode::Difference,
        BlendMode::Exclusion,
        BlendMode::Multiply,
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity
    ];
    let rect = Rect::from_size((64.0, 64.0));
    let (mut stroke, mut src, mut dst) = (Paint::default(), Paint::default(), Paint::default());
    stroke.set_style(PaintStyle::Stroke);
    let font = Font::from_typeface_with_size(&Typeface::default(), 24.0);
    let src_points : (Point, Point) = (
        (0.0, 0.0).into(),
        (64.0, 0.0).into()
    );
    let src_colors = [
        Color::MAGENTA & 0x00FFFFFF,
        Color::MAGENTA ];
    src.set_shader(GradientShader::linear(
        src_points, src_colors.as_ref(), None,
        ShaderTileMode::Clamp, Default::default(), None).as_ref());

    let dst_points : (Point, Point) = (
        (0.0, 0.0).into(),
        (0.0, 64.0).into()
    );
    let dst_colors = [
        Color::CYAN & 0x00FFFFFF,
        Color::CYAN
    ];
    dst.set_shader(GradientShader::linear(
        dst_points, dst_colors.as_ref(), None,
        ShaderTileMode::Clamp, Default::default(), None).as_ref());
    canvas.clear(Color::WHITE);
    let n = modes.len();
    let k = (n - 1) / 3 + 1;
    assert_eq!(k * 64, 640);  // tall enough
    for i in 0..n {
        let mut canvas = AutoCanvasRestore::guard(canvas, true);
        canvas.translate((192.0 * (i / k) as scalar, 64.0 * (i % k) as scalar));
        let desc = modes[i].name();
        draw_str(&mut canvas, desc, 68.0, 30.0, &font, &Paint::default());
        canvas.clip_rect(Rect::from_size((64.0, 64.0)), Default::default());
        canvas.draw_color(Color::LIGHT_GRAY, BlendMode::default());
        canvas.save_layer(&Default::default());
        canvas.clear(Color::TRANSPARENT);
        canvas.draw_paint(&dst);
        src.set_blend_mode(modes[i]);
        canvas.draw_paint(&src);
        canvas.draw_rect(rect, &stroke);
    }
}
