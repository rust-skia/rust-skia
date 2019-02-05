use rust_skia::{SkPaint, C_SkPaint_destruct, SkPaint_Style, SkColor};

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

    pub fn set_color(&mut self, color: SkColor) {
        unsafe { self.native.setColor(color) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { self.native.setAntiAlias(anti_alias) }
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { self.native.setStrokeWidth(width) }
    }

    pub fn set_style(&mut self, style: SkPaint_Style) {
        unsafe { self.native.setStyle(style) }
    }
}
