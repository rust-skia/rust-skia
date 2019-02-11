use std::ffi::c_void;
use std::os::raw;
use rust_skia::{C_GrVkBackendContext_New, C_GrVkBackendContext_Delete, VkInstance, VkDevice };

#[derive(Debug)]
pub struct BackendContext {
    pub (crate) native: *mut c_void
}

impl Drop for BackendContext {
    fn drop(&mut self) {
        unsafe { C_GrVkBackendContext_Delete(self.native) }
    }
}

// A proper Option<fn()> return type here makes trouble on the Rust side, so we keep that a void* for now.
type GetProc = Option<unsafe extern "C" fn (*const raw::c_char, VkInstance, VkDevice) -> *const raw::c_void>;

impl BackendContext {

    pub unsafe fn new(
        instance: *mut c_void,
        physical_device: *mut c_void,
        device: *mut c_void,
        queue: *mut c_void,
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