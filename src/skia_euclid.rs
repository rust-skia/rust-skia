use crate::prelude::*;
use rust_skia::SkPoint3;

pub struct Skia;

pub type Point3 = euclid::Point3D<f32>;
pub type Vector3 = Point3;
pub type Color3f = Point3;

pub trait SkiaPoint<S> : Sized {
    fn is_zero(&self) -> bool;
}

pub trait SkiaPointFloat<S> {
    fn length(&self) -> S;

    #[must_use]
    fn normalize(&self) -> Self;
    #[must_use]
    fn scale(&self, scale: S) -> Self;

    fn is_finite(&self) -> bool;
    fn distance(a: &Self, b: &Self) -> S;
    fn dot_product(a: &Self, b: &Self) -> S;
    fn cross_product(a: &Self, b: &Self) -> S;
}

pub trait SkiaSize<S> {
    fn new_empty() -> Self;
    fn is_zero(&self) -> bool;
    fn is_empty(&self) -> bool;
}

pub trait SkiaRect<S> : Sized {
    fn new_empty() -> Self;
    fn from_wh(w: S, h: S) -> Self;
    // exists:
    // fn from_size(size: euclid::Size2D<S>) -> Self;
    fn from_ltrb(l: S, t: S, r: S, b: S) -> Self;
    fn from_xywh(x: S, y: S, w: S, h: S) -> Self;

    fn left(&self) -> S;
    fn right(&self) -> S;
    fn top(&self) -> S;
    fn bottom(&self) -> S;

    fn x(&self) -> S;
    fn y(&self) -> S;
    fn top_left(&self) -> euclid::Point2D<S>;
    fn width(&self) -> S;
    fn height(&self) -> S;
    fn size(&self) -> euclid::Size2D<S>;

    fn is_empty(&self) -> bool;
    fn is_sorted(&self) -> bool;

    #[must_use]
    fn offset(&self, dx: S, dy: S) -> Self;
    #[must_use]
    fn offset_to(&self, new_x: S, new_y: S) -> Self;
    #[must_use]
    fn inset(&self, dx: S, dy: S) -> Self;
    #[must_use]
    fn outset(&self, dx: S, dy: S) -> Self;

    #[must_use]
    fn intersect(&self, r: &Self) -> Option<Self>;
    #[must_use]
    fn insersect_no_empty_check(&self, r: &Self) -> Option<Self>;

    fn intersects(a: &Self, b: &Self) -> bool;
    fn intersects_no_empty_check(a: &Self, b: &Self) -> bool;

    #[must_use]
    fn join(&self, r: &euclid::Rect<S>) -> Self;
    #[must_use]
    fn sort(&self) -> Self;

    const EMPTY: Self;
}

pub trait SkiaRect64 {
    fn width_64(&self) -> i64;
    fn height_64(&self) -> i64;
    fn is_empty_64(&self) -> bool;
}

pub trait SkiaRectContains<T, A> {
    fn contains(&self, arg: A) -> bool;
    fn contains_no_empty_check(&self, arg: A) -> bool;
}

impl NativeRepresentation<SkPoint3> for Point3 {
    fn into_native(self) -> SkPoint3 {
        SkPoint3 {
            fX: self.x,
            fY: self.y,
            fZ: self.z
        }
    }

    fn from_native(native: SkPoint3) -> Self {
        Point3::new(native.fX, native.fY, native.fZ)
    }
}

//
// Liftable
//

impl Liftable<(f32, f32, f32)> for Point3 {
    fn lift_from(source: (f32, f32, f32)) -> Self {
        Point3::new(source.0, source.1, source.2)
    }
}
