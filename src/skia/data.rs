use std::slice;
use rust_skia::{SkData, C_SkData_unref};

#[derive(Debug)]
pub struct Data {
    pub(crate) native: *mut SkData
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { C_SkData_unref(&*self.native) }
    }
}

impl Data {
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (*self.native).bytes();
            slice::from_raw_parts(bytes, self.size())
        }
    }

    pub fn size(&self) -> usize {
        unsafe { (*self.native).size() }
    }
}

