use crate::prelude::*;
use skia_bindings::{
    C_SkData_MakeEmpty, C_SkData_MakeSubset, C_SkData_MakeUninitialized, C_SkData_MakeWithCString,
    C_SkData_MakeWithCopy, C_SkData_ref, C_SkData_unique, C_SkData_unref, SkData,
};
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::slice;

pub type Data = RCHandle<SkData>;
unsafe impl Send for Data {}

impl NativeRefCounted for SkData {
    fn _ref(&self) {
        unsafe { C_SkData_ref(self) }
    }

    fn _unref(&self) {
        unsafe { C_SkData_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { C_SkData_unique(self) }
    }
}

impl Deref for RCHandle<SkData> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl PartialEq for RCHandle<SkData> {
    // Although there is an implementation in SkData for equality testig, we
    // prefer to stay on the Rust side for that.
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl RCHandle<SkData> {
    pub fn size(&self) -> usize {
        unsafe { self.native().size() }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    #[deprecated(since = "0.12.0", note = "use as_bytes()")]
    pub fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = self.native().bytes();
            slice::from_raw_parts(bytes, self.size())
        }
    }

    // TODO:
    // pub fn writable_data(&mut self) -> &mut [u8]

    pub fn copy_range(&self, offset: usize, buffer: &mut [u8]) -> &Self {
        buffer.copy_from_slice(&self.as_bytes()[offset..offset + buffer.len()]);
        self
    }

    // TODO: rename to copy_from() ? or from_bytes()?
    pub fn new_copy(data: &[u8]) -> Self {
        Data::from_ptr(unsafe { C_SkData_MakeWithCopy(data.as_ptr() as _, data.len()) }).unwrap()
    }

    pub unsafe fn new_uninitialized(length: usize) -> Data {
        Data::from_ptr(C_SkData_MakeUninitialized(length)).unwrap()
    }

    // TODO: use Range as stand in for offset / length?
    pub fn new_subset(data: &Data, offset: usize, length: usize) -> Data {
        Data::from_ptr(unsafe { C_SkData_MakeSubset(data.native(), offset, length) }).unwrap()
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
        Data::from_ptr(unsafe { C_SkData_MakeWithCString(cstr.as_ptr()) }).unwrap()
    }

    // TODO: MakeFromFileName (not sure if we need that)
    // TODO: MakeFromFile (not sure if we need that)
    // TODO: MakeFromStream

    pub fn new_empty() -> Self {
        Data::from_ptr(unsafe { C_SkData_MakeEmpty() }).unwrap()
    }
}

#[test]
fn data_supports_equals() {
    let x: &[u8] = &[1u8, 2u8, 3u8];
    let d1 = Data::new_copy(x);
    let d2 = Data::new_copy(x);
    assert!(d1 == d2)
}
