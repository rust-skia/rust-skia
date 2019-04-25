use crate::prelude::*;
use std::{mem, ops};
use crate::core::{Matrix, MatrixTypeMask, Vector4, scalar, Vector3 };
use skia_bindings::{
    C_SkMatrix44_Equals,
    SkMatrix44,
    C_SkMatrix44_SkMatrix,
    C_SkMatrix44_Mul,
    C_SkMatrix44_MulV4,
    C_SkMatrix44_ConstructIdentity,
};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Matrix44(SkMatrix44);

impl NativeTransmutable<SkMatrix44> for Matrix44 {}
#[test] fn test_matrix44_layout() { Matrix44::test_layout() }

impl NativePartialEq for SkMatrix44 {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkMatrix44_Equals(self, rhs)}
    }
}

impl Default for Matrix44 {
    fn default() -> Self {
        Matrix44::new_identity()
    }
}

impl Into<Matrix> for Matrix44 {
    fn into(self) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { C_SkMatrix44_SkMatrix(self.native(), m.native_mut()) };
        m
    }
}

impl ops::Mul for Matrix44 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::new_identity();
        unsafe {
            C_SkMatrix44_Mul(self.native(), rhs.native(), out.native_mut())
        }
        out
    }
}

impl ops::Mul<Vector4> for Matrix44 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let mut out = Vector4::default();
        unsafe {
            C_SkMatrix44_MulV4(self.native(), rhs.native(), out.native_mut())
        }
        out
    }
}

impl Matrix44 {

    pub const ROWS : usize = 4;
    pub const COLUMNS : usize = 4;

    pub fn new_identity() -> Self {
        Matrix44::from_native(unsafe {
            let mut matrix: SkMatrix44 = mem::zeroed();
            C_SkMatrix44_ConstructIdentity(&mut matrix);
            matrix
        })
    }

    pub fn get_type(&self) -> MatrixTypeMask {
        MatrixTypeMask::from_bits_truncate(unsafe {
            self.native().getType()
        }.try_into().unwrap())
    }

    pub fn is_identity(&self) -> bool {
        unsafe { self.native().isIdentity() }
    }

    pub fn is_translate(&self) -> bool {
        unsafe { self.native().isTranslate() }
    }

    pub fn is_scale_translate(&self) -> bool {
        unsafe { self.native().isScaleTranslate() }
    }

    pub fn is_scale(&self) -> bool {
        // linker error:
        // unsafe { self.0.isScale() }
        (self.get_type() & !MatrixTypeMask::SCALE).is_empty()
    }

    pub fn has_perspective(&self) -> bool {
        // would cause a linker error
        self.get_type().contains(MatrixTypeMask::PERSPECTIVE)
    }

    pub fn set_identity(&mut self) -> &mut Self {
        unsafe { self.native_mut().setIdentity() }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        // reset() would cause a linker error.
        self.set_identity()
    }

    pub fn get(&self, (row, column): (usize, usize)) -> scalar {
        assert!(row <= Self::ROWS && column <= Self::COLUMNS);
        unsafe { self.native().get(row as _ , column as _) }
    }

    pub fn set(&mut self, (row, column): (usize, usize), value: scalar) -> &mut Self {
        assert!(row <= Self::ROWS && column <= Self::COLUMNS);
        unsafe { self.native_mut().set(row as _, column as _, value) }
        self
    }

    pub fn as_col_major(&self, floats: &mut [scalar; 16]) {
        unsafe { self.native().asColMajorf(floats.as_mut_ptr())}
    }

    pub fn as_row_major(&self, floats: &mut [scalar; 16]) {
        unsafe { self.native().asRowMajorf(floats.as_mut_ptr())}
    }

    pub fn set_col_major(&mut self, floats: &[scalar; 16]) -> &mut Self {
        unsafe { self.native_mut().setColMajorf(floats.as_ptr()) }
        self
    }

    pub fn set_row_major(&mut self, floats: &[scalar; 16]) -> &mut Self {
        unsafe { self.native_mut().setRowMajorf(floats.as_ptr()) }
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_3x3(&mut self,
                   m_00: scalar, m_10: scalar, m_20: scalar,
                   m_01: scalar, m_11: scalar, m_21: scalar,
                   m_02: scalar, m_12: scalar, m_22: scalar) -> &mut Self {
        unsafe {
            self.native_mut().set3x3(m_00, m_10, m_20, m_01, m_11, m_21, m_02, m_12, m_22)
        }
        self
    }

    pub fn set_3x3_row_major(&mut self, floats: &[scalar; 9]) -> &mut Self {
        unsafe { self.native_mut().set3x3RowMajorf(floats.as_ptr())}
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_4x4(&mut self,
        m_00: scalar, m_10: scalar, m_20: scalar, m_30: scalar,
        m_01: scalar, m_11: scalar, m_21: scalar, m_31: scalar,
        m_02: scalar, m_12: scalar, m_22: scalar, m_32: scalar,
        m_03: scalar, m_13: scalar, m_23: scalar, m_33: scalar,
    ) -> &mut Self {
        unsafe { self.native_mut().set4x4(
            m_00, m_10, m_20, m_30,
            m_01, m_11, m_21, m_31,
            m_02, m_12, m_22, m_32,
            m_03, m_13, m_23, m_33,
        )};
        self
    }

    pub fn set_translate(&mut self, d: Vector3) -> &mut Self {
        unsafe { self.native_mut().setTranslate(d.x, d.y, d.z) }
        self
    }

    pub fn pre_translate(&mut self, d: Vector3) -> &mut Self {
        unsafe { self.native_mut().preTranslate(d.x, d.y, d.z) }
        self
    }

    pub fn post_translate(&mut self, d: Vector3) -> &mut Self {
        unsafe { self.native_mut().postTranslate(d.x, d.y, d.z) }
        self
    }

    // TODO: implement

    /*
    pub fn set_rotate_degrees_about(&mut self, v: Vector3, degrees: scalar) -> &mut Self {
        unimplemented!("linker error");
    }
    */

    pub fn set_rotate_about(&mut self, v: Vector3, radians: scalar) -> &mut Self {
        unsafe {
            self.native_mut().setRotateAbout(v.x, v.y, v.z, radians)
        }
        self
    }

    pub fn set_rotate_about_unit(&mut self, v: Vector3, radians: scalar) -> &mut Self {
        unsafe {
            self.native_mut().setRotateAboutUnit(v.x, v.y, v.z, radians)
        }
        self
    }

    pub fn set_concat(&mut self, a: &Self, b: &Self) -> &mut Self {
        unsafe { self.native_mut().setConcat(a.native(), b.native())}
        self
    }

    pub fn pre_concat(&mut self, m: &Self) -> &mut Self {
        // would cause a linker error
        self.set_concat(&self.clone(), &m)
    }

    pub fn post_concat(&mut self, m: &Self) -> &mut Self {
        // would cause a linker error
        self.set_concat(&m, &self.clone())
    }

    pub fn inverse(&self) -> Option<Matrix44> {
        let mut r = Matrix44::default();
        unsafe { self.native().invert(r.native_mut()) }
            .if_true_some(r)
    }

    pub fn transpose(&mut self) -> &mut Self {
        unsafe { self.native_mut().transpose() }
        self
    }

    pub fn map_scalars(&self, src: &[scalar; 4], dst: &mut [scalar; 4]) {
        unsafe {
            self.native().mapScalars(src.as_ptr(), dst.as_mut_ptr())
        }
    }
}

pub trait SetPrePostScale<T> {
    fn set_scale(&mut self, v: T) -> &mut Self;
    fn pre_scale(&mut self, v: T) -> &mut Self;
    fn post_scale(&mut self, v: T) -> &mut Self;
}

impl SetPrePostScale<scalar> for Matrix44 {

    fn set_scale(&mut self, s: scalar) -> &mut Self {
        self.set_scale((s, s, s))
    }

    fn pre_scale(&mut self, s: scalar) -> &mut Self {
        self.pre_scale((s, s, s))
    }

    fn post_scale(&mut self, s: scalar) -> &mut Self {
        self.post_scale((s, s, s))
    }
}

impl SetPrePostScale<(scalar, scalar, scalar)> for Matrix44 {

    fn set_scale(&mut self, (sx, sy, sz): (scalar, scalar, scalar)) -> &mut Self {
        unsafe { self.native_mut().setScale(sx, sy, sz) }
        self
    }

    fn pre_scale(&mut self, (sx, sy, sz): (scalar, scalar, scalar)) -> &mut Self {
        unsafe { self.native_mut().preScale(sx, sy, sz) }
        self
    }

    fn post_scale(&mut self, (sx, sy, sz): (scalar, scalar, scalar)) -> &mut Self {
        unsafe { self.native_mut().postScale(sx, sy, sz) }
        self
    }
}

pub trait MapScalars<T> {
    fn map_scalars(&self, v: T);
}

impl MapScalars<(&[scalar; 4], &mut [scalar; 4])> for Matrix44 {
    fn map_scalars(&self, (src, dst): (&[scalar; 4], &mut [scalar; 4])) {
        unsafe { self.native().mapScalars(src.as_ptr(), dst.as_mut_ptr()) }
    }
}

impl MapScalars<&mut [scalar; 4]> for Matrix44 {
    fn map_scalars(&self, vec: &mut [scalar; 4]) {
        unsafe { self.native().mapScalars(vec.as_mut_ptr(), vec.as_mut_ptr()) }
    }
}

pub trait Map2<T> {
    fn map2(&self, v: T);
}

impl Map2<(&[scalar], &mut[scalar])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[scalar], &mut[scalar])) {
        assert_eq!(0, (src2.len() % 2) & 1);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe { self.native().map2(src2.as_ptr(), (src2.len() / 2).try_into().unwrap(), dst4.as_mut_ptr()) }
    }
}

impl Map2<(&[f64], &mut[f64])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[f64], &mut[f64])) {
        assert_eq!(0, (src2.len() % 2) & 1);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe { self.native().map21(src2.as_ptr(), (src2.len() / 2).try_into().unwrap(), dst4.as_mut_ptr()) }
    }
}

#[test]
fn create_identity_and_clone() {
    Matrix44::new_identity();
    let identity = Matrix44::new_identity();
    let _cloned = identity.clone();
}
