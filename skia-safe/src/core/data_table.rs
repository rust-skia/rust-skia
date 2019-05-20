use std::ffi::CStr;

// pub type DataTable = RCHandle<SkDataTable>;
pub enum DataTable {}

// TODO: implement an Index and Iter.

impl DataTable {
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    pub fn count(&self) -> usize {
        unimplemented!()
    }

    pub fn at_size(&self, index: usize) -> usize {
        unimplemented!()
    }

    pub fn at(&self, index: usize) -> &[u8] {
        unimplemented!()
    }

    // TODO: atT()? (may be too unsecure).
    // Implementation _must_ check the assumption about the null-byte here.
    pub fn at_str(&self, index: usize) -> &CStr {
        unimplemented!()
    }

    pub fn new_empty() -> Self {
        unimplemented!()
    }

    pub fn copy_from_slices(slices: &[&[u8]]) -> Self {
        unimplemented!()
    }

    pub fn copy_from_slice(slice: &[u8], elem_size: usize) -> Self {
        unimplemented!()
    }
}
