// Author: Alberto González Palomo https://sentido-labs.com
// ©2019 Alberto González Palomo https://sentido-labs.com
// Released under the MIT license: https://opensource.org/licenses/MIT
#![allow(unknown_lints)]
#![allow(clippy::unusual_byte_groupings)]
use skia_safe::{
    gradient_shader, Color, Matrix, Paint, PaintJoin, PaintStyle, Path, Point, TileMode,
};
use std::cmp::min;

const PI: f32 = std::f32::consts::PI;
const DEGREES_IN_RADIANS: f32 = PI / 180.0;
const PEN_SIZE: f32 = 1.0;

fn point_in_circle(center: (f32, f32), radius: f32, radians: f32) -> (f32, f32) {
    (
        center.0 + radius * radians.cos(),
        center.1 - radius * radians.sin(),
    )
}

pub fn render_frame(
    frame: usize,
    fps: usize,
    bpm: usize,
    canvas: &skia_safe::canvas::Canvas,
) -> usize {
    let step = 12.0 * bpm as f32 / 60.0 / fps as f32;
    let frame_count = (360.0 / step) as usize;

    let size = {
        let dim = canvas.image_info().dimensions();
        min(dim.width, dim.height)
    };

    let center = (size / 2, size / 2);
    let chain_ring_radius = size / 2 * 100 / 100;
    let triangle_radius = size / 2 * 53 / 100;

    let rotation = frame as f32 * step;
    chain_ring(canvas, center, chain_ring_radius, rotation, 32);

    let triangle_rotation = 60.0 + rotation;
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        Some(0),
        Color::GREEN,
        true,
    );
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        Some(1),
        Color::BLUE,
        true,
    );
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        Some(2),
        Color::RED,
        true,
    );
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        Some(0),
        Color::YELLOW,
        false,
    );
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        Some(1),
        Color::CYAN,
        false,
    );
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        Some(2),
        Color::MAGENTA,
        false,
    );

    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        None,
        Color::from(0x77_222222),
        true,
    );
    triangle(
        canvas,
        center,
        triangle_radius,
        triangle_rotation,
        None,
        Color::from(0x77_222222),
        false,
    );

    frame_count - (frame + 1)
}

fn chain_ring(
    canvas: &skia_safe::canvas::Canvas,
    center: (i32, i32),
    radius: i32,
    rotation: f32,
    teeth_count: i32,
) {
    canvas.save();
    canvas.translate(Point::from(center));
    canvas.save();
    canvas.rotate(rotation, None);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0));

    let center = (0, 0);
    let c = (center.0 as f32, center.1 as f32);
    let outer_radius = radius as f32;
    let inner_radius = outer_radius * 0.73;
    let ridge_radius = outer_radius * 0.85;
    let teeth_length = (outer_radius - ridge_radius) * 0.8;

    let delta = 2.0 * PI / (teeth_count as f32);
    let teeth_bottom_gap = 0.2 * delta;

    let mut alpha = PI / 2.0;
    let mut path = Path::new();
    for i in 0..teeth_count {
        let mut a = alpha - delta / 2.0 + teeth_bottom_gap / 2.0;
        let v = point_in_circle(c, outer_radius - teeth_length, a);
        if i == 0 {
            path.move_to(v);
        } else {
            path.line_to(v);
        }
        let middle = a + (delta - teeth_bottom_gap) / 2.0;
        a += delta - teeth_bottom_gap;
        path.cubic_to(
            point_in_circle(c, outer_radius * 1.035, middle),
            point_in_circle(c, outer_radius * 1.035, middle),
            point_in_circle(c, outer_radius - teeth_length, a),
        );
        a += teeth_bottom_gap;
        path.line_to(point_in_circle(c, outer_radius - teeth_length, a));

        alpha += delta;
    }
    path.close();

    let delta = -2.0 * PI / 5.0;
    let teeth_bottom_gap = 0.70 * delta;

    alpha = PI / 2.0;
    for i in 0..5 {
        let mut a = alpha - delta / 2.0 + teeth_bottom_gap / 2.0;
        let v = point_in_circle(c, inner_radius, a);
        if i == 0 {
            path.move_to(v);
        } else {
            path.line_to(v);
        }
        let middle = a + (delta - teeth_bottom_gap) / 2.0;
        a += delta - teeth_bottom_gap;
        path.cubic_to(
            point_in_circle(c, inner_radius - teeth_length * 1.33, middle),
            point_in_circle(c, inner_radius - teeth_length * 1.33, middle),
            point_in_circle(c, inner_radius, a),
        );
        a += teeth_bottom_gap;
        path.cubic_to(
            point_in_circle(c, inner_radius * 1.05, a - teeth_bottom_gap * 0.67),
            point_in_circle(c, inner_radius * 1.05, a - teeth_bottom_gap * 0.34),
            point_in_circle(c, inner_radius, a),
        );

        alpha += delta;
    }
    path.close();

    let bolt_radius = inner_radius * 0.81 * (delta - teeth_bottom_gap) / delta / PI;
    alpha = PI / 2.0;
    for _i in 0..5 {
        let c = point_in_circle(c, inner_radius + bolt_radius * 0.33, alpha);
        let mut a = alpha;
        for j in 0..5 {
            if j == 0 {
                path.move_to(point_in_circle(c, bolt_radius, a));
            } else {
                path.cubic_to(
                    point_in_circle(c, bolt_radius * 1.14, a + PI / 3.0),
                    point_in_circle(c, bolt_radius * 1.14, a + PI / 6.0),
                    point_in_circle(c, bolt_radius, a),
                );
            }
            a -= PI / 2.0;
        }
        path.close();

        alpha += delta;
    }

    paint.set_style(PaintStyle::Fill);
    // Rust shade, from steel gray to rust color:
    paint.set_shader(gradient_shader::radial(
        (0.0, 0.04 * ridge_radius),
        ridge_radius,
        [Color::from(0xff_555555), Color::from(0xff_7b492d)].as_ref(),
        [0.8, 1.0].as_ref(),
        TileMode::Clamp,
        None,
        None,
    ));
    canvas.draw_path(&path, &paint);
    paint.set_shader(None); // Remove gradient.
    paint.set_style(PaintStyle::Stroke);
    paint.set_color(0xff_592e1f);
    canvas.draw_path(&path, &paint);

    canvas.restore();

    // Ridge around the chain ring, under the gear teeth:
    gradient(
        &mut paint,
        (0.0, -ridge_radius),
        (2.0 * ridge_radius, 2.0 * ridge_radius),
        (Color::from(0xff_592e1f), Color::from(0xff_885543)),
    );
    canvas.draw_circle(center, ridge_radius, &paint);

    canvas.restore();
}

#[allow(clippy::many_single_char_names)]
fn triangle(
    canvas: &skia_safe::canvas::Canvas,
    center: (i32, i32),
    radius: i32,
    degrees: f32,
    vertex: Option<i32>,
    color: Color,
    wankel: bool,
) {
    let c = (center.0 as f32, center.1 as f32);
    let r = radius as f32;
    let b = r * 0.9;
    let delta = 120.0 * DEGREES_IN_RADIANS;
    let side = r / ((PI - delta) / 2.0).cos() * 2.0;

    let mut alpha = degrees * DEGREES_IN_RADIANS;
    let mut path = Path::new();
    let mut paint = Paint::default();
    match vertex {
        Some(index) => {
            let a = (degrees + (120 * index) as f32) * DEGREES_IN_RADIANS;
            let center = point_in_circle(c, r, a);
            let radii = match index {
                0 | 2 => {
                    if wankel {
                        (0.36 * side, 0.404 * side)
                    } else {
                        (0.30 * side, 0.60 * side)
                    }
                }
                1 => {
                    if wankel {
                        (0.404 * side, 0.50 * side)
                    } else {
                        (0.420 * side, 0.50 * side)
                    }
                }
                i => panic!("Invalid vertex index {i} for triangle."),
            };
            gradient(&mut paint, center, radii, (color, Color::from(0x00_0000ff)))
        }
        None => {
            paint.set_anti_alias(true);
            paint.set_stroke_width(
                PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0),
            );
            paint.set_style(PaintStyle::Stroke);
            paint.set_stroke_join(PaintJoin::Bevel);
            // Highlight reflection on the top triangle edge:
            paint.set_shader(gradient_shader::radial(
                (c.0, c.1 - 0.5 * r),
                0.5 * r,
                [Color::from(0xff_ffffff), color].as_ref(),
                None,
                TileMode::Clamp,
                None,
                None,
            ));
        }
    };
    for i in 0..4 {
        let v = point_in_circle(c, r, alpha);
        if i == 0 {
            path.move_to(v);
        } else if wankel {
            path.cubic_to(
                point_in_circle(c, b, alpha - 2.0 * delta / 3.0),
                point_in_circle(c, b, alpha - delta / 3.0),
                v,
            );
        } else {
            path.line_to(v);
        }
        alpha += delta;
    }
    path.close();
    canvas.draw_path(&path, &paint);
}

fn gradient(paint: &mut Paint, center: (f32, f32), radii: (f32, f32), colors: (Color, Color)) {
    let mut matrix = Matrix::scale((1.0, radii.1 / radii.0));
    matrix.post_translate((center.0, center.1));
    #[allow(clippy::tuple_array_conversions)]
    paint.set_shader(gradient_shader::radial(
        (0.0, 0.0),
        radii.0,
        [colors.0, colors.1].as_ref(),
        None,
        TileMode::Clamp,
        None,
        &matrix,
    ));
}
