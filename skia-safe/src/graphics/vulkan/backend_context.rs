use std::ffi;
use std::os::raw;
use skia_bindings::{C_GrVkBackendContext_New, C_GrVkBackendContext_Delete, VkInstance, VkDevice };
use super::GetProc;

#[derive(Debug)]
pub struct BackendContext {
    pub (crate) native: *mut ffi::c_void
}

impl Drop for BackendContext {
    fn drop(&mut self) {
        unsafe { C_GrVkBackendContext_Delete(self.native) }
    }
}

impl BackendContext {

    pub unsafe fn new(
        instance: *mut ffi::c_void,
        physical_device: *mut ffi::c_void,
        device: *mut ffi::c_void,
        queue: *mut ffi::c_void,
        graphics_queue_index: u32,
        get_proc: GetProc
        ) -> BackendContext {

        BackendContext {
            native: C_GrVkBackendContext_New(
                instance,
                physical_device,
                device,
                queue,
                graphics_queue_index,
                get_proc) }
    }
}