use rust_skia::{GrBackendTexture, C_GrBackendTexture_destruct};

#[cfg(feature = "vulkan")]
use super::vulkan;

pub struct BackendTexture {
    pub(crate) native: GrBackendTexture
}

impl Drop for BackendTexture {
    fn drop(&mut self) {
        unsafe { C_GrBackendTexture_destruct(&self.native) }
    }
}

impl BackendTexture {

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(width: i32, height: i32, vk_info: &vulkan::ImageInfo) -> BackendTexture
    {
        BackendTexture {
            native: unsafe { GrBackendTexture::new2(width, height, &vk_info.native) }
        }
    }

}
