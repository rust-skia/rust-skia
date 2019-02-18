use rust_skia::{SkCanvas, C_SkCanvas_destruct};
use super::{Path, Paint, Color, Surface};

pub struct Canvas {
    pub(crate) native: *mut SkCanvas,
    pub(crate) owner: Option<Surface>
}

impl Drop for Canvas {
    fn drop (&mut self) {
        match &self.owner {
            Some(_) => {},
            None => { unsafe { C_SkCanvas_destruct(self.native) } }
        }
    }
}

impl Canvas {

    pub fn clear(&mut self, color: Color) {
        unsafe { (*self.native).clear(color.0) }
    }

    pub fn save(&mut self) -> i32 {
        unsafe { (*self.native).save() }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe { (*self.native).translate(dx, dy) }
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe { (*self.native).scale(sx, sy) }
    }

    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe { (*self.native).drawPath(&path.native, &paint.native) }
    }

    pub fn flush(&mut self) {
        unsafe { (*self.native).flush(); }
    }
}