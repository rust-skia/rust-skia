use std::slice;
use rust_skia::{SkData, C_SkData_unref, C_SkData_ref};
use crate::prelude::RefCounted;

#[derive(Debug, RCCloneDrop)]
pub struct Data(pub(crate) *mut SkData);

impl RefCounted for Data {
    fn _ref(&self) {
        unsafe { C_SkData_ref(self.0) }
    }

    fn _unref(&self) {
        unsafe { C_SkData_unref(self.0) }
    }
}

impl Data {
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (*self.0).bytes();
            slice::from_raw_parts(bytes, self.size())
        }
    }

    pub fn size(&self) -> usize {
        unsafe { (*self.0).size() }
    }
}
