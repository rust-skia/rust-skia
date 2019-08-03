use crate::prelude::*;
use crate::scalar;
use skia_bindings::SkPoint3;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub type Vector3 = Point3;
pub type Color3f = Point3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Point3 {
    pub x: scalar,
    pub y: scalar,
    pub z: scalar,
}

impl NativeTransmutable<SkPoint3> for Point3 {}

#[test]
fn test_point3_layout() {
    Point3::test_layout()
}

impl From<(scalar, scalar, scalar)> for Point3 {
    fn from((x, y, z): (scalar, scalar, scalar)) -> Self {
        Self::new(x, y, z)
    }
}

impl Neg for Point3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Point3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Point3) {
        *self = *self + rhs;
    }
}

impl Sub for Point3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Point3 {
    fn sub_assign(&mut self, rhs: Point3) {
        *self = *self - rhs;
    }
}

impl Point3 {
    pub const fn new(x: scalar, y: scalar, z: scalar) -> Self {
        Self { x, y, z }
    }

    pub fn set(&mut self, x: scalar, y: scalar, z: scalar) {
        *self = Self::new(x, y, z);
    }

    pub fn length_xyz(x: scalar, y: scalar, z: scalar) -> scalar {
        unsafe { SkPoint3::Length(x, y, z) }
    }

    pub fn length(&self) -> scalar {
        // does not link:
        // unsafe { self.native().length() }
        unsafe { SkPoint3::Length(self.x, self.y, self.z) }
    }

    pub fn normalize(&mut self) -> bool {
        unsafe { self.native_mut().normalize() }
    }

    #[must_use]
    pub fn normalized(&self) -> Option<Self> {
        let mut normalized = *self;
        unsafe { normalized.native_mut().normalize() }.if_true_some(normalized)
    }

    // TODO: with_scale()?
    #[must_use]
    pub fn scaled(&self, scale: scalar) -> Self {
        // scale() does not link.
        Self::from_native(unsafe { self.native().makeScale(scale) })
    }

    pub fn scale(&mut self, value: scalar) {
        *self = self.scaled(value);
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn dot_product(a: Self, b: Self) -> scalar {
        unsafe { SkPoint3::DotProduct(a.native(), b.native()) }
    }

    pub fn dot(&self, vec: Self) -> scalar {
        unsafe { self.native().dot(vec.native()) }
    }

    pub fn cross_product(a: Self, b: Self) -> Point3 {
        Self::from_native(unsafe { SkPoint3::CrossProduct(a.native(), b.native()) })
    }

    pub fn cross(&self, vec: Self) -> Point3 {
        Self::from_native(unsafe { self.native().cross(vec.native()) })
    }
}
