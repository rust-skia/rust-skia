use crate::prelude::*;
use std::slice;
use rust_skia::{
    SkData,
    C_SkData_unref,
    C_SkData_ref
};
use crate::{
    prelude::RefCounted,
    prelude::RCHandle
};

pub type Data = RCHandle<SkData>;

impl RefCounted for SkData {
    fn _ref(&self) {
        unsafe { C_SkData_ref(self) }
    }

    fn _unref(&self) {
        unsafe { C_SkData_unref(self) }
    }
}

impl Data {
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
