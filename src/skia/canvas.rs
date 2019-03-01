use crate::prelude::*;
use std::mem;
use rust_skia::SkCanvas;
use super::{Path, Paint, Color, Surface};

// Note: References to a canvas are exposed bound to the lifetime of
// the owning instance.

#[repr(transparent)]
pub struct Canvas(SkCanvas);

impl Canvas {

    pub(crate) fn borrow_from_surface(surface: &mut Surface) -> &mut Self {
        let sk_canvas_ref = unsafe { &mut *surface.native_mut().getCanvas() };
        unsafe { mem::transmute::<&mut SkCanvas, &mut Self>(sk_canvas_ref) }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe { self.native_mut().clear(color.0) }
    }

    pub fn save(&mut self) -> i32 {
        unsafe { self.native_mut().save() }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe { self.native_mut().translate(dx, dy) }
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe { self.native_mut().scale(sx, sy) }
    }

    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe { self.native_mut().drawPath(path.native(), paint.native()) }
    }

    pub fn flush(&mut self) {
        unsafe { self.native_mut().flush(); }
    }

    pub(crate) fn native_mut(&mut self) -> &mut SkCanvas {
        &mut self.0
    }
}