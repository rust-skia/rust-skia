use crate::interop::{FromStrs, String};
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::SkStrings;
use std::ops::Index;

pub type Strings = Handle<SkStrings>;
unsafe impl Send for Strings {}
unsafe impl Sync for Strings {}

impl NativeDrop for SkStrings {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkStrings_destruct(self);
        }
    }
}

impl Handle<SkStrings> {
    /// Constructs a native Strings array from a slice of SkStrings by moving them.
    pub fn new(mut strings: Vec<String>) -> Self {
        Strings::construct(|s| unsafe {
            sb::C_SkStrings_construct(s, strings.native_mut().as_mut_ptr(), strings.len())
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        unsafe {
            sb::C_SkStrings_ptr_count(self.native(), &mut count);
        }
        count
    }
}

impl Index<usize> for Handle<SkStrings> {
    type Output = String;
    fn index(&self, index: usize) -> &Self::Output {
        let mut count = 0;
        let ptr = unsafe { sb::C_SkStrings_ptr_count(self.native(), &mut count) };
        let slice = unsafe { std::slice::from_raw_parts(ptr as *const String, count) };
        &slice[index]
    }
}

impl FromStrs for Handle<SkStrings> {
    fn from_strs(strings: &[impl AsRef<str>]) -> Self {
        Strings::new(strings.iter().map(String::from_str).collect())
    }
}

#[test]
fn test_strings() {
    let strings = ["Hello", "World"];
    let strings = Strings::from_strs(&strings);
    assert_eq!(strings[0].as_str(), "Hello");
    assert_eq!(strings[1].as_str(), "World");
}
