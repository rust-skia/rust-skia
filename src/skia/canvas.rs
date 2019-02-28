use crate::prelude::*;
use rust_skia::{SkCanvas, C_SkCanvas_destruct};
use super::{Path, Paint, Color, Surface};

pub struct Canvas {
    native: *mut SkCanvas,
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

impl NativeAccess<SkCanvas> for Canvas {
    fn native(&self) -> &SkCanvas {
        unsafe { & *self.native }
    }

    fn native_mut(&mut self) -> &mut SkCanvas {
        unsafe { & mut *self.native }
    }
}

impl Canvas {

    pub(crate) fn from_surface(surface: &mut Surface) -> Canvas {
        Canvas {
            native: unsafe { surface.native_mut().getCanvas() },
            owner: Some(surface.clone())
        }
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
}