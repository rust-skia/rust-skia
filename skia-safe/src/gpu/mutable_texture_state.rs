use crate::prelude::*;
use skia_bindings::{self as sb, skgpu_MutableTextureState};
use std::fmt;

pub type MutableTextureState = Handle<skgpu_MutableTextureState>;
unsafe_send_sync!(MutableTextureState);

impl NativeDrop for skgpu_MutableTextureState {
    fn drop(&mut self) {
        unsafe { sb::C_MutableTextureState_destruct(self) }
    }
}

impl Default for MutableTextureState {
    fn default() -> Self {
        Self::construct(|s| unsafe { sb::C_MutableTextureState_Construct(s) })
    }
}

impl fmt::Debug for MutableTextureState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutableTextureState").finish()
    }
}

impl MutableTextureState {
    #[cfg(feature = "vulkan")]
    pub fn new_vk(layout: crate::gpu::vk::ImageLayout, queue_family_index: u32) -> Self {
        Self::construct(|ptr| unsafe {
            sb::C_MutableTextureState_ConstructVK(ptr, layout, queue_family_index)
        })
    }
}
