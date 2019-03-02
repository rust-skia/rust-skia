use crate::prelude::*;
use crate::skia::Color;
use rust_skia::{
    SkPaint,
    C_SkPaint_destruct,
    SkPaint_Style};

pub type PaintStyle = EnumHandle<SkPaint_Style>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Style> {
    pub const Stroke: Self = Self(SkPaint_Style::kStroke_Style);
    pub const Fill: Self = Self(SkPaint_Style::kFill_Style);
    pub const StrokeAndFill: Self = Self(SkPaint_Style::kStrokeAndFill_Style);
}

pub type Paint = Handle<SkPaint>;

impl NativeDrop for SkPaint {
    fn drop(&mut self) {
        unsafe { C_SkPaint_destruct(self) }
    }
}

impl Default for Handle<SkPaint> {
    fn default() -> Self {
        Paint::from_native(unsafe { SkPaint::new() })
    }
}

impl Handle<SkPaint> {

    pub fn set_color(&mut self, color: Color) {
        unsafe { self.native_mut().setColor(color.into_native()) }
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

