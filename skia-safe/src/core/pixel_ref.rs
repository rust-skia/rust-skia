use crate::prelude::*;
use skia_bindings::{
    C_SkPixelRef_height, C_SkPixelRef_isImmutable, C_SkPixelRef_notifyAddedToCache,
    C_SkPixelRef_pixels, C_SkPixelRef_rowBytes, C_SkPixelRef_width, SkPixelRef, SkRefCntBase,
};
use std::os::raw::c_void;

pub type PixelRef = RCHandle<SkPixelRef>;
unsafe impl Sync for RCHandle<SkPixelRef> {}

impl NativeRefCountedBase for SkPixelRef {
    type Base = SkRefCntBase;
}

impl RCHandle<SkPixelRef> {
    // TODO: wrap constructor with pixels borrowed.

    pub fn width(&self) -> i32 {
        unsafe { C_SkPixelRef_width(self.native()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { C_SkPixelRef_height(self.native()) }
    }

    pub unsafe fn pixels(&self) -> *mut c_void {
        C_SkPixelRef_pixels(self.native())
    }

    pub fn row_bytes(&self) -> usize {
        unsafe { C_SkPixelRef_rowBytes(self.native()) }
    }

    pub fn generation_id(&self) -> u32 {
        unsafe { self.native().getGenerationID() }
    }

    pub fn notify_pixels_changed(&mut self) {
        unsafe { self.native_mut().notifyPixelsChanged() }
    }

    pub fn is_immutable(&self) -> bool {
        unsafe { C_SkPixelRef_isImmutable(self.native()) }
    }

    pub fn set_immutable(&mut self) {
        unsafe { self.native_mut().setImmutable() }
    }

    // TODO addGenIDChangeListener()

    pub fn notify_added_to_cache(&mut self) {
        unsafe { C_SkPixelRef_notifyAddedToCache(self.native_mut()) }
    }
}
