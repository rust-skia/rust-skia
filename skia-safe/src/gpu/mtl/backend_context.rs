use super::Handle;
use crate::prelude::{self, NativeDrop};
use skia_bindings as sb;
use skia_bindings::GrMtlBackendContext;

pub type BackendContext = prelude::Handle<GrMtlBackendContext>;
unsafe impl Send for BackendContext {}
unsafe impl Sync for BackendContext {}

impl NativeDrop for GrMtlBackendContext {
    fn drop(&mut self) {
        unsafe { sb::C_GrMtlBackendContext_Destruct(self) }
    }
}

impl BackendContext {
    /// # Safety
    ///
    /// Unsafe because it expects various objects in form of `c_void` pointers.
    ///
    /// This function retains all the non-`null` handles passed to it and releases them as soon the
    /// [BackendContext] is dropped.
    pub unsafe fn new(device: Handle, queue: Handle, binary_archive: Handle) -> Self {
        BackendContext::construct(|bc| {
            sb::C_GrMtlBackendContext_Construct(bc, device, queue, binary_archive)
        })
    }
}
