use crate::prelude::*;
use std::slice;
use skia_bindings::{
    SkData,
    C_SkData_unref,
    C_SkData_ref,
    C_SkData_MakeWithCopy
};

pub type Data = RCHandle<SkData>;

impl NativeRefCounted for SkData {
    fn _ref(&self) {
        unsafe { C_SkData_ref(self) }
    }

    fn _unref(&self) {
        unsafe { C_SkData_unref(self) }
    }
}

// TODO: complete the implementation.
// TODO: think about if we should support Data at all, Rust arrays and slices seem to
// cover that.
impl RCHandle<SkData> {

    pub fn new_copy(data: &[u8]) -> Self {
        Data::from_ptr(unsafe {
            C_SkData_MakeWithCopy(data.as_ptr() as _, data.len())
        }).unwrap()
    }

    pub fn bytes(&self) -> &[u8] {
        unsafe {
            let bytes = self.native().bytes();
            slice::from_raw_parts(bytes, self.size())
        }
    }

    pub fn size(&self) -> usize {
        unsafe { self.native().size() }
    }
}
