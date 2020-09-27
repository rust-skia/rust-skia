use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::GrBackendSurfaceMutableState;

pub type BackendSurfaceMutableState = Handle<GrBackendSurfaceMutableState>;
unsafe impl Send for BackendSurfaceMutableState {}
unsafe impl Sync for BackendSurfaceMutableState {}

impl NativeDrop for GrBackendSurfaceMutableState {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendSurfaceMutableState_destruct(self) }
    }
}

impl BackendSurfaceMutableState {
    #[cfg(feature = "vulkan")]
    pub fn new_vk(layout: crate::gpu::vk::ImageLayout, queue_family_index: u32) -> Self {
        Self::construct(|ptr| unsafe {
            sb::C_GrBackendSurfaceMutableState_ConstructVK(ptr, layout, queue_family_index)
        })
    }
}
