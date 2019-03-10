use skia_safe::skia::{Canvas, Path, scalar, Paint, Color};
use crate::artifact;

pub fn draw() {
    artifact::draw_canvas_256("heptagram", draw_heptagram);
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
