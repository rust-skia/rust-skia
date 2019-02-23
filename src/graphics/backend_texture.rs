use crate::prelude::*;
use std::mem;
use rust_skia::{GrBackendTexture, C_GrBackendTexture_destruct, GrVkImageInfo};

#[cfg(feature = "vulkan")]
use super::vulkan;

pub struct BackendTexture (pub(crate) GrBackendTexture);

impl Drop for BackendTexture {
    fn drop(&mut self) {
        unsafe { C_GrBackendTexture_destruct(&self.0) }
    }
}

impl Clone for BackendTexture {
    fn clone(&self) -> Self {
        BackendTexture(self.0.clone())
    }
}

impl InternalClone for GrBackendTexture {
    fn clone(&self) -> Self {
        unsafe { GrBackendTexture::new4(self) }
    }
}

impl BackendTexture {

    #[cfg(feature = "vulkan")]
    pub unsafe fn new_vulkan(
        (width, height): (u32, u32),
        vk_info: &vulkan::ImageInfo) -> BackendTexture {
        Self::from_raw(
            GrBackendTexture::new2(
                width as i32,
                height as i32,
                &vk_info.native))
            .unwrap()
    }

    pub (crate) unsafe fn from_raw(backend_texture: GrBackendTexture) -> Option<BackendTexture> {
        if backend_texture.fIsValid {
            Some (BackendTexture(backend_texture))
        } else {
            None
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn width(&self) -> u32 {
        unsafe { self.0.width() as u32 }
    }

    #[cfg(feature = "vulkan")]
    pub fn height(&self) -> u32 {
        unsafe { self.0.height() as u32 }
    }

    #[cfg(feature = "vulkan")]
    pub fn has_mip_maps(&self) -> bool {
        unsafe { self.0.hasMipMaps() }
    }

    #[cfg(feature = "vulkan")]
    pub fn get_image_info(&self) -> Option<vulkan::ImageInfo> {
        unsafe {
            // constructor not available.
            let mut image_info : GrVkImageInfo = mem::zeroed();
            if self.0.getVkImageInfo(&mut image_info as _) {
                Some(vulkan::ImageInfo::from_raw(image_info))
            } else {
                None
            }
        }
    }
}
