use std::mem;
use std::ops::{Index, IndexMut};
use crate::prelude::*;
use crate::skia::{
    Vector,
    Scalar,
    MatrixTypeMask,
    Point,
    Rect,
    Point3,
    Size,
    scalar
};
use rust_skia::{
    SkMatrix_ScaleToFit,
    SkMatrix,
};

pub type MatrixScaleToFit = EnumHandle<SkMatrix_ScaleToFit>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkMatrix_ScaleToFit> {
    pub const Fill: Self = Self(SkMatrix_ScaleToFit::kFill_ScaleToFit);
    pub const Start: Self = Self(SkMatrix_ScaleToFit::kStart_ScaleToFit);
    pub const Center: Self = Self(SkMatrix_ScaleToFit::kCenter_ScaleToFit);
    pub const End: Self = Self(SkMatrix_ScaleToFit::kEnd_ScaleToFit);
}

pub type Matrix = ValueHandle<SkMatrix>;

impl NativePartialEq for SkMatrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { rust_skia::C_SkMatrix_Equals(self, rhs) }
    }
}

pub enum MatrixMember {
    ScaleX = 0,
    SkewX = 1,
    TransX = 2,
    SkewY = 3,
    ScaleY = 4,
    TransY = 5,
    Persp0 = 6,
    Persp1 = 7,
    Persp2 = 8
}

pub enum AffineMatrixMember {
    ScaleX = 0,
    SkewY = 1,
    SkewX = 2,
    ScaleY = 3,
    TransX = 4,
    TransY = 5
}

impl Index<MatrixMember> for ValueHandle<SkMatrix> {
    type Output = scalar;

    fn index(&self, index: MatrixMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<AffineMatrixMember> for ValueHandle<SkMatrix> {
    type Output = scalar;

    fn index(&self, index: AffineMatrixMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<usize> for ValueHandle<SkMatrix> {
    type Output = scalar;

    fn index(&self, index: usize) -> &Self::Output {
        &self.native().fMat[index]
    }
}

impl IndexMut<MatrixMember> for ValueHandle<SkMatrix> {
    fn index_mut(&mut self, index: MatrixMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<AffineMatrixMember> for ValueHandle<SkMatrix> {
    fn index_mut(&mut self, index: AffineMatrixMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<usize> for ValueHandle<SkMatrix> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.native_mut().fMat.index_mut(index)
    }
}

impl Default for ValueHandle<SkMatrix> {
    fn default() -> Self {
        Matrix::new_identity()
    }
}

impl ValueHandle<SkMatrix> {

    pub fn new_scale(sx: scalar, sy: scalar) -> Matrix {
        unsafe { SkMatrix::MakeScale(sx, sy) }.into_handle()
    }

    pub fn new_trans(d: Vector) -> Matrix {
        unsafe { SkMatrix::MakeTrans(d.x, d.y) }.into_handle()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_all(
        scale_x: scalar, skew_x: scalar, trans_x: scalar,
        skew_y: scalar, scale_y: scalar, trans_y: scalar,
        pers_0: scalar, pers_1: scalar, pers_2: scalar) -> Matrix {
        unsafe { SkMatrix::MakeAll(
            scale_x, skew_x, trans_x,
            skew_y, scale_y, trans_y,
            pers_0, pers_1, pers_2)
        }.into_handle()
    }

    pub fn get_type(&self) -> MatrixTypeMask {
        MatrixTypeMask::from_bits_truncate(unsafe {
            self.native().getType()
        } as _)
    }

    pub fn is_identity(&self) -> bool {
        unsafe { self.native().isIdentity() }
    }

    pub fn is_scale_translate(&self) -> bool {
        unsafe { self.native().isScaleTranslate() }
    }

    pub fn is_translate(&self) -> bool {
        // isTranslate does not link
        (self.get_type() & !MatrixTypeMask::Translate).is_empty()
    }

    pub fn rect_stays_rect(&self) -> bool {
        unsafe { self.native().rectStaysRect() }
    }

    pub fn preserves_axis_alignment(&self) -> bool {
        unsafe { self.native().preservesAxisAlignment() }
    }

    pub fn has_perspective(&self) -> bool {
        unsafe { self.native().hasPerspective() }
    }

    pub fn is_similarity(&self) -> bool {
        unsafe { self.native().isSimilarity(scalar::NEARLY_ZERO) }
    }

    pub fn preserves_right_angles(&self) -> bool {
        unsafe { self.native().preservesRightAngles(scalar::NEARLY_ZERO) }
    }

    pub fn get_scale_x(&self) -> scalar {
        unsafe { self.native().getScaleX() }
    }

    pub fn get_scale_y(&self) -> scalar {
        unsafe { self.native().getScaleY() }
    }

    pub fn get_skew_y(&self) -> scalar {
        unsafe { self.native().getSkewY() }
    }

    pub fn get_skew_x(&self) -> scalar {
        unsafe { self.native().getSkewX() }
    }

    pub fn get_translate_x(&self) -> scalar {
        unsafe { self.native().getTranslateX() }
    }

    pub fn get_translate_y(&self) -> scalar {
        unsafe { self.native().getTranslateY() }
    }

    pub fn get_persp_x(&self) -> scalar {
        unsafe { self.native().getPerspX() }
    }

    pub fn get_persp_y(&self) -> scalar {
        unsafe { self.native().getPerspY() }
    }

    pub fn set_scale_x(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::ScaleX] = v;
        self
    }

    pub fn set_scale_y(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::ScaleY] = v;
        self
    }

    pub fn set_skew_y(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::SkewY] = v;
        self
    }

    pub fn set_skew_x(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::SkewX] = v;
        self
    }

    pub fn set_translate_x(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::TransX] = v;
        self
    }

    pub fn set_translate_y(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::TransY] = v;
        self
    }

    pub fn set_persp_x(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::Persp0] = v;
        self
    }

    pub fn set_persp_y(&mut self, v: scalar) -> &mut Self {
        self[MatrixMember::Persp1] = v;
        self
    }

    pub fn get_9(&self, buffer: &mut[scalar; 9]) {
        unsafe { self.native().get9(buffer.as_mut_ptr()) }
    }

    pub fn set_9(&mut self, buffer: &[scalar; 9]) -> &mut Self {
        unsafe { self.native_mut().set9(buffer.as_ptr()) }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    pub fn set_identity(&mut self) -> &mut Self {
        unsafe { self.native_mut().setIdentity() }
        self
    }

    pub fn set_translate(&mut self, v: Vector) -> &mut Self {
        unsafe { self.native_mut().setTranslate(v.x, v.y) }
        self
    }

    pub fn set_scale(&mut self, sx: scalar, sy: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().setScale(sx, sy, pivot.x, pivot.y) }
        self
    }

    pub fn set_rotate(&mut self, degrees: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().setRotate(degrees, pivot.x, pivot.y) }
        self
    }

    pub fn set_sin_cos(&mut self, sin_value: scalar, cos_value: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().setSinCos(sin_value, cos_value, pivot.x, pivot.y) }
        self
    }

    pub fn set_skew(&mut self, kx: scalar, ky: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().setSkew(kx, ky, pivot.x, pivot.y) }
        self
    }

    pub fn set_concat(&mut self, a: &Matrix, b: &Matrix) -> &mut Self {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
        self
    }

    pub fn pre_translate(&mut self, delta: Vector) -> &mut Self {
        unsafe { self.native_mut().preTranslate(delta.x, delta.y) }
        self
    }

    pub fn pre_scale(&mut self, sx: scalar, sy: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().preScale(sx, sy, pivot.x, pivot.y) }
        self
    }

    pub fn pre_rotate(&mut self, degrees: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().preRotate(degrees, pivot.x, pivot.y) }
        self
    }

    pub fn pre_skew(&mut self, kx: scalar, ky: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().preSkew(kx, ky, pivot.x, pivot.y) }
        self
    }

    pub fn pre_concat(&mut self, other: &Matrix) -> &mut Self {
        unsafe { self.native_mut().preConcat(other.native()) }
        self
    }

    pub fn post_translate(&mut self, delta: Vector) -> &mut Self {
        unsafe { self.native_mut().postTranslate(delta.x, delta.y) }
        self
    }

    pub fn post_scale(&mut self, sx: scalar, sy: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().postScale(sx, sy, pivot.x, pivot.y) }
        self
    }

    pub fn post_idiv(&mut self, div_x: i32, div_y: i32) -> bool {
        unsafe { self.native_mut().postIDiv(div_x, div_y) }
    }

    pub fn post_rotate(&mut self, degrees: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().postRotate(degrees, pivot.x, pivot.y) }
        self
    }

    pub fn post_skew(&mut self, kx: scalar, ky: scalar, pivot: Point) -> &mut Self {
        unsafe { self.native_mut().postSkew(kx, ky, pivot.x, pivot.y) }
        self
    }

    pub fn post_concat(&mut self, other: &Matrix) -> &mut Self {
        unsafe { self.native_mut().postConcat(other.native()) }
        self
    }

    pub fn from_rect_to_rect(src: &Rect, dst: &Rect, stf: MatrixScaleToFit) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setRectToRect(&src.into_native(), &dst.into_native(), stf.native().to_owned()) }
            .if_true_some(m)
    }

    pub fn from_poly_to_poly(src: &[Point], dst: &[Point]) -> Option<Matrix> {
        if src.len() != dst.len() {
            return None
        }

        let mut m = Matrix::new_identity();
        unsafe {
            m.native_mut().setPolyToPoly(src.native().as_ptr(), dst.native().as_ptr(), src.len() as _)
        }.if_true_some(m)
    }

    #[warn(unused)]
    pub fn invert(&self) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        unsafe { self.native().invert(m.native_mut()) }
            .if_true_some(m)
    }

    pub fn set_affine_identity(affine: &mut [scalar; 6]) {
        unsafe { SkMatrix::SetAffineIdentity(affine.as_mut_ptr()) }
    }

    #[warn(unused)]
    pub fn as_affine(&mut self) -> Option<[scalar; 6]> {
        let mut affine = [scalar::default(); 6];
        unsafe { self.native_mut().asAffine(affine.as_mut_ptr()) }
            .if_true_some(affine)
    }

    pub fn from_affine(affine: &[scalar; 6]) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setAffine(affine.as_ptr()) }
        m
    }

    pub fn map_points(&self, dst: &mut [Point], src: &[Point]) {
        assert!(dst.len() >= src.len());

        unsafe {
            self.native().mapPoints(
                dst.native_mut().as_mut_ptr(),
                src.native().as_ptr(),
                src.len().try_into().unwrap())
        }
    }

    pub fn map_points_inplace(&self, pts: &mut[Point]) {
        unsafe {
            self.native().mapPoints1(
                pts.native_mut().as_mut_ptr(),
                pts.len().try_into().unwrap())
        }
    }

    pub fn map_homogeneous_points(&self, dst: &mut[Point3], src: &[Point3]) {
        assert!(dst.len() >= src.len());

        unsafe {
            self.native().mapHomogeneousPoints(
                dst.native_mut().as_mut_ptr(),
                src.native().as_ptr(),
                src.len().try_into().unwrap())
        }
    }

    pub fn map_xy(&self, x: scalar, y: scalar) -> Point {
        Point::from_native(unsafe { self.native().mapXY1(x, y) })
    }

    pub fn map_point(&self, point: Point) -> Point {
        Point::from_native(unsafe { self.native().mapXY1(point.x, point.y) })
    }

    pub fn map_vectors(&self, dst: &mut[Vector], src: &[Vector]) {
        assert!(dst.len() >= src.len());
        unsafe {
            self.native().mapVectors(
                dst.native_mut().as_mut_ptr(),
                src.native().as_ptr(),
                src.len().try_into().unwrap())
        }
    }

    pub fn map_vectors_inplace(&self, vecs: &mut[Vector]) {
        unsafe {
            self.native().mapVectors1(
                vecs.native_mut().as_mut_ptr(),
                vecs.len().try_into().unwrap())
        }
    }

    pub fn map_vector(&self, vec: Vector) -> Vector {
        Vector::from_native(unsafe { self.native().mapVector1(vec.x, vec.y) })
    }

    pub fn map_rect(&self, rect: Rect) -> (Rect, bool) {
        let mut rect = rect.into_native();
        let rect_stays_rect = unsafe { self.native().mapRect1(&mut rect) };
        (Rect::from_native(rect), rect_stays_rect)
    }

    pub fn map_rect_to_quad(&self, rect: Rect) -> [Point; 4] {
        let mut points = [Point::default(); 4];
        unsafe { self.native().mapRectToQuad(points.native_mut().as_mut_ptr(), rect.native()) };
        points
    }

    pub fn map_radius(&self, radius: scalar) -> scalar {
        unsafe { self.native().mapRadius(radius) }
    }

    pub fn is_fixed_step_in_x(&self) -> bool {
        unsafe { self.native().isFixedStepInX() }
    }

    pub fn fixed_step_in_x(&self, y: scalar) -> Vector {
        Vector::from_native(unsafe { self.native().fixedStepInX(y) })
    }

    pub fn cheap_equal_to(&self, other: &Matrix) -> bool {
        unsafe { self.native().cheapEqualTo(other.native()) }
    }

    pub fn min_scale(&self) -> scalar {
        unsafe { self.native().getMinScale() }
    }

    pub fn max_scale(&self) -> scalar {
        unsafe { self.native().getMaxScale() }
    }

    pub fn min_max_scales(&self) -> (scalar, scalar) {
        let mut r: [scalar; 2] = Default::default();
        unsafe { self.native().getMinMaxScales(r.as_mut_ptr()) };
        (r[0], r[1])
    }

    pub fn decompose_scale(&self, mut remaining: Option<&mut Matrix>) -> Option<Size> {
        let mut size = Size::default();
        unsafe {
            self.native().decomposeScale(size.native_mut(), remaining.native_ptr_or_null_mut())
        }.if_true_some(size)
    }

    pub fn i() -> &'static Matrix {
        &IDENTITY
    }

    pub fn invalid_matrix() -> &'static Matrix {
        &INVALID
    }

    pub fn concat(a: &Matrix, b: &Matrix) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setConcat(a.native(), b.native()) };
        m
    }

    pub fn dirty_matrix_type_cache(&mut self) {
        // does not link:
        // unsafe { self.native_mut().dirtyMatrixTypeCache() }
        self.native_mut().fTypeMask = 0x80;
    }

    pub fn set_scale_translate(&mut self, sx: scalar, sy: scalar, tx: scalar, ty: scalar) -> &mut Self {
        unsafe { self.native_mut().setScaleTranslate(sx, sy, tx, ty) }
        self
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn new_identity() -> Matrix {
        // SkMatrix contains no C++ types, so this is safe:
        let mut m : SkMatrix = unsafe { mem::zeroed() };
        unsafe { m.reset() };
        Matrix::from_native(m)
    }
}

impl IndexGet for Matrix {}
impl IndexSet for Matrix {}

lazy_static! {
    static ref IDENTITY : Matrix = Matrix::new_identity();
    static ref INVALID : Matrix = unsafe { *SkMatrix::InvalidMatrix() }.into_handle();
}

#[test]
fn test_get_set_trait_compilation() {
    let mut m = Matrix::new_identity();
    let _x = m.get(AffineMatrixMember::ScaleX);
    m.set(AffineMatrixMember::ScaleX, 1.0);
}

#[test]
fn test_tuple_to_vector() {
    let mut m = Matrix::new_identity();
    m.set_translate((10.0, 11.0).into());
    assert_eq!(10.0, m.get_translate_x());
    assert_eq!(11.0, m.get_translate_y());
}