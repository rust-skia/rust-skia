use crate::prelude::*;
use skia_bindings::{self as sb, GrBackendSurfaceMutableState};
use std::fmt;

pub type BackendSurfaceMutableState = Handle<GrBackendSurfaceMutableState>;
unsafe_send_sync!(BackendSurfaceMutableState);

impl NativeDrop for GrBackendSurfaceMutableState {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendSurfaceMutableState_destruct(self) }
    }
}

impl Default for BackendSurfaceMutableState {
    fn default() -> Self {
        BackendSurfaceMutableState::construct(|s| unsafe {
            sb::C_GrBackendSurfaceMutableState_Construct(s)
        })
    }
}

impl fmt::Debug for BackendSurfaceMutableState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BackendSurfaceMutableState").finish()
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
