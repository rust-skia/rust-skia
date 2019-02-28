use std::{
    ops::Index,
    iter,
    mem,
    ops::IndexMut,
    ptr
};
use crate::prelude::*;
use crate::skia::{
    Vector,
    Scalar,
    MatrixTypeMask,
    Point,
    Rect,
    Point3,
    Size
};
use rust_skia::{
    SkPoint,
    SkMatrix_ScaleToFit,
    SkMatrix,
    SkPoint3,
    SkSize
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

impl Index<MatrixMember> for Matrix {
    type Output = f32;

    fn index(&self, index: MatrixMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<AffineMatrixMember> for Matrix {
    type Output = f32;

    fn index(&self, index: AffineMatrixMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.native().fMat[index]
    }
}

impl IndexMut<MatrixMember> for Matrix {
    fn index_mut(&mut self, index: MatrixMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<AffineMatrixMember> for Matrix {
    fn index_mut(&mut self, index: AffineMatrixMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.native_mut().fMat.index_mut(index)
    }
}

impl Matrix {

    pub fn new_scale(sx: f32, sy: f32) -> Matrix {
        unsafe { SkMatrix::MakeScale(sx, sy) }.into_handle()
    }

    pub fn new_trans(dx: f32, dy: f32) -> Matrix {
        unsafe { SkMatrix::MakeTrans(dx, dy) }.into_handle()
    }

    pub fn new_all(
        scale_x: f32, skew_x: f32, trans_x: f32,
        skew_y: f32, scale_y: f32, trans_y: f32,
        pers_0: f32, pers_1: f32, pers_2: f32) -> Matrix {
        unsafe { SkMatrix::MakeAll(
            scale_x, skew_x, trans_x,
            skew_y, scale_y, trans_y,
            pers_0, pers_1, pers_2)
        }.into_handle()
    }

    pub fn get_type(&self) -> MatrixTypeMask {
        unsafe { self.native().getType() }.into()
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
        unsafe { self.native().isSimilarity(f32::NEARLY_ZERO) }
    }

    pub fn preserves_right_angles(&self) -> bool {
        unsafe { self.native().preservesRightAngles(f32::NEARLY_ZERO) }
    }

    pub fn get_scale_x(&self) -> f32 {
        unsafe { self.native().getScaleX() }
    }

    pub fn get_scale_y(&self) -> f32 {
        unsafe { self.native().getScaleY() }
    }

    pub fn get_skew_y(&self) -> f32 {
        unsafe { self.native().getSkewY() }
    }

    pub fn get_skew_x(&self) -> f32 {
        unsafe { self.native().getSkewX() }
    }

    pub fn get_translate_x(&self) -> f32 {
        unsafe { self.native().getTranslateX() }
    }

    pub fn get_translate_y(&self) -> f32 {
        unsafe { self.native().getTranslateY() }
    }

    pub fn get_persp_x(&self) -> f32 {
        unsafe { self.native().getPerspX() }
    }

    pub fn get_persp_y(&self) -> f32 {
        unsafe { self.native().getPerspY() }
    }

    pub fn set_scale_x(&mut self, v: f32) {
        self[MatrixMember::ScaleX] = v;
    }

    pub fn set_scale_y(&mut self, v: f32) {
        self[MatrixMember::ScaleY] = v;
    }

    pub fn set_skew_y(&mut self, v: f32) {
        self[MatrixMember::SkewY] = v;
    }

    pub fn set_skew_x(&mut self, v: f32) {
        self[MatrixMember::SkewX] = v;
    }

    pub fn set_translate_x(&mut self, v: f32) {
        self[MatrixMember::TransX] = v;
    }

    pub fn set_translate_y(&mut self, v: f32) {
        self[MatrixMember::TransY] = v;
    }

    pub fn set_persp_x(&mut self, v: f32) {
        self[MatrixMember::Persp0] = v;
    }

    pub fn set_persp_y(&mut self, v: f32) {
        self[MatrixMember::Persp1] = v;
    }

    pub fn get_9(&self, buffer: &mut[f32; 9]) {
        unsafe { self.native().get9(buffer.as_mut_ptr()) }
    }

    pub fn set_9(&mut self, buffer: &[f32; 9]) {
        unsafe { self.native_mut().set9(buffer.as_ptr()) }
    }

    pub fn reset(&mut self) {
        unsafe { self.native_mut().reset() }
    }

    pub fn set_identity(&mut self) {
        unsafe { self.native_mut().setIdentity() }
    }

    pub fn set_translate(&mut self, v: Vector) {
        unsafe { self.native_mut().setTranslate(v.x, v.y) }
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setScale(sx, sy, pivot.x, pivot.y) }
    }

    pub fn set_rotate(&mut self, degrees: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setRotate(degrees, pivot.x, pivot.y) }
    }

    pub fn set_sin_cos(&mut self, sin_value: f32, cos_value: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setSinCos(sin_value, cos_value, pivot.x, pivot.y) }
    }

    pub fn set_skew(&mut self, kx: f32, ky: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setSkew(kx, ky, pivot.x, pivot.y) }
    }

    pub fn set_concat(&mut self, a: &Matrix, b: &Matrix) {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
    }

    pub fn pre_translate(&mut self, delta: Vector) {
        unsafe { self.native_mut().preTranslate(delta.x, delta.y) }
    }

    pub fn pre_scale(&mut self, sx: f32, sy: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().preScale(sx, sy, pivot.x, pivot.y) }
    }

    pub fn pre_rotate(&mut self, degrees: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().preRotate(degrees, pivot.x, pivot.y) }
    }

    pub fn pre_skew(&mut self, kx: f32, ky: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().preSkew(kx, ky, pivot.x, pivot.y) }
    }

    pub fn pre_concat(&mut self, other: &Matrix) {
        unsafe { self.native_mut().preConcat(other.native()) }
    }

    pub fn post_translate(&mut self, delta: Vector) {
        unsafe { self.native_mut().postTranslate(delta.x, delta.y) }
    }

    pub fn post_scale(&mut self, sx: f32, sy: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().postScale(sx, sy, pivot.x, pivot.y) }
    }

    pub fn post_idiv(&mut self, div_x: i32, div_y: i32) -> bool {
        unsafe { self.native_mut().postIDiv(div_x, div_y) }
    }

    pub fn post_rotate(&mut self, degrees: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().postRotate(degrees, pivot.x, pivot.y) }
    }

    pub fn post_skew(&mut self, kx: f32, ky: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().postSkew(kx, ky, pivot.x, pivot.y) }
    }

    pub fn post_concat(&mut self, other: &Matrix) {
        unsafe { self.native_mut().postConcat(other.native()) }
    }

    pub fn new_rect_to_rect(src: &Rect, dst: &Rect, stf: MatrixScaleToFit) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        if unsafe { m.native_mut().setRectToRect(&src.into_native(), &dst.into_native(), stf.native().to_owned()) } {
            Some(m)
        } else {
            None
        }
    }

    pub fn new_poly_to_poly(src: &[Point], dst: &[Point]) -> Option<Matrix> {
        if src.len() != dst.len() {
            return None
        }

        let src : Vec<SkPoint> = src.to_native();
        let dst : Vec<SkPoint> = dst.to_native();

        let mut m = Matrix::new_identity();
        if unsafe { m.native_mut().setPolyToPoly(src.as_ptr(), dst.as_ptr(), src.len() as _) } {
            Some(m)
        } else {
            None
        }
    }

    #[warn(unused)]
    pub fn invert(&self) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        if unsafe { self.native().invert(m.native_mut()) } {
            Some(m)
        } else {
            None
        }
    }

    pub fn set_affine_identity(affine: &mut [f32; 6]) {
        unsafe { SkMatrix::SetAffineIdentity(affine.as_mut_ptr()) }
    }

    #[warn(unused)]
    pub fn as_affine(&mut self) -> Option<[f32; 6]> {
        let mut affine = [0.0; 6];
        if unsafe { self.native_mut().asAffine(affine.as_mut_ptr()) } {
            Some(affine)
        } else {
            None
        }
    }

    pub fn new_affine(affine: &[f32; 6]) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setAffine(affine.as_ptr()) }
        m
    }

    pub fn map_points(&self, dst: &mut [Point], src: &[Point]) {
        assert!(dst.len() >= src.len());

        let src_native = src.to_native();
        let mut dst_native : Vec<SkPoint> = iter::repeat(SkPoint { fX: 0.0, fY: 0.0 }).take(src.len()).collect();
        unsafe {
            self.native().mapPoints(
                dst_native.as_mut_ptr(),
                src_native.as_ptr(),
                src.len().try_into().unwrap())
        }
        dst_native
            .iter()
            .enumerate()
            .for_each(|(i, p)| dst[i] = Point::from_native(*p));
    }

    pub fn map_points_inplace(&self, pts: &mut[Point]) {
        let mut pts_native = pts.to_native();
        unsafe {
            self.native().mapPoints1(
                pts_native.as_mut_ptr(),
                pts_native.len().try_into().unwrap())
        }
        pts_native
            .iter()
            .enumerate()
            .for_each(|(i, p)| pts[i] = Point::from_native(*p));
    }

    pub fn map_homogeneous_points(&self, dst: &mut[Point3], src: &[Point3]) {
        assert!(dst.len() >= src.len());

        let src_native : Vec<SkPoint3> = src.to_native();
        let mut dst_native : Vec<SkPoint3> = iter::repeat(SkPoint3 { fX: 0.0, fY: 0.0, fZ: 0.0}).take(src.len()).collect();

        unsafe {
            self.native().mapHomogeneousPoints(
                dst_native.as_mut_ptr(),
                src_native.as_ptr(),
                src.len().try_into().unwrap())
        }
        dst_native
            .iter()
            .enumerate()
            .for_each(|(i, p)| dst[i] = Point3::from_native(*p));
    }

    pub fn map_xy(&self, x: f32, y: f32) -> Point {
        Point::from_native(unsafe { self.native().mapXY1(x, y) })
    }

    pub fn map_vectors(&self, dst: &mut[Vector], src: &[Vector]) {
        assert!(dst.len() >= src.len());

        let src_native = src.to_native();
        let mut dst_native : Vec<SkPoint> = iter::repeat(SkPoint { fX: 0.0, fY: 0.0 }).take(src.len()).collect();
        unsafe {
            self.native().mapVectors(
                dst_native.as_mut_ptr(),
                src_native.as_ptr(),
                src.len().try_into().unwrap())
        }
        dst_native
            .iter()
            .enumerate()
            .for_each(|(i, p)| dst[i] = Point::from_native(*p));
    }

    pub fn map_vectors_inplace(&self, vecs: &mut[Vector]) {
        let mut vecs_native = vecs.to_native();
        unsafe {
            self.native().mapVectors1(
                vecs_native.as_mut_ptr(),
                vecs_native.len().try_into().unwrap())
        }
        vecs_native
            .iter()
            .enumerate()
            .for_each(|(i, p)| vecs[i] = Point::from_native(*p));
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
        let mut points = [SkPoint { fX: 0.0, fY: 0.0 }; 4];
        unsafe { self.native().mapRectToQuad(points.as_mut_ptr(), &rect.into_native()) };
        let mut r = [Point::new(0.0, 0.0); 4];
        points
            .iter()
            .enumerate()
            .for_each(|(i, p)| r[i] = Point::from_native(*p));
        r
    }

    pub fn map_radius(&self, radius: f32) -> f32 {
        unsafe { self.native().mapRadius(radius) }
    }

    pub fn is_fixed_step_in_x(&self) -> bool {
        unsafe { self.native().isFixedStepInX() }
    }

    pub fn fixed_step_in_x(&self, y: f32) -> Vector {
        Vector::from_native(unsafe { self.native().fixedStepInX(y) })
    }

    pub fn cheap_equal_to(&self, other: &Matrix) -> bool {
        unsafe { self.native().cheapEqualTo(other.native()) }
    }

    pub fn min_scale(&self) -> f32 {
        unsafe { self.native().getMinScale() }
    }

    pub fn max_scale(&self) -> f32 {
        unsafe { self.native().getMaxScale() }
    }

    pub fn min_max_scales(&self) -> (f32, f32) {
        let mut r: [f32; 2] = Default::default();
        unsafe { self.native().getMinMaxScales(r.as_mut_ptr()) };
        (r[0], r[1])
    }

    pub fn decompose_scale(&self, remaining: Option<&mut SkMatrix>) -> Option<Size> {
        let mut size = SkSize { fWidth: 0.0, fHeight: 0.0 };
        let remaining =
            match remaining {
                Some(remaining) => remaining as _,
                None => ptr::null_mut()
            };
        if unsafe { self.native().decomposeScale(&mut size, remaining) } {
            Some (Size::from_native(size))
        } else {
            None
        }
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

    pub fn set_scale_translate(&mut self, sx: f32, sy: f32, tx: f32, ty: f32) {
        unsafe { self.native_mut().setScaleTranslate(sx, sy, tx, ty) }
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
    let x = m.get(AffineMatrixMember::ScaleX);
    m.set(AffineMatrixMember::ScaleX, 1.0);
}

fn test_tuple_to_vector() {
    let mut m = Matrix::new_identity();
    m.set_translate((10.0, 10.0).lift())
}