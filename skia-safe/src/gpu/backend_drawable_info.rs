#[cfg(feature = "vulkan")]
use crate::gpu::vk;
use crate::gpu::BackendAPI;
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::GrBackendDrawableInfo;

pub type BackendDrawableInfo = Handle<GrBackendDrawableInfo>;
unsafe impl Send for BackendDrawableInfo {}
unsafe impl Sync for BackendDrawableInfo {}

impl NativeDrop for GrBackendDrawableInfo {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendDrawableInfo_destruct(self) }
    }
}

impl Handle<GrBackendDrawableInfo> {
    pub fn new() -> BackendDrawableInfo {
        Self::construct(|di| unsafe { sb::C_GrBackendDrawableInfo_Construct(di) })
    }

    #[cfg(feature = "vulkan")]
    pub fn from_vk(info: &vk::DrawableInfo) -> BackendDrawableInfo {
        Self::construct(|di| unsafe { sb::C_GrBackendDrawableInfo_Construct2(di, info.native()) })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_GrBackendDrawableInfo_isValid(self.native()) }
    }

    pub fn backend(&self) -> BackendAPI {
        unsafe { sb::C_GrBackendDrawableInfo_backend(self.native()) }
    }

    #[cfg(feature = "vulkan")]
    pub fn get_vk_drawable_info(&self) -> Option<vk::DrawableInfo> {
        unsafe {
            let mut di = vk::DrawableInfo::default();
            sb::C_GrBackendDrawableInfo_getVkDrawableInfo(self.native(), di.native_mut())
                .if_true_some(di)
        }
    }
}
