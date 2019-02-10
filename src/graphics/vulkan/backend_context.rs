use std::ffi::c_void;
use rust_skia::{C_GrVkBackendContext_New, C_GrVkBackendContext_Delete};

#[derive(Debug)]
pub struct BackendContext {
    pub (crate) native: *mut c_void
}

impl Drop for BackendContext {
    fn drop(&mut self) {
        unsafe { C_GrVkBackendContext_Delete(self.native) }
    }
}

impl BackendContext {

    pub unsafe fn new(
        instance: *mut c_void,
        physical_device: *mut c_void,
        device: *mut c_void,
        queue: *mut c_void,
        graphics_queue_index: u32
        ) -> BackendContext {

        BackendContext {
            native: C_GrVkBackendContext_New(
                instance,
                physical_device,
                device,
                queue,
                graphics_queue_index) }
    }
}