use std::marker::PhantomData;
use rust_skia::{SkCanvas};
use super::{Path, Paint, Color};

#[derive(Debug)]
pub struct Canvas<'a> {
    pub(crate) native: *mut SkCanvas,
    pub(crate) phantom: PhantomData<&'a SkCanvas>
}

impl<'a> Canvas<'a> {

    #[inline]
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