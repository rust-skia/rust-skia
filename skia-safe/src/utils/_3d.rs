//! TODO: The original Skia function names in this module are prefixed with Sk3, but we
//!       export them without a prefix.
//!       Should we place them into a module?
use crate::prelude::*;
use crate::{Matrix44, Point, Point3};
use skia_bindings::{Sk3LookAt, Sk3MapPts, Sk3Perspective, SkMatrix44};

pub fn look_at(
    eye: impl Into<Point3>,
    center: impl Into<Point3>,
    up: impl Into<Point3>,
) -> Matrix44 {
    let mut m4 = Matrix44::new();
    unsafe {
        Sk3LookAt(
            m4.native_mut(),
            eye.into().native(),
            center.into().native(),
            up.into().native(),
        );
    }
    m4
}

pub fn perspective(near: f32, far: f32, angle: f32) -> Matrix44 {
    let mut m4 = Matrix44::new();
    unsafe {
        Sk3Perspective(m4.native_mut(), near, far, angle);
    }
    m4
}

pub fn map_points(dst: &mut [Point], m4: &Matrix44, src: &[Point3]) {
    assert_eq!(src.len(), dst.len());
    unsafe {
        Sk3MapPts(
            dst.native_mut().as_mut_ptr(),
            m4.native(),
            src.native().as_ptr(),
            src.len().try_into().unwrap(),
        )
    }
}

impl Handle<SkMatrix44> {
    pub fn look_at(
        eye: impl Into<Point3>,
        center: impl Into<Point3>,
        up: impl Into<Point3>,
    ) -> Matrix44 {
        look_at(eye, center, up)
    }

    pub fn perspective(near: f32, far: f32, angle: f32) -> Matrix44 {
        perspective(near, far, angle)
    }

    pub fn map_points(&self, src: &[Point3], dst: &mut [Point]) -> &Self {
        map_points(dst, self, src);
        self
    }
}
