use skia_bindings as sb;

use crate::{
    gpu::{BackendSemaphore, vk},
    prelude::*,
};

pub mod backend_semaphores {
    use super::*;

    /// Returns a [`BackendSemaphore`] that wraps the given Vulkan
    /// `VkSemaphore` handle. The handle remains owned by the caller; Skia
    /// signals or waits on it when this semaphore is passed to a flush
    /// or [`crate::Surface::wait`] operation.
    ///
    /// # Safety
    /// `semaphore` must be a valid `VkSemaphore` that stays alive until
    /// all operations Skia performs with the returned [`BackendSemaphore`]
    /// have completed.
    pub unsafe fn make_vk(semaphore: vk::Semaphore) -> BackendSemaphore {
        let backend_semaphore = BackendSemaphore::construct(|out| unsafe {
            sb::C_GrBackendSemaphore_ConstructVk(out, semaphore)
        });
        assert!(backend_semaphore.is_initialized());
        backend_semaphore
    }

    /// Returns the underlying `VkSemaphore` handle of an initialized
    /// Vulkan-flavoured [`BackendSemaphore`], or `None` when the
    /// semaphore is not initialized as Vulkan.
    pub fn get_vk_semaphore(semaphore: &BackendSemaphore) -> Option<vk::Semaphore> {
        if !semaphore.is_initialized() || semaphore.backend() != sb::GrBackendApi::Vulkan {
            return None;
        }
        // No null check (like `get_d3d_fence_info`): `vk::Semaphore` is
        // `u64` on 32-bit targets, where `is_null()` would not compile.
        Some(unsafe { sb::C_GrBackendSemaphores_GetVkSemaphore(semaphore.native()) })
    }
}
