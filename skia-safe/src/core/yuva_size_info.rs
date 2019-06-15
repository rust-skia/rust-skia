use crate::prelude::*;
use crate::{EncodedOrigin, ISize};
use skia_bindings::{C_SkYUVASizeInfo_computeTotalBytes, C_SkYUVASizeInfo_equals, SkYUVASizeInfo};
use std::ffi::c_void;

#[derive(Clone, Default, Debug)]
pub struct YUVASizeInfo {
    pub sizes: [ISize; Self::MAX_COUNT],
    pub width_bytes: [usize; Self::MAX_COUNT],
    pub origin: EncodedOrigin,
}

impl PartialEq for YUVASizeInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { C_SkYUVASizeInfo_equals(self.native(), other.native()) }
    }
}

impl NativeTransmutable<SkYUVASizeInfo> for YUVASizeInfo {}

impl YUVASizeInfo {
    pub const MAX_COUNT: usize = 4;

    pub fn compute_total_bytes(&self) -> usize {
        // does not link:
        // unsafe { self.native().computeTotalBytes() }
        unsafe { C_SkYUVASizeInfo_computeTotalBytes(self.native()) }
    }

    pub unsafe fn compute_planes(
        &self,
        base: *mut c_void,
        planes: &mut [*mut c_void; Self::MAX_COUNT],
    ) {
        self.native().computePlanes(base, planes.as_mut_ptr());
    }
}
