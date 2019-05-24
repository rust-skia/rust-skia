use crate::prelude::*;
use std::{mem, ops};
use crate::{Matrix, scalar, Scalar, Vector3 };
use skia_bindings::{
    SkVector4,
    C_SkMatrix44_Equals,
    C_SkMatrix44_destruct,
    SkMatrix44,
    C_SkMatrix44_SkMatrix,
    C_SkMatrix44_Mul,
    C_SkMatrix44_MulV4,
    C_SkMatrix44_Construct,
    C_SkMatrix44_ConstructIdentity,
    C_SkMatrix44_CopyConstruct
};

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Vector4 {
    x: scalar,
    y: scalar,
    z: scalar,
    w: scalar
}

impl NativeTransmutable<SkVector4> for Vector4 {}

#[test]
fn test_vector4_layout() {
    Vector4::test_layout()
}

impl Default for Vector4 {
    fn default() -> Self {
        (0.0, 0.0, 0.0).into()
    }
}

impl From<(scalar, scalar, scalar, scalar)> for Vector4 {
    fn from((x, y, z, w): (scalar, scalar, scalar, scalar)) -> Self {
        Vector4::new(x, y, z, w)
    }
}

impl From<(scalar, scalar, scalar)> for Vector4 {
    fn from((x, y, z): (scalar, scalar, scalar)) -> Self {
        Vector4::new(x, y, z, None)
    }
}

impl From<[scalar; 4]> for Vector4 {
    fn from(v4: [scalar; 4]) -> Self {
        Vector4::new(v4[0], v4[1], v4[2], v4[3])
    }
}

impl From<[scalar; 3]> for Vector4 {
    fn from(v3: [scalar; 3]) -> Self {
        Vector4::new(v3[0], v3[1], v3[2], None)
    }
}

impl Vector4 {
    pub fn new(x: scalar, y: scalar, z: scalar, w: impl Into<Option<scalar>>) -> Self {
        Vector4 {
            x, y, z, w: w.into().unwrap_or(scalar::ONE)
        }
    }

    pub fn equals(&self, x: scalar, y: scalar, z: scalar, w: impl Into<Option<scalar>>) -> bool {
        *self == Self::new(x, y, z, w)
    }

    pub fn set(&mut self, x: scalar, y: scalar, z: scalar, w: impl Into<Option<scalar>>) {
        *self = Self::new(x, y, z, w)
    }
}

bitflags! {
    pub struct TypeMask: u32 {
        const IDENTITY = skia_bindings::SkMatrix44_TypeMask_kIdentity_Mask as u32;
        const TRANSLATE = skia_bindings::SkMatrix44_TypeMask_kTranslate_Mask as u32;
        const SCALE = skia_bindings::SkMatrix44_TypeMask_kScale_Mask as u32;
        const AFFINE = skia_bindings::SkMatrix44_TypeMask_kAffine_Mask as u32;
        const PERSPECTIVE = skia_bindings::SkMatrix44_TypeMask_kPerspective_Mask as u32;
    }
}

pub type Matrix44 = Handle<SkMatrix44>;

impl NativeDrop for SkMatrix44 {
    fn drop(&mut self) {
        unsafe { C_SkMatrix44_destruct(self) }
    }
}

impl NativeClone for SkMatrix44 {
    fn clone(&self) -> Self {
        // does not link under Linux:
        // unsafe { SkMatrix44::new3(self) }
        unsafe {
            let mut matrix = mem::zeroed();
            C_SkMatrix44_CopyConstruct(&mut matrix, self);
            matrix
        }
    }
}

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

impl Into<Matrix> for Handle<SkMatrix44> {
    fn into(self) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { C_SkMatrix44_SkMatrix(self.native(), m.native_mut()) };
        m
    }
}

impl ops::Mul for Handle<SkMatrix44> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::new();
        unsafe {
            C_SkMatrix44_Mul(self.native(), rhs.native(), out.native_mut())
        }
        out
    }
}

impl ops::Mul<Vector4> for Handle<SkMatrix44> {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let mut out = Vector4::default();
        unsafe {
            C_SkMatrix44_MulV4(self.native(), rhs.native(), out.native_mut())
        }
        out
    }
}

impl Handle<SkMatrix44> {

    pub const ROWS : usize = 4;
    pub const COLUMNS : usize = 4;

    pub fn new() -> Self {
        Self::construct_c(C_SkMatrix44_Construct)
    }

    pub fn new_identity() -> Self {
        Self::construct_c(C_SkMatrix44_ConstructIdentity)
    }

    pub fn get_type(&self) -> TypeMask {
        TypeMask::from_bits_truncate(unsafe {
            self.native().getType()
        } as _)
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
        // TODO: create and use a wrapper function for isScale()
        (self.get_type() & !TypeMask::SCALE).is_empty()
    }

    pub fn has_perspective(&self) -> bool {
        // would cause a linker error
        self.get_type().contains(TypeMask::PERSPECTIVE)
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
        assert!(row < Self::ROWS && column < Self::COLUMNS);
        unsafe { self.native().get(row as _ , column as _) }
    }

    pub fn set(&mut self, (row, column): (usize, usize), value: scalar) -> &mut Self {
        assert!(row < Self::ROWS && column < Self::COLUMNS);
        unsafe { self.native_mut().set(row as _, column as _, value) }
        self
    }

    // TODO: getDouble(), setDouble(), getFloat(), setFloat()?

    pub fn as_col_major(&self, floats: &mut [scalar; 16]) {
        unsafe { self.native().asColMajorf(floats.as_mut_ptr())}
    }

    // TODO: asColMajord()?

    pub fn as_row_major(&self, floats: &mut [scalar; 16]) {
        unsafe { self.native().asRowMajorf(floats.as_mut_ptr())}
    }

    // TODO: asRowMajord()?

    pub fn set_col_major(&mut self, floats: &[scalar; 16]) -> &mut Self {
        unsafe { self.native_mut().setColMajorf(floats.as_ptr()) }
        self
    }

    // TODO: setColMajord()?

    pub fn set_row_major(&mut self, floats: &[scalar; 16]) -> &mut Self {
        unsafe { self.native_mut().setRowMajorf(floats.as_ptr()) }
        self
    }

    // TODO: setRowMajord()?

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

    pub fn set_translate(&mut self, d: impl Into<Vector3>) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().setTranslate(d.x, d.y, d.z) }
        self
    }

    pub fn pre_translate(&mut self, d: impl Into<Vector3>) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().preTranslate(d.x, d.y, d.z) }
        self
    }

    pub fn post_translate(&mut self, d: impl Into<Vector3>) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().postTranslate(d.x, d.y, d.z) }
        self
    }

    // Note: set_scale(), pre_scale() and post_scale() is implemented as a Trait below.

    pub fn set_rotate_degrees_about(&mut self, v: impl Into<Vector3>, degrees: scalar) -> &mut Self {
        unimplemented!()
    }

    pub fn set_rotate_about(&mut self, v: impl Into<Vector3>, radians: scalar) -> &mut Self {
        let v = v.into();
        unsafe {
            self.native_mut().setRotateAbout(v.x, v.y, v.z, radians)
        }
        self
    }

    pub fn set_rotate_about_unit(&mut self, v: impl Into<Vector3>, radians: scalar) -> &mut Self {
        let v = v.into();
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

    #[deprecated(since = "0.11.0", note = "use invert()")]
    #[must_use]
    pub fn inverse(&self) -> Option<Matrix44> {
        self.invert()
    }

    #[must_use]
    pub fn invert(&self) -> Option<Matrix44> {
        let mut r = Matrix44::new();
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

    // map2 is implemented as Trait below.

    pub fn preserves_2d_axis_alignment(&self, epsilon: impl Into<Option<scalar>>) -> bool {
        unsafe {
            self.native().preserves2dAxisAlignment(epsilon.into().unwrap_or(Scalar::NEARLY_ZERO))
        }
    }

    pub fn dump(&self) {
        unsafe {
            self.native().dump()
        }
    }

    pub fn determinant(&self) -> f64 {
        unsafe {
            self.native().determinant()
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
        assert_eq!(0, src2.len() % 2);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe { self.native().map2(src2.as_ptr(), (src2.len() / 2).try_into().unwrap(), dst4.as_mut_ptr()) }
    }
}

impl Map2<(&[f64], &mut[f64])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[f64], &mut[f64])) {
        assert_eq!(0, src2.len() % 2);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe { self.native().map21(src2.as_ptr(), (src2.len() / 2).try_into().unwrap(), dst4.as_mut_ptr()) }
    }
}

#[test]
fn create_identity_and_clone() {
    Matrix44::new();
    let identity = Matrix44::new_identity();
    let _cloned = identity.clone();
}
