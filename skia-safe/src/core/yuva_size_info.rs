use crate::prelude::*;
use crate::{EncodedOrigin, ISize};
use skia_bindings as sb;
use skia_bindings::SkYUVASizeInfo;
use std::ffi::c_void;

#[derive(Clone, Default, Debug)]
pub struct YUVASizeInfo {
    pub sizes: [ISize; Self::MAX_COUNT],
    pub width_bytes: [usize; Self::MAX_COUNT],
    pub origin: EncodedOrigin,
}

impl PartialEq for YUVASizeInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_SkYUVASizeInfo_equals(self.native(), other.native()) }
    }
}

impl NativeTransmutable<SkYUVASizeInfo> for YUVASizeInfo {}

impl YUVASizeInfo {
    pub const MAX_COUNT: usize = 4;

    pub fn compute_total_bytes(&self) -> usize {
        unsafe { sb::C_SkYUVASizeInfo_computeTotalBytes(self.native()) }
    }

    // TODO: try to expose a safe(r) Rust function.
    /// # Safety
    /// This function is a forwarder to the native C++ function and is therefore inherently unsafe.
    pub unsafe fn compute_planes(
        &self,
        base: *mut c_void,
        planes: &mut [*mut c_void; Self::MAX_COUNT],
    ) {
        self.native().computePlanes(base, planes.as_mut_ptr());
    }
}
