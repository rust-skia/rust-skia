use core::mem;
use skia_safe::skia;
use skia_safe::bindings::*;

pub struct Canvas {
    surface: skia::Surface,
    path: skia::Path,
    rect: skia::Rect,
    paint: skia::Paint,
    width: i32,
    height: i32,
}

#[inline]
pub fn set_a_rgb(a: U8CPU, r: U8CPU, g: U8CPU, b: U8CPU) -> SkColor {
    debug_assert!(a <= 255);
    debug_assert!(r <= 255);
    debug_assert!(g <= 255);
    debug_assert!(b <= 255);
    (a << 24) | (r << 16) | (g << 8) | (b << 0)
}

impl Canvas {

    pub fn new(width: i32, height: i32) -> Canvas {
        let surface =
            skia::Surface::new_raster_n32_premul(width, height)
                .expect("no surface!");
        let path = skia::Path::new();
        let rect = skia::Rect::new_iwh(width, height);
        let mut paint = skia::Paint::new();
        paint.set_color(SK_ColorBLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(SK_ColorWHITE);
        Canvas {
            surface,
            path,
            rect,
            paint,
            width,
            height,
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
        self.paint.set_style(SkPaint_Style_kStroke_Style);
        self.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn fill(&mut self, fill_rule: Option<FillRule>) {
        let rule = fill_rule.unwrap_or(FillRule::default());
        self.paint.set_style(SkPaint_Style_kFill_Style);
        // self.sk_path.setFillType(rule.to_skia_type());
        self.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    #[inline]
    pub fn data(&mut self) -> skia::Data {
        let image = self.surface.make_image_snapshot();
        image.encode_to_data()
    }

    #[inline]
    fn canvas(&self) -> skia::Canvas {
        self.surface.canvas()
    }
}

pub trait ToSkiaType {
    type SkType;

    fn to_skia_type(&self) -> Self::SkType;
}

#[derive(Debug)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

impl ToSkiaType for FillRule {
    type SkType = SkPath_FillType;

    #[inline]
    fn to_skia_type(&self) -> SkPath_FillType {
        match self {
            FillRule::Nonzero => SkPath_FillType_kWinding_FillType,
            FillRule::Evenodd => SkPath_FillType_kEvenOdd_FillType,
        }
    }
}

impl Default for FillRule {
    fn default() -> Self {
        FillRule::Nonzero
    }
}
