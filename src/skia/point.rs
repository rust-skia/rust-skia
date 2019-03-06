use crate::prelude::*;
use crate::skia::scalar;
use rust_skia::{
    SkIPoint,
    SkPoint
};
use std::ops::{
    Sub,
    Add,
    Neg,
    Mul
};
use crate::skia::ISize;
use crate::skia::Size;

pub type IVector = IPoint;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct IPoint {
    pub x: i32,
    pub y: i32
}

impl NativeTransmutable<SkIPoint> for IPoint {}

#[test]
fn test_layout() {
    IPoint::test_layout()
}

impl Neg for IPoint {
    type Output = IPoint;
    fn neg(self) -> Self::Output {
        IPoint::new(-self.x, -self.y)
    }
}

impl Add for IPoint {
    type Output = IPoint;
    fn add(self, rhs: Self) -> Self::Output {
        IPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<ISize> for IPoint {
    type Output = IPoint;
    fn add(self, rhs: ISize) -> Self::Output {
        IPoint::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl Sub for IPoint {
    type Output = IPoint;
    fn sub(self, rhs: Self) -> Self::Output {
        IPoint::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<ISize> for IPoint {
    type Output = IPoint;
    fn sub(self, rhs: ISize) -> Self::Output {
        IPoint::new(self.x - rhs.width, self.y - rhs.height)
    }
}

impl IPoint {
    pub fn new(x: i32, y: i32) -> IPoint {
        IPoint { x, y }
    }

    pub fn is_zero(&self) -> bool {
        // does not link:
        // unsafe { self.native().isZero() }
        (self.x | self.y) == 0
    }
}

pub type Vector = Point;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Point {
    pub x: scalar,
    pub y: scalar
}

impl NativeTransmutable<SkPoint> for Point {}

#[test]
fn point_layout() {
    Point::test_layout()
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Size> for Point {
    type Output = Self;
    fn add(self, rhs: Size) -> Self {
        Point::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Size> for Point {
    type Output = Self;
    fn sub(self, rhs: Size) -> Self {
        Point::new(self.x - rhs.width, self.y - rhs.height)
    }
}

impl Mul<scalar> for Point {
    type Output = Self;
    fn mul(self, rhs: scalar) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Point {
    pub fn new(x: scalar, y: scalar) -> Self {
        Point { x, y }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn length(&self) -> scalar {
        unsafe { self.native().length() }
    }

    pub fn distance_to_origin(&self) -> scalar {
        self.length()
    }

    #[warn(unused)]
    pub fn normalized(&self) -> Option<Self> {
        let mut cloned = self.clone();
        unsafe { cloned.native_mut().normalize() }
            .if_true_some(cloned)
    }

    #[warn(unused)]
    pub fn with_length(&self, length: scalar) -> Option<Self> {
        let mut cloned = self.clone();
        unsafe { cloned.native_mut().setLength(length) }
            .if_true_some(cloned)
    }

    #[warn(unused)]
    pub fn scaled(&self, scale: scalar) -> Self {
        let mut cloned = self.clone();
        unsafe { cloned.native_mut().scale1(scale) }
        cloned
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn distance(a: &Self, b: &Self) -> scalar {
        unsafe { SkPoint::Distance(a.native(), b.native()) }
    }

    pub fn dot_product(a: &Self, b: &Self) -> scalar {
        unsafe { SkPoint::DotProduct(a.native(), b.native()) }
    }

    pub fn cross_product(a: &Self, b: &Self) -> scalar {
        unsafe { SkPoint::CrossProduct(a.native(), b.native() )}
    }
}

//
// From
//

impl From<(i32, i32)> for IPoint {
    fn from(source: (i32, i32)) -> Self {
        IPoint::new(source.0, source.1)
    }
}

impl From<(scalar, scalar)> for Point {
    fn from(source: (scalar, scalar)) -> Self {
        Point::new(source.0, source.1)
    }
}

impl From<IPoint> for Point {
    fn from(source: IPoint) -> Self {
        Self::new(source.x as _, source.y as _)
    }
}