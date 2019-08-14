use crate::prelude::*;
use skia_bindings::{
    C_SkPixelRef_notifyAddedToCache, SkPixelRef, SkPixelRef_Mutability, SkRefCntBase,
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
        self.native().fWidth
    }

    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    pub unsafe fn pixels(&self) -> *mut c_void {
        self.native().fPixels
    }

    pub fn row_bytes(&self) -> usize {
        self.native().fRowBytes
    }

    pub fn generation_id(&self) -> u32 {
        unsafe { self.native().getGenerationID() }
    }

    pub fn notify_pixels_changed(&mut self) {
        unsafe { self.native_mut().notifyPixelsChanged() }
    }

    pub fn is_immutable(&self) -> bool {
        self.native().fMutability() != SkPixelRef_Mutability::kMutable
    }

    pub fn set_immutable(&mut self) {
        unsafe { self.native_mut().setImmutable() }
    }

    // TODO addGenIDChangeListener()

    pub fn notify_added_to_cache(&mut self) {
        unsafe { C_SkPixelRef_notifyAddedToCache(self.native_mut()) }
    }
}
