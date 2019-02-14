use rust_skia::{SkPaint, C_SkPaint_destruct, SkPaint_Style};
use super::Color;

pub struct Paint {
    pub(crate) native: SkPaint
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe { C_SkPaint_destruct(&self.native) }
    }
}

impl Paint {
    pub fn new() -> Paint {
        Paint { native: unsafe { SkPaint::new() }}
    }

    pub fn set_color(&mut self, color: Color) {
        unsafe { self.native.setColor(color.0) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { self.native.setAntiAlias(anti_alias) }
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { self.native.setStrokeWidth(width) }
    }

    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe { self.native.setStyle(style.0) }
    }
}

pub struct PaintStyle(SkPaint_Style);

#[allow(non_upper_case_globals)]
impl PaintStyle {
    pub const Stroke: PaintStyle = PaintStyle(SkPaint_Style::kStroke_Style);
    pub const Fill: PaintStyle = PaintStyle(SkPaint_Style::kFill_Style);
    pub const StrokeAndFill: PaintStyle = PaintStyle(SkPaint_Style::kStrokeAndFill_Style);
}
