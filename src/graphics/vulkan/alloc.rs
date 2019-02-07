use rust_skia::{GrVkAlloc, VkDeviceMemory};
use std::ffi::c_void;

pub struct Alloc {
    pub(crate) native: GrVkAlloc
}

impl Alloc {

    pub unsafe fn new(memory: *mut c_void, offset: u64, size: u64, flags: u32) -> Alloc
    {
        Alloc { native: GrVkAlloc {
            fMemory: memory as VkDeviceMemory,
            fOffset: offset,
            fSize: size,
            fFlags: flags,
            fBackendMemory: 0,
            fUsesSystemHeap: false
        }}
    }
}