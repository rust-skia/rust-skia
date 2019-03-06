use crate::prelude::*;
use crate::skia::scalar;
use rust_skia::SkPoint3;
use std::ops::{Add, Sub, Neg};

pub type Vector3 = Point3;
pub type Color3f = Point3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point3 {
    pub x: scalar,
    pub y: scalar,
    pub z: scalar
}

impl NativeTransmutable<SkPoint3> for Point3 {}

#[test]
fn test_layout() {
    Point3::test_layout()
}

impl Default for Point3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<(f32, f32, f32)> for Point3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
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

impl Sub for Point3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Point3 {
    pub fn new(x: scalar, y: scalar, z: scalar) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> scalar {
        // does not link:
        // unsafe { self.native().length() }
        unsafe { SkPoint3::Length(self.x, self.y, self.z) }
    }

    #[warn(unused)]
    pub fn normalized(&self) -> Option<Self> {
        let mut normalized = self.clone();
        unsafe { normalized.native_mut().normalize() }
            .if_true_some(normalized)
    }

    #[warn(unused)]
    pub fn scaled(&self, scale: scalar) -> Self {
        // scale() does not link.
        Self::from_native(unsafe { self.native().makeScale(scale) })
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn dot_product(a: Self, b: Self) -> scalar {
        unsafe { SkPoint3::DotProduct(a.native(), b.native()) }
    }

    pub fn cross_product(a: Self, b: Self) -> Point3 {
        Self::from_native(unsafe {
            SkPoint3::CrossProduct(a.native(), b.native())
        })
    }
}