use super::BackendApi;
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

impl NativeClone for skgpu_MutableTextureState {
    fn clone(&self) -> Self {
        construct(|s| unsafe { sb::C_MutableTextureState_CopyConstruct(s, self) })
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
    #[cfg(feature = "vulkan")]
    pub fn vk_image_layout(&self) -> sb::VkImageLayout {
        unsafe { sb::C_MutableTextureState_getVkImageLayout(self.native()) }
    }

    #[cfg(feature = "vulkan")]
    pub fn queue_family_index(&self) -> u32 {
        unsafe { sb::C_MutableTextureState_getQueueFamilyIndex(self.native()) }
    }

    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_MutableTextureState_backend(self.native()) }
    }
}
