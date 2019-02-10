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
    pub unsafe fn new_vulkan(
        (width, height): (u32, u32),
        vk_info: &vulkan::ImageInfo) -> BackendTexture
    {
        BackendTexture {
            native: {
                GrBackendTexture::new2(
                    width as i32,
                    height as i32,
                    &vk_info.native) }
        }
    }
}
