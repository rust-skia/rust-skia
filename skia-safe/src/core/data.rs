use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::SkData;
use std::{
    ffi::{CStr, CString},
    ops::Deref,
};

pub type Data = RCHandle<SkData>;
unsafe impl Send for Data {}
unsafe impl Sync for Data {}

impl NativeRefCounted for SkData {
    fn _ref(&self) {
        unsafe { sb::C_SkData_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_SkData_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkData_unique(self) }
    }
}

impl Deref for RCHandle<SkData> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl PartialEq for RCHandle<SkData> {
    // Although there is an implementation in SkData for equality testing, we
    // prefer to stay on the Rust side for that.
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl RCHandle<SkData> {
    pub fn size(&self) -> usize {
        self.native().fSize
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { safer::from_raw_parts(self.native().fPtr as _, self.size()) }
    }

    // TODO:
    // pub unsafe fn writable_data(&mut self) -> &mut [u8]

    pub fn copy_range(&self, offset: usize, buffer: &mut [u8]) -> &Self {
        buffer.copy_from_slice(&self.as_bytes()[offset..offset + buffer.len()]);
        self
    }

    // TODO: rename to copy_from() ? or from_bytes()?
    pub fn new_copy(data: &[u8]) -> Self {
        Data::from_ptr(unsafe { sb::C_SkData_MakeWithCopy(data.as_ptr() as _, data.len()) })
            .unwrap()
    }

    /// Constructs Data from a given byte slice without copying it.
    ///
    /// Users must make sure that the underlying slice will outlive the lifetime of the Data.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_bytes(data: &[u8]) -> Self {
        Data::from_ptr(sb::C_SkData_MakeWithoutCopy(data.as_ptr() as _, data.len())).unwrap()
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_uninitialized(length: usize) -> Data {
        Data::from_ptr(sb::C_SkData_MakeUninitialized(length)).unwrap()
    }

    // TODO: use Range as stand in for offset / length?
    pub fn new_subset(data: &Data, offset: usize, length: usize) -> Data {
        Data::from_ptr(unsafe { sb::C_SkData_MakeSubset(data.native(), offset, length) }).unwrap()
    }

    /// Constructs Data from a copy of a &str.
    ///
    /// Functions that use Data as a string container usually expect it to contain
    /// a c-string including the terminating 0 byte, so this function converts
    /// the string to a CString and forwards it to new_cstr().
    pub fn new_str(str: impl AsRef<str>) -> Data {
        Self::new_cstr(&CString::new(str.as_ref()).unwrap())
    }

    /// Constructs Data from a &CStr by copying its contents.
    pub fn new_cstr(cstr: &CStr) -> Data {
        Data::from_ptr(unsafe { sb::C_SkData_MakeWithCString(cstr.as_ptr()) }).unwrap()
    }

    // TODO: MakeFromFileName (not sure if we need that)
    // TODO: MakeFromFile (not sure if we need that)
    // TODO: MakeFromStream

    pub fn new_empty() -> Self {
        Data::from_ptr(unsafe { sb::C_SkData_MakeEmpty() }).unwrap()
    }
}

#[cfg(test)]
impl RefCount for SkData {
    fn ref_cnt(&self) -> usize {
        self._base.ref_cnt()
    }
}

#[test]
fn data_supports_equals() {
    let x: &[u8] = &[1u8, 2u8, 3u8];
    let d1 = Data::new_copy(x);
    let d2 = Data::new_copy(x);
    assert!(d1 == d2)
}
