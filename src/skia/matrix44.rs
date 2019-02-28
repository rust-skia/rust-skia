use crate::prelude::*;
use std::{mem, ops};
use super::{Matrix, MatrixTypeMask, Vector4};
use rust_skia::{
    C_SkMatrix44_Equals,
    C_SkMatrix44_Destruct,
    SkMatrix44,
    C_SkMatrix44_SkMatrix,
    C_SkMatrix44_Mul,
    SkVector4,
    C_SkMatrix44_MulV4,
    C_SkMatrix44_Construct,
    SkMatrix44_Identity_Constructor_kIdentity_Constructor
};

pub struct Matrix44(pub(crate) SkMatrix44);

impl Drop for Matrix44 {
    fn drop(&mut self) {
        unsafe { C_SkMatrix44_Destruct(&mut self.0) }
    }
}

impl Clone for Matrix44 {
    fn clone(&self) -> Self {
        Matrix44(unsafe {SkMatrix44::new3(&self.0)})
    }
}

impl PartialEq for Matrix44 {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkMatrix44_Equals(&self.0, &rhs.0)}
    }
}

impl Into<Matrix> for Matrix44 {
    fn into(self) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { C_SkMatrix44_SkMatrix(&self.0, m.native_mut()) };
        m
    }
}

impl ops::Mul for Matrix44 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Matrix44::new();
        unsafe { C_SkMatrix44_Mul(&self.0, &rhs.0, &mut out.0) }
        out
    }
}

impl ops::Mul<Vector4> for Matrix44 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let mut out = SkVector4 { fData: [0.0, 0.0, 0.0, 0.0] };
        unsafe { C_SkMatrix44_MulV4(&self.0, &rhs.into(), &mut out) }
        out.into()
    }
}

impl Matrix44 {

    pub const ROWS : u32 = 4;
    pub const COLUMNS : u32 = 4;

    pub fn new() -> Matrix44 {
        let mut m : SkMatrix44 = unsafe { mem::uninitialized() };
        unsafe { C_SkMatrix44_Construct(&mut m) };
        Matrix44(m)
    }

    pub fn new_identity() -> Matrix44 {
        Matrix44(unsafe{SkMatrix44::new1(SkMatrix44_Identity_Constructor_kIdentity_Constructor)})
    }

    pub fn get_type(&self) -> MatrixTypeMask {
        unsafe { self.0.getType().into() }
    }

    pub fn is_identity(&self) -> bool {
        unsafe { self.0.isIdentity() }
    }

    pub fn is_translate(&self) -> bool {
        unsafe { self.0.isTranslate() }
    }

    pub fn is_scale_translate(&self) -> bool {
        unsafe { self.0.isScaleTranslate() }
    }

    pub fn is_scale(&self) -> bool {
        // linker error:
        // unsafe { self.0.isScale() }
        (self.get_type() & !MatrixTypeMask::Scale).is_empty()
    }

    pub fn has_perspective(&self) -> bool {
        // would cause a linker error
        self.get_type().contains(MatrixTypeMask::Perspective)
    }

    pub fn set_identity(&mut self) {
        unsafe { self.0.setIdentity() }
    }

    pub fn reset(&mut self) {
        // would cause a linker error.
        self.set_identity();
    }

    pub fn get(&self, (row, column): (u32, u32)) -> f32 {
        assert!(row <= Self::ROWS && column <= Self::COLUMNS);
        unsafe { self.0.get(row as _ , column as _) }
    }

    pub fn set(&mut self, (row, column): (u32, u32), value: f32) {
        assert!(row <= Self::ROWS && column <= Self::COLUMNS);
        unsafe { self.0.set(row as _, column as _, value) }
    }

    pub fn as_col_major(&self, floats: &mut [f32; 16]) {
        unsafe { self.0.asColMajorf(floats.as_mut_ptr())}
    }

    pub fn as_row_major(&self, floats: &mut [f32; 16]) {
        unsafe { self.0.asRowMajorf(floats.as_mut_ptr())}
    }

    pub fn set_col_major(&mut self, floats: &[f32; 16]) {
        unsafe { self.0.setColMajorf(floats.as_ptr()) }
    }

    pub fn set_row_major(&mut self, floats: &[f32; 16]) {
        unsafe { self.0.setRowMajorf(floats.as_ptr()) }
    }

    pub fn set_3x3(&mut self,
                   m_00: f32, m_10: f32, m_20: f32,
                   m_01: f32, m_11: f32, m_21: f32,
                   m_02: f32, m_12: f32, m_22: f32) {
        unsafe { self.0.set3x3(m_00, m_10, m_20, m_01, m_11, m_21, m_02, m_12, m_22) }
    }

    pub fn set_3x3_row_major(&mut self, floats: &[f32; 9]) {
        unsafe { self.0.set3x3RowMajorf(floats.as_ptr())}
    }

    pub fn set_translate(&mut self, (dx, dy, dz): (f32, f32, f32)) {
        unsafe { self.0.setTranslate(dx, dy, dz) }
    }

    pub fn pre_translate(&mut self, (dx, dy, dz): (f32, f32, f32)) {
        unsafe { self.0.preTranslate(dx, dy, dz) }
    }

    pub fn post_translate(&mut self, (dx, dy, dz): (f32, f32, f32)) {
        unsafe { self.0.postTranslate(dx, dy, dz) }
    }

    pub fn set_rotate_degrees_about(&mut self, x: f32, y: f32, z: f32, degrees: f32) {
        unimplemented!("linker error");
    }

    pub fn set_rotate_about(&mut self, (x, y, z) : (f32, f32, f32), radians: f32) {
        unsafe { self.0.setRotateAbout(x, y, z, radians)}
    }

    pub fn set_rotate_about_unit(&mut self, (x, y, z) : (f32, f32, f32), radians: f32) {
        unsafe { self.0.setRotateAboutUnit(x, y, z, radians)}
    }

    pub fn set_concat(&mut self, a: &Self, b: &Self) {
        unsafe { self.0.setConcat(&a.0, &b.0)}
    }

    pub fn pre_concat(&mut self, m: &Self) {
        // would cause a linker error
        self.set_concat(&self.clone(), &m)
    }

    pub fn post_concat(&mut self, m: &Self) {
        // would cause a linker error
        self.set_concat(&m, &self.clone())
    }

    pub fn inverse(&self) -> Option<Matrix44> {
        let mut r = Matrix44::new();
        if unsafe { self.0.invert(&mut r.0) } {
            Some(r)
        } else {
            None
        }
    }

    pub fn transpose(&mut self) {
        unsafe { self.0.transpose() }
    }

    pub fn map_scalars(&self, src: &[f32; 4], dst: &mut [f32; 4]) {
        unsafe { self.0.mapScalars(src.as_ptr(), dst.as_mut_ptr()) }
    }
}

pub trait SetPrePostScale<T> {
    fn set_scale(&mut self, v: T);
    fn pre_scale(&mut self, v: T);
    fn post_scale(&mut self, v: T);
}

impl SetPrePostScale<f32> for Matrix44 {

    fn set_scale(&mut self, s: f32) {
        self.set_scale((s, s, s));
    }

    fn pre_scale(&mut self, s: f32) {
        self.pre_scale((s, s, s));
    }

    fn post_scale(&mut self, s: f32) {
        self.post_scale((s, s, s))
    }
}

impl SetPrePostScale<(f32, f32, f32)> for Matrix44 {

    fn set_scale(&mut self, (sx, sy, sz): (f32, f32, f32)) {
        unsafe { self.0.setScale(sx, sy, sz) }
    }

    fn pre_scale(&mut self, (sx, sy, sz): (f32, f32, f32)) {
        unsafe { self.0.preScale(sx, sy, sz) }
    }

    fn post_scale(&mut self, (sx, sy, sz): (f32, f32, f32)) {
        unsafe { self.0.postScale(sx, sy, sz) }
    }
}

pub trait MapScalars<T> {
    fn map_scalars(&self, v: T);
}

impl MapScalars<(&[f32; 4], &mut [f32; 4])> for Matrix44 {
    fn map_scalars(&self, (src, dst): (&[f32; 4], &mut [f32; 4])) {
        unsafe { self.0.mapScalars(src.as_ptr(), dst.as_mut_ptr()) }
    }
}

impl MapScalars<&mut [f32; 4]> for Matrix44 {
    fn map_scalars(&self, vec: &mut [f32; 4]) {
        unsafe { self.0.mapScalars(vec.as_mut_ptr(), vec.as_mut_ptr()) }
    }
}

pub trait Map2<T> {
    fn map2(&self, v: T);
}

impl Map2<(&[f32], &mut[f32])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[f32], &mut[f32])) {
        assert_eq!(0, (src2.len() % 2) & 1);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe { self.0.map2(src2.as_ptr(), (src2.len() / 2).try_into().unwrap(), dst4.as_mut_ptr()) }
    }
}

impl Map2<(&[f64], &mut[f64])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[f64], &mut[f64])) {
        assert_eq!(0, (src2.len() % 2) & 1);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe { self.0.map21(src2.as_ptr(), (src2.len() / 2).try_into().unwrap(), dst4.as_mut_ptr()) }
    }
}

#[test]
fn create_identity_and_clone() {
    Matrix44::new();
    let identity = Matrix44::new_identity();
    identity.clone();
}
