#![allow(deprecated)]
use crate::{prelude::*, scalar, Matrix, Scalar, Vector3};
use skia_bindings::{self as sb, SkMatrix44, SkVector4};
use std::{fmt, ops};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[deprecated(since = "0.30.0", note = "use V4 instead")]
pub struct Vector4 {
    x: scalar,
    y: scalar,
    z: scalar,
    w: scalar,
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
            x,
            y,
            z,
            w: w.into().unwrap_or(scalar::ONE),
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
    pub struct TypeMask: u8 {
        const IDENTITY = sb::SkMatrix44_kIdentity_Mask as _;
        const TRANSLATE = sb::SkMatrix44_kTranslate_Mask as _;
        const SCALE = sb::SkMatrix44_kScale_Mask as _;
        const AFFINE = sb::SkMatrix44_kAffine_Mask as _;
        const PERSPECTIVE = sb::SkMatrix44_kPerspective_Mask as _;
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
#[deprecated(since = "0.30.0", note = "use M44 instead")]
pub struct Matrix44(SkMatrix44);

impl NativeTransmutable<SkMatrix44> for Matrix44 {}
#[test]
fn test_matrix44_layout() {
    Matrix44::test_layout()
}

impl PartialEq for Matrix44 {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkMatrix44_Equals(self.native(), rhs.native()) }
    }
}

impl Default for Matrix44 {
    fn default() -> Self {
        Matrix44::new_identity()
    }
}

impl fmt::Debug for Matrix44 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matrix44")
            .field("mat", &self.native().fMat)
            .field("type", &self.get_type())
            .finish()
    }
}

impl From<Matrix44> for Matrix {
    fn from(m44: Matrix44) -> Self {
        let mut m = Matrix::new_identity();
        unsafe { sb::C_SkMatrix44_SkMatrix(m44.native(), m.native_mut()) };
        m
    }
}

impl ops::Mul for Matrix44 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::new_identity();
        unsafe { sb::C_SkMatrix44_Mul(self.native(), rhs.native(), out.native_mut()) }
        out
    }
}

impl ops::Mul<Vector4> for Matrix44 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let mut out = Vector4::default();
        unsafe { sb::C_SkMatrix44_MulV4(self.native(), rhs.native(), out.native_mut()) }
        out
    }
}

impl Matrix44 {
    pub const ROWS: usize = 4;
    pub const COLUMNS: usize = 4;

    pub fn new_identity() -> Self {
        Matrix44::construct(|matrix| unsafe {
            sb::C_SkMatrix44_ConstructIdentity(matrix);
        })
    }

    pub fn new_nan() -> Self {
        Matrix44::construct(|matrix| unsafe {
            sb::C_SkMatrix44_ConstructNaN(matrix);
        })
    }

    pub fn get_type(&self) -> TypeMask {
        TypeMask::from_bits_truncate(self.native().fTypeMask)
    }

    pub fn is_identity(&self) -> bool {
        self.get_type() == TypeMask::IDENTITY
    }

    pub fn is_translate(&self) -> bool {
        (self.get_type() & !TypeMask::TRANSLATE).is_empty()
    }

    pub fn is_scale_translate(&self) -> bool {
        (self.get_type() & !(TypeMask::SCALE | TypeMask::TRANSLATE)).is_empty()
    }

    pub fn is_scale(&self) -> bool {
        (self.get_type() & !TypeMask::SCALE).is_empty()
    }

    pub fn has_perspective(&self) -> bool {
        self.get_type().contains(TypeMask::PERSPECTIVE)
    }

    pub fn set_identity(&mut self) -> &mut Self {
        unsafe { self.native_mut().setIdentity() }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.set_identity()
    }

    pub fn get(&self, (row, col): (usize, usize)) -> scalar {
        self.native().fMat[col][row]
    }

    pub fn set(&mut self, (row, col): (usize, usize), value: scalar) -> &mut Self {
        self.native_mut().fMat[col][row] = value;
        self
    }

    // TODO: getDouble(), setDouble(), getFloat(), setFloat()?

    pub fn as_col_major(&self, floats: &mut [scalar; 16]) {
        unsafe { self.native().asColMajorf(floats.as_mut_ptr()) }
    }

    // TODO: asColMajord()?

    pub fn as_row_major(&self, floats: &mut [scalar; 16]) {
        unsafe { self.native().asRowMajorf(floats.as_mut_ptr()) }
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
    pub fn set_3x3(
        &mut self,
        m_00: scalar,
        m_10: scalar,
        m_20: scalar,
        m_01: scalar,
        m_11: scalar,
        m_21: scalar,
        m_02: scalar,
        m_12: scalar,
        m_22: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .set3x3(m_00, m_10, m_20, m_01, m_11, m_21, m_02, m_12, m_22)
        }
        self
    }

    pub fn set_3x3_row_major(&mut self, floats: &[scalar; 9]) -> &mut Self {
        unsafe { self.native_mut().set3x3RowMajorf(floats.as_ptr()) }
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_4x4(
        &mut self,
        m_00: scalar,
        m_10: scalar,
        m_20: scalar,
        m_30: scalar,
        m_01: scalar,
        m_11: scalar,
        m_21: scalar,
        m_31: scalar,
        m_02: scalar,
        m_12: scalar,
        m_22: scalar,
        m_32: scalar,
        m_03: scalar,
        m_13: scalar,
        m_23: scalar,
        m_33: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut().set4x4(
                m_00, m_10, m_20, m_30, m_01, m_11, m_21, m_31, m_02, m_12, m_22, m_32, m_03, m_13,
                m_23, m_33,
            )
        };
        self
    }

    pub fn set_translate(&mut self, d: impl Into<Vector3>) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().setTranslate(d.x, d.y, d.z);
        }
        self
    }

    pub fn pre_translate(&mut self, d: impl Into<Vector3>) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().preTranslate(d.x, d.y, d.z);
        }
        self
    }

    pub fn post_translate(&mut self, d: impl Into<Vector3>) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().postTranslate(d.x, d.y, d.z);
        }
        self
    }

    // Note: set_scale(), pre_scale() and post_scale() are implemented as a Trait below.

    pub fn set_rotate_degrees_about(
        &mut self,
        v: impl Into<Vector3>,
        degrees: scalar,
    ) -> &mut Self {
        self.set_rotate_about(v, degrees * std::f32::consts::PI / 180.0)
    }

    pub fn set_rotate_about(&mut self, v: impl Into<Vector3>, radians: scalar) -> &mut Self {
        let v = v.into();
        unsafe { self.native_mut().setRotateAbout(v.x, v.y, v.z, radians) }
        self
    }

    pub fn set_rotate_about_unit(&mut self, v: impl Into<Vector3>, radians: scalar) -> &mut Self {
        let v = v.into();
        unsafe { self.native_mut().setRotateAboutUnit(v.x, v.y, v.z, radians) }
        self
    }

    pub fn set_concat(&mut self, a: &Self, b: &Self) -> &mut Self {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
        self
    }

    pub fn pre_concat(&mut self, m: &Self) -> &mut Self {
        self.set_concat(&self.clone(), &m)
    }

    pub fn post_concat(&mut self, m: &Self) -> &mut Self {
        self.set_concat(&m, &self.clone())
    }

    #[must_use]
    pub fn invert(&self) -> Option<Matrix44> {
        let mut r = Matrix44::default();
        unsafe { self.native().invert(r.native_mut()) }.if_true_some(r)
    }

    pub fn transpose(&mut self) -> &mut Self {
        unsafe { self.native_mut().transpose() }
        self
    }

    pub fn map_scalars(&self, src: &[scalar; 4], dst: &mut [scalar; 4]) {
        unsafe { self.native().mapScalars(src.as_ptr(), dst.as_mut_ptr()) }
    }

    // map2 is implemented as Trait below.

    pub fn preserves_2d_axis_alignment(&self, epsilon: impl Into<Option<scalar>>) -> bool {
        unsafe {
            self.native()
                .preserves2dAxisAlignment(epsilon.into().unwrap_or(Scalar::NEARLY_ZERO))
        }
    }

    pub fn dump(&self) {
        unsafe { self.native().dump() }
    }

    pub fn determinant(&self) -> f64 {
        unsafe { self.native().determinant() }
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
        unsafe {
            self.native_mut().setScale(sx, sy, sz);
        }
        self
    }

    fn pre_scale(&mut self, (sx, sy, sz): (scalar, scalar, scalar)) -> &mut Self {
        unsafe {
            self.native_mut().preScale(sx, sy, sz);
        }
        self
    }

    fn post_scale(&mut self, (sx, sy, sz): (scalar, scalar, scalar)) -> &mut Self {
        unsafe {
            self.native_mut().postScale(sx, sy, sz);
        }
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

impl Map2<(&[scalar], &mut [scalar])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[scalar], &mut [scalar])) {
        assert_eq!(0, src2.len() % 2);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe {
            self.native().map2(
                src2.as_ptr(),
                (src2.len() / 2).try_into().unwrap(),
                dst4.as_mut_ptr(),
            )
        }
    }
}

impl Map2<(&[f64], &mut [f64])> for Matrix44 {
    fn map2(&self, (src2, dst4): (&[f64], &mut [f64])) {
        assert_eq!(0, src2.len() % 2);
        assert_eq!(src2.len() * 2, dst4.len());
        unsafe {
            self.native().map21(
                src2.as_ptr(),
                (src2.len() / 2).try_into().unwrap(),
                dst4.as_mut_ptr(),
            )
        }
    }
}

#[test]
fn create_identity() {
    Matrix44::new_identity();
    let _identity = Matrix44::new_identity();
}
