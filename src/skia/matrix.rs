use std::mem;
use rust_skia::SkMatrix;
use crate::prelude::*;

pub type Matrix = Handle<SkMatrix>;

impl NativeDrop for SkMatrix {
    fn drop(&mut self) {}
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { rust_skia::C_SkMatrix_Equals(self.native(), rhs.native()) }
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

