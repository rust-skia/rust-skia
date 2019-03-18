use crate::prelude::*;
use skia_bindings::{GrBackendTexture, C_GrBackendTexture_destruct};

#[cfg(feature = "vulkan")]
use super::vulkan;

pub type BackendTexture = Handle<GrBackendTexture>;

impl NativeDrop for GrBackendTexture {
    fn drop(&mut self) {
        unsafe { C_GrBackendTexture_destruct(self) }
    }
}

impl NativeClone for GrBackendTexture {
    fn clone(&self) -> Self {
        unsafe { GrBackendTexture::new4(self) }
    }
}

impl Handle<GrBackendTexture> {

    #[cfg(feature = "vulkan")]
    pub unsafe fn new_vulkan(
        (width, height): (i32, i32),
        vk_info: &vulkan::ImageInfo) -> BackendTexture {
        Self::from_raw(
            GrBackendTexture::new2(
                width,
                height,
                vk_info.native()))
            .unwrap()
    }

    pub (crate) unsafe fn from_raw(backend_texture: GrBackendTexture) -> Option<BackendTexture> {
        if backend_texture.fIsValid {
            Some (BackendTexture::from_native(backend_texture))
        } else {
            None
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    #[cfg(feature = "vulkan")]
    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    #[cfg(feature = "vulkan")]
    pub fn has_mip_maps(&self) -> bool {
        unsafe { self.native().hasMipMaps() }
    }

    #[cfg(feature = "vulkan")]
    pub fn get_image_info(&self) -> Option<vulkan::ImageInfo> {
        unsafe {
            // constructor not available.
            let mut image_info = vulkan::ImageInfo::default();
            self.native().getVkImageInfo(image_info.native_mut())
                .if_true_some(image_info)
        }
    }
}
