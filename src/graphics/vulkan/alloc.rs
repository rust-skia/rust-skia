use skia_bindings::{GrVkAlloc, VkDeviceMemory, VkDeviceSize};
use std::ffi::c_void;

#[derive(Debug)]
pub struct Alloc {
    pub(crate) native: GrVkAlloc
}

impl Alloc {

    pub unsafe fn new(memory: *mut c_void, offset: u64, size: u64, flags: u32) -> Alloc
    {
        Self::from_raw(GrVkAlloc {
            fMemory: memory as VkDeviceMemory,
            fOffset: offset,
            fSize: size,
            fFlags: flags,
            fBackendMemory: 0,
            fUsesSystemHeap: false
        })
    }

    pub(crate) unsafe fn from_raw(alloc: GrVkAlloc) -> Alloc {
        Alloc { native: alloc }
    }

    #[inline]
    pub fn memory(&self) -> VkDeviceMemory {
        self.native.fMemory
    }

    #[inline]
    pub fn offset(&self) -> VkDeviceSize {
        self.native.fOffset
    }

    #[inline]
    pub fn size(&self) -> VkDeviceSize {
        self.native.fSize
    }

    #[inline]
    pub fn flags(&self) -> u32 {
        self.native.fFlags
    }
}