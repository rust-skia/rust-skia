use crate::prelude::*;
use skia_bindings::{SkString, C_SkString_destruct};
use std::{slice, str};

pub type String = Handle<SkString>;

impl NativeDrop for SkString {
    fn drop(&mut self) {
        unsafe {
            C_SkString_destruct(self);
        }
    }
}

impl ToString for String {
    fn to_string(&self) -> std::string::String {
        self.as_str().into()
    }
}

impl Handle<SkString> {
    pub fn from_str(str: &str) -> String {
        let bytes = str.as_bytes();
        Handle::from_native(
            unsafe {
                SkString::new3(bytes.as_ptr() as _, bytes.len())
            }
        )
    }

    pub fn as_str(&self) -> &str {
        let slice = unsafe {
            slice::from_raw_parts(self.native().c_str() as _, self.native().size())
        };
        str::from_utf8(slice).unwrap()
    }
}

#[test]
fn string_from_rust_and_back() {
    let str = "Hello";
    let string = String::from_str(str);
    assert_eq!(str, string.as_str())
}
