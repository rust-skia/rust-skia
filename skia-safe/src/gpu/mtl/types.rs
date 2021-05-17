// Can't use crate::prelude::* here, because we declare Handle in this module, too.
use crate::prelude::{self, NativeAccess, NativeDrop, NativePartialEq};
use skia_bindings::{self as sb, GrMtlTextureInfo};
use std::{fmt, ptr};

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

impl fmt::Debug for TextureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextureInfo")
            .field("texture", &self.texture())
            .finish()
    }
}

impl TextureInfo {
    /// # Safety
    ///
    /// Unsafe because the texture provided must either be `null` or pointing to a Metal texture by
    /// providing a raw pointer.
    ///
    /// This function retains the texture and releases it as soon TextureInfo is dropped.
    pub unsafe fn new(texture: Handle) -> Self {
        Self::construct(|ti| sb::C_GrMtlTextureInfo_Construct(ti, texture))
    }

    pub fn texture(&self) -> Handle {
        self.native().fTexture.fObject
    }
}

#[cfg(test)]
mod tests {
    use super::TextureInfo;

    #[test]
    fn default_texture_info() {
        drop(TextureInfo::default());
    }
}
