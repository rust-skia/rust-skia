//! TODO: The original Skia function names in this module are prefixed with Sk3, but we
//!       export them without a prefix.
//!       Should we place them into a module?
#![allow(deprecated)]
use crate::{Matrix44, Point, Point3, M44, V3};

#[deprecated(since = "0.29.0", note = "use M44::look_at()")]
pub fn look_at(
    eye: impl Into<Point3>,
    center: impl Into<Point3>,
    up: impl Into<Point3>,
) -> Matrix44 {
    let eye = eye.into().into();
    let center = center.into().into();
    let up = up.into().into();
    M44::look_at(&eye, &center, &up).to_matrix44()
}

#[deprecated(since = "0.29.0", note = "use M44::perspective()")]
pub fn perspective(near: f32, far: f32, angle: f32) -> Matrix44 {
    M44::perspective(near, far, angle).to_matrix44()
}

#[deprecated(since = "0.29.0", note = "use M44::map()")]
pub fn map_points(dst: &mut [Point], m4: &Matrix44, src: &[Point3]) {
    assert_eq!(src.len(), dst.len());

    let m44: M44 = m4.into();
    src.iter().enumerate().for_each(|(i, p)| {
        let v3: V3 = (*p).into();
        let mapped = &m44 * v3;
        dst[i] = Point::new(mapped.x, mapped.y);
    });
}

impl Matrix44 {
    #[deprecated(since = "0.29.0", note = "use M44::look_at()")]
    pub fn look_at(
        eye: impl Into<Point3>,
        center: impl Into<Point3>,
        up: impl Into<Point3>,
    ) -> Matrix44 {
        look_at(eye, center, up)
    }

    #[deprecated(since = "0.29.0", note = "use M44::perspective()")]
    pub fn perspective(near: f32, far: f32, angle: f32) -> Matrix44 {
        perspective(near, far, angle)
    }

    #[deprecated(since = "0.29.0", note = "use M44::map() or M44::Mul")]
    pub fn map_points(&self, src: &[Point3], dst: &mut [Point]) -> &Self {
        map_points(dst, self, src);
        self
    }
}

impl From<Point3> for V3 {
    fn from(p: Point3) -> Self {
        Self::new(p.x, p.y, p.z)
    }
}
