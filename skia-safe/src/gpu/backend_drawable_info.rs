#[cfg(feature = "vulkan")]
use crate::gpu::vk;
use crate::gpu::BackendAPI;
use crate::prelude::*;
use skia_bindings::{
    C_GrBackendDrawableInfo_backend, C_GrBackendDrawableInfo_construct,
    C_GrBackendDrawableInfo_destruct, C_GrBackendDrawableInfo_isValid, GrBackendDrawableInfo,
};

pub type BackendDrawableInfo = Handle<GrBackendDrawableInfo>;

impl NativeDrop for GrBackendDrawableInfo {
    fn drop(&mut self) {
        unsafe { C_GrBackendDrawableInfo_destruct(self) }
    }
}

impl Handle<GrBackendDrawableInfo> {
    pub fn new() -> BackendDrawableInfo {
        // does not link:
        // Self::from_native(unsafe { GrBackendDrawableInfo::new() })
        Self::construct(|di| unsafe { C_GrBackendDrawableInfo_construct(di) })
    }

    #[cfg(feature = "vulkan")]
    pub fn from_vk(info: &vk::DrawableInfo) -> BackendDrawableInfo {
        Self::from_native(unsafe { GrBackendDrawableInfo::new1(info.native()) })
    }

    pub fn is_valid(&self) -> bool {
        // does not link:
        // unsafe { self.native().isValid() }
        unsafe { C_GrBackendDrawableInfo_isValid(self.native()) }
    }

    pub fn backend(&self) -> BackendAPI {
        // does not link:
        // BackendAPI::from_native(unsafe { self.native().backend() })
        BackendAPI::from_native(unsafe { C_GrBackendDrawableInfo_backend(self.native()) })
    }

    #[cfg(feature = "vulkan")]
    pub fn get_vk_drawable_info(&self) -> Option<vk::DrawableInfo> {
        unsafe {
            let mut di = vk::DrawableInfo::default();
            skia_bindings::C_GrBackendDrawableInfo_getVkDrawableInfo(self.native(), di.native_mut())
                .if_true_some(di)
        }
    }
}
