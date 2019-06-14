#[cfg(feature = "vulkan")]
use crate::gpu::vk;
use crate::gpu::BackendAPI;
use crate::prelude::*;
use skia_bindings::{C_GrBackendDrawableInfo_destruct, GrBackendDrawableInfo};

pub type BackendDrawableInfo = Handle<GrBackendDrawableInfo>;

impl NativeDrop for GrBackendDrawableInfo {
    fn drop(&mut self) {
        unsafe { C_GrBackendDrawableInfo_destruct(self) }
    }
}

impl Handle<GrBackendDrawableInfo> {
    pub fn new() -> BackendDrawableInfo {
        Self::from_native(unsafe { GrBackendDrawableInfo::new() })
    }

    #[cfg(feature = "vulkan")]
    pub fn from_vk(info: &vk::DrawableInfo) -> BackendDrawableInfo {
        Self::from_native(unsafe { GrBackendDrawableInfo::new1(info.native()) })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }

    pub fn backend(&self) -> BackendAPI {
        BackendAPI::from_native(unsafe { self.native().backend() })
    }

    #[cfg(feature = "vulkan")]
    pub fn get_vk_drawable_info(&self) -> vk::DrawableInfo {
        use std::mem;
        unsafe {
            let mut di = mem::zeroed();
            self.native().getVkDrawableInfo(&mut di)
        }
        .if_true_some(di)
    }
}
