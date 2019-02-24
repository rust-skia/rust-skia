use std::mem;
use rust_skia::SkMatrix;
use crate::prelude::*;

pub type Matrix = ValueHandle<SkMatrix>;

impl NativePartialEq for SkMatrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { rust_skia::C_SkMatrix_Equals(self, rhs) }
    }
}

impl Matrix {
    // creates an identity matrix.
    pub fn new() -> Matrix {
        // SkMatrix contains no C++ types, so this is safe:
        let mut m : SkMatrix = unsafe { mem::zeroed() };
        unsafe { m.reset() };
        Matrix::from_native(m)
    }
}
