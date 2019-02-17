use std::mem;
use rust_skia::SkMatrix;

#[derive(Copy, Clone)]
pub struct Matrix(pub(crate) SkMatrix);

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { rust_skia::C_SkMatrix_Equals(&self.0, &rhs.0) }
    }
}

impl Matrix {
    // creates an identity matrix.
    pub fn new() -> Matrix {
        // SkMatrix contains no C++ types, so this is safe:
        let mut m : SkMatrix = unsafe { mem::zeroed() };
        unsafe { m.reset() };
        Matrix(m)
    }
}

