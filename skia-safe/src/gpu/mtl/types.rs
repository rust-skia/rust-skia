use crate::prelude;
use crate::prelude::{NativeAccess, NativeDrop, NativePartialEq};
use skia_bindings as sb;
use skia_bindings::GrMtlTextureInfo;
use std::{ffi, ptr};

pub use skia_bindings::GrMTLHandle as Handle;
pub use skia_bindings::GrMTLPixelFormat as PixelFormat;

pub type TextureInfo = prelude::Handle<GrMtlTextureInfo>;
unsafe impl Send for TextureInfo {}
unsafe impl Sync for TextureInfo {}

impl NativeDrop for GrMtlTextureInfo {
    fn drop(&mut self) {
        unsafe { sb::C_GrMtlTextureInfo_Destruct(self) }
    }
}

impl NativePartialEq for GrMtlTextureInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_GrMtlTextureInfo_Equals(self, other) }
    }
}

impl Default for TextureInfo {
    fn default() -> Self {
        unsafe { Self::new(ptr::null()) }
    }
}

impl prelude::Handle<GrMtlTextureInfo> {
    /// #Safety
    ///
    /// Unsafe because the texture provided must either be null or pointing to a Metal texture.
    /// This function consumes the texture and releases it as soon TextureInfo is dropped.
    ///
    /// If the texture needs to be used otherwise, it's reference count must be increased before calling this function.
    pub unsafe fn new(texture: *const ffi::c_void) -> Self {
        Self::construct(|ti| sb::C_GrMtlTextureInfo_Construct(ti, texture))
    }

    pub fn texture(&self) -> *const ffi::c_void {
        self.native().fTexture.fObject
    }
}

#[test]
fn empty_texture_info() {
    drop(TextureInfo::default());
}
