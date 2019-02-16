use rust_skia::*;
use std::mem;

pub struct Matrix44(pub(crate) SkMatrix44);

impl Matrix44 {
    pub fn new() -> Matrix44 {
        let mut m : SkMatrix44 = unsafe {mem::zeroed()};
        unsafe { C_SkMatrix44_Construct(&mut m as _) };
        Matrix44(m)
    }

    pub fn new_identity() -> Matrix44 {
        Matrix44(unsafe{SkMatrix44::new1(SkMatrix44_Identity_Constructor_kIdentity_Constructor)})
    }
}

impl Clone for Matrix44 {
    fn clone(&self) -> Self {
        Matrix44(unsafe {SkMatrix44::new3(&self.0 as _)})
    }
}

#[test]
fn create_identity_and_clone() {
    Matrix44::new();
    let identity = Matrix44::new_identity();
    identity.clone();
}
