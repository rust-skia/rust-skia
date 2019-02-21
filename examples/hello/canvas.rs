use core::mem;
use skia_safe::skia;
use skia_safe::skia::Color;

pub struct Canvas {
    surface: skia::Surface,
    path: skia::Path,
    paint: skia::Paint,
}

impl Canvas {

    pub fn new(width: u32, height: u32) -> Canvas {
        let surface =
            skia::Surface::new_raster_n32_premul(width, height)
                .expect("no surface!");
        let path = skia::Path::new();
        let mut paint = skia::Paint::new();
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        Canvas {
            surface,
            path,
            paint,
        }
    }

    #[inline]
    pub fn save(&mut self) {
        self.canvas().save();
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate(dx, dy);
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale(sx, sy);
    }

    #[inline]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.begin_path();
        self.path.move_to(x, y);
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(x, y);
    }

    #[inline]
    pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to(cpx, cpy, x, y);
    }

    #[inline]
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.cubic_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    #[inline]
    pub fn close_path(&mut self) {
        self.path.close();
    }

    #[inline]
    pub fn begin_path(&mut self) {
        let new_path = skia::Path::new();
        self.canvas().draw_path(&self.path, &self.paint);
        mem::replace(&mut self.path, new_path);
    }

    #[inline]
    pub fn stroke(&mut self) {
        self.paint.set_style(skia::PaintStyle::Stroke);
        self.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn fill(&mut self) {
        self.paint.set_style(skia::PaintStyle::Fill);
        self.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    #[inline]
    pub fn data(&mut self) -> skia::Data {
        let image = self.surface.make_image_snapshot();
        image.encode_to_data().unwrap()
    }

    #[inline]
    fn canvas(&self) -> skia::Canvas {
        self.surface.canvas()
    }
}
