use std::slice;
use rust_skia::{SkData, C_SkData_unref, C_SkData_ref};

#[derive(Debug)]
pub struct Data(pub(crate) *mut SkData);

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { C_SkData_unref(self.0) }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        unsafe { C_SkData_ref(self.0) };
        Data(self.0)
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

