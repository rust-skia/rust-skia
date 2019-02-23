use crate::prelude::*;
use rust_skia::{SkPaint, C_SkPaint_destruct, SkPaint_Style};
use super::Color;

pub type Paint = Handle<SkPaint>;

impl NativeDrop for SkPaint {
    fn drop(&mut self) {
        unsafe { C_SkPaint_destruct(self) }
    }
}

impl Paint {
    pub fn new() -> Paint {
        Paint::from_native(unsafe { SkPaint::new() })
    }

    pub fn set_color(&mut self, color: Color) {
        unsafe { self.native_mut().setColor(color.0) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { self.native_mut().setAntiAlias(anti_alias) }
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { self.native_mut().setStrokeWidth(width) }
    }

    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe { self.native_mut().setStyle(style.0) }
    }
}

#[derive(Copy, Clone)]
pub struct PaintStyle(pub(crate) SkPaint_Style);

#[allow(non_upper_case_globals)]
impl PaintStyle {
    pub const Stroke: PaintStyle = PaintStyle(SkPaint_Style::kStroke_Style);
    pub const Fill: PaintStyle = PaintStyle(SkPaint_Style::kFill_Style);
    pub const StrokeAndFill: PaintStyle = PaintStyle(SkPaint_Style::kStrokeAndFill_Style);
}
