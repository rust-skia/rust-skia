use std::mem;
use crate::prelude::*;
use crate::skia::{Path, Paint, Color};
use std::ops::{Deref, DerefMut};
use rust_skia::{
    SkCanvas_SkCanvas_destructor,
    SkCanvas,
    C_SkCanvas_destruct
};

// Warning: do never access SkCanvas fields from Rust, bindgen generates a wrong layout
// as of version 0.47.3.

/// The canvas type that is returned when it is managed by another instance,
/// like Surface, for example. For these cases, the Canvas' reference that is
/// returned is bound to the lifetime of the owner.

#[repr(transparent)]
pub struct Canvas(SkCanvas);

impl NativeAccess<SkCanvas> for Canvas {
    fn native(&self) -> &SkCanvas {
        &self.0
    }

    fn native_mut(&mut self) -> &mut SkCanvas {
        &mut self.0
    }
}

/// This is the type representing a canvas that is owned and destructed
/// when it goes out of scope.
pub struct OwnedCanvas(*mut Canvas);

impl Deref for OwnedCanvas {
    type Target = Canvas;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for OwnedCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl Drop for OwnedCanvas {
    fn drop(&mut self) {
        unsafe { C_SkCanvas_destruct(self.native()) }
    }
}

impl Canvas {

    pub(crate) fn borrow_from_native(native: &mut SkCanvas) -> &mut Self {
        unsafe { mem::transmute::<&mut SkCanvas, &mut Self>(native) }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe { self.native_mut().clear(color.into_native()) }
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