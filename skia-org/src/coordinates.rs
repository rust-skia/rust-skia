//! Ports the examples of Skia's page about coordinate spaces
//! (`site/docs/user/coordinates.md` in the Skia repository).
//!
//! The examples draw a 50 × 50 square, mostly filled with a horizontal green-to-blue gradient, and
//! show how the canvas transform and a shader's local matrix move the square and its gradient. The
//! page embeds the fiddles by hash, so the code was taken from fiddle.skia.org and translated from
//! Skia's pre-2020 gradient API.

use std::path;

use skia_safe::{Canvas, Color, Color4f, Matrix, Paint, Point, Rect, Shader, TileMode, gradient};

use crate::DrawingDriver;

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = &path.join("Coordinate-Spaces");

    driver.draw_image(SIZE, path, "01-local-coordinates", draw_local_coordinates);
    driver.draw_image(SIZE, path, "02-gradient", draw_gradient);
    driver.draw_image(SIZE, path, "03-moved-geometry", draw_moved_geometry);
    driver.draw_image(SIZE, path, "04-translated-canvas", draw_translated_canvas);
    driver.draw_image(SIZE, path, "05-rotated-canvas", draw_rotated_canvas);
    driver.draw_image(SIZE, path, "06-rotated-shader", draw_rotated_shader);
}

/// The size of every example image.
const SIZE: (i32, i32) = (200, 200);

/// A square with a solid color, placed in device coordinates.
fn draw_local_coordinates(canvas: &Canvas) {
    canvas.clear(Color::GRAY);
    let mut paint = Paint::default();
    paint.set_color(Color::GREEN);
    canvas.draw_rect(Rect::from_xywh(100.0, 50.0, 50.0, 50.0), &paint);
}

/// The gradient spans the square at the origin.
fn draw_gradient(canvas: &Canvas) {
    canvas.clear(Color::GRAY);
    canvas.draw_rect(square(), &gradient_paint(None));
}

/// The square moves, but the gradient stays at the origin, so the square shows only its end color.
fn draw_moved_geometry(canvas: &Canvas) {
    canvas.clear(Color::GRAY);
    canvas.draw_rect(
        Rect::from_xywh(100.0, 50.0, 50.0, 50.0),
        &gradient_paint(None),
    );
}

/// Translating the canvas moves the local coordinate space, so the gradient moves with the square.
fn draw_translated_canvas(canvas: &Canvas) {
    canvas.clear(Color::GRAY);
    canvas.translate((100.0, 50.0));
    canvas.draw_rect(square(), &gradient_paint(None));
}

/// Rotating the canvas rotates both the square and its gradient.
fn draw_rotated_canvas(canvas: &Canvas) {
    canvas.clear(Color::GRAY);
    canvas.translate((50.0, 50.0));
    canvas.concat(&rotation());
    canvas.draw_rect(square(), &gradient_paint(None));
}

/// The shader's local matrix rotates only the gradient, not the square.
fn draw_rotated_shader(canvas: &Canvas) {
    canvas.clear(Color::GRAY);
    canvas.translate((50.0, 50.0));
    canvas.draw_rect(square(), &gradient_paint(Some(&rotation())));
}

/// The square at the origin of the local coordinate space.
fn square() -> Rect {
    Rect::from_wh(50.0, 50.0)
}

/// A rotation by 45 degrees around the center of [`square`].
fn rotation() -> Matrix {
    Matrix::rotate_deg_pivot(45.0, Point::new(25.0, 25.0))
}

/// A paint whose shader is the green-to-blue gradient from x = 0 to x = 50, transformed by
/// `local_matrix` when it is given.
fn gradient_paint(local_matrix: Option<&Matrix>) -> Paint {
    let mut paint = Paint::default();
    paint.set_shader(linear_gradient(local_matrix));
    paint
}

/// The green-to-blue gradient from x = 0 to x = 50, transformed by `local_matrix` when it is given.
fn linear_gradient(local_matrix: Option<&Matrix>) -> Option<Shader> {
    let points: (Point, Point) = ((0.0, 0.0).into(), (50.0, 0.0).into());
    let colors: [Color4f; 2] = [Color::GREEN.into(), Color::BLUE.into()];
    let colors = gradient::Colors::new_evenly_spaced(&colors, TileMode::Clamp, None);
    let gradient = gradient::Gradient::new(colors, gradient::Interpolation::default());
    gradient::shaders::linear_gradient(points, &gradient, local_matrix)
}
