use crate::prelude::*;
use std::{slice, ops};
use skia_bindings::{SkData, C_SkData_unref, C_SkData_ref, C_SkData_MakeWithCopy, C_SkData_MakeEmpty, C_SkData_MakeSubset, C_SkData_MakeUninitialized, C_SkData_MakeWithCString};
use std::ops::Deref;
use std::ffi::CStr;

pub type Data = RCHandle<SkData>;

impl NativeRefCounted for SkData {
    fn _ref(&self) {
        unsafe { C_SkData_ref(self) }
    }

    fn _unref(&self) {
        unsafe { C_SkData_unref(self) }
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

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let bytes = self.native().bytes();
            slice::from_raw_parts(bytes, self.size())
        }
    }

    // TODO: (needs function unique())
    // pub fn writable_data(&mut self) -> &mut [u8]

    pub fn copy_range(&self, offset: usize, buffer: &mut [u8]) -> &Self {
        buffer.copy_from_slice(&self.as_bytes()[offset..offset + buffer.len()]);
        self
    }

    // TODO: rename to copy_from ?
    pub fn new_copy(data: &[u8]) -> Self {
        Data::from_ptr(unsafe {
            C_SkData_MakeWithCopy(data.as_ptr() as _, data.len())
        }).unwrap()
    }

    pub unsafe fn new_uninitialized(length: usize) -> Data {
        Data::from_ptr(unsafe {
            C_SkData_MakeUninitialized(length)
        }).unwrap()
    }

    // TODO: use Range as stand in for offset / length?
    pub fn new_subset(data: &Data, offset: usize, length: usize) -> Data {
        Data::from_ptr(unsafe {
            C_SkData_MakeSubset(data.native(), offset, length)
        }).unwrap()
    }

    // TODO: rename to from_cstr()?
    pub fn new_cstr(cstr: &CStr) -> Data {
        Data::from_ptr(unsafe {
            C_SkData_MakeWithCString(cstr.as_ptr())
        }).unwrap()
    }

    // TODO: MakeFromFileName (not sure if we need that)
    // TODO: MakeFromFile (not sure if we need that)
    // TODO: MakeFromStream

    pub fn new_empty() -> Self {
        Data::from_ptr(unsafe {
            C_SkData_MakeEmpty()
        }).unwrap()
    }
}

#[test]
fn data_supports_equals() {
    let x : &[u8] = &[1u8, 2u8, 3u8];
    let d1 = Data::new_copy(x);
    let d2 = Data::new_copy(x);
    assert!(d1 == d2)
}
