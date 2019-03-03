use rust_skia::{
    SkIRect,
    SkIPoint,
    SkISize,
    SkPoint,
    SkSize,
    SkRect
};
use crate::prelude::*;
use rust_skia::SkPoint3;

pub struct Skia;

type IPoint = euclid::Point2D<i32>;
type IVector = Point;
pub type ISize = euclid::Size2D<i32>;
pub type IRect = euclid::Rect<i32>;

type Point = euclid::Point2D<f32>;
type Vector = Point;
pub type Size = euclid::Size2D<f32>;
pub type Rect = euclid::Rect<f32>;

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

pub trait SkiaSizeFloat<S> {
    #[must_use]
    fn to_round(&self) -> ISize;
    #[must_use]
    fn to_ceil(&self) -> ISize;
    #[must_use]
    fn to_floor(&self) -> ISize;
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

pub trait SkiaRectFloat<S> {
    fn new_iwh(w: i32, h: i32) -> Self;
    fn new(r: IRect) -> Self;

    fn is_finite(&self) -> bool;
    fn center_x(&self) -> S;
    fn center_y(&self) -> S;
    fn new_bounds(points: &[euclid::Point2D<S>]) -> Self;
    fn new_bounds_check(points: &[euclid::Point2D<S>]) -> Self;
    fn new_bounds_no_check(points: &[euclid::Point2D<S>]) -> Self;

    #[must_use]
    fn round(&self) -> Self;
    #[must_use]
    fn round_out(&self) -> Self;
    #[must_use]
    fn round_in(&self) -> Self;
}

pub trait SkiaRectContains<T, A> {
    fn contains(&self, arg: A) -> bool;
    fn contains_no_empty_check(&self, arg: A) -> bool;
}

impl SkiaPoint<i32> for IPoint {
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl SkiaSize<i32> for ISize {
    fn new_empty() -> Self {
        Self::zero()
    }

    fn is_zero(&self) -> bool {
        *self == Self::new_empty()
    }

    fn is_empty(&self) -> bool {
        self.is_empty_or_negative()
    }
}

/*

impl NativeRepresentation<SkIPoint> for IPoint {
    fn into_native(self) -> SkIPoint {
        SkIPoint { fX: self.x, fY: self.y }
    }

    fn from_native(native: SkIPoint) -> Self {
        IPoint::new(native.fX, native.fY)
    }
}

*/

impl NativeRepresentation<SkISize> for ISize {
    fn into_native(self) -> SkISize {
        SkISize { fWidth: self.width, fHeight: self.height }
    }

    fn from_native(native: SkISize) -> Self {
        ISize::new(native.fWidth, native.fHeight)
    }
}

impl NativeRepresentation<SkIRect> for IRect {
    fn into_native(self) -> SkIRect {
        let br = self.bottom_right();
        SkIRect{
            fLeft: self.origin.x,
            fTop: self.origin.y,
            fRight: br.x,
            fBottom: br.y
        }
    }

    fn from_native(native: SkIRect) -> Self {
        IRect::new(
            IPoint::new(native.fLeft, native.fTop),
            ISize::new(unsafe { native.width() }, unsafe { native.height() }))
    }
}

impl NativeRepresentation<SkPoint> for Point {
    fn into_native(self) -> SkPoint {
        SkPoint { fX: self.x, fY: self.y }
    }

    fn from_native(native: SkPoint) -> Self {
        Point::new(native.fX, native.fY)
    }
}

impl NativeRepresentation<SkSize> for Size {
    fn into_native(self) -> SkSize {
        SkSize { fWidth: self.width, fHeight: self.height }
    }

    fn from_native(native: SkSize) -> Self {
        Size::new(native.fWidth, native.fHeight)
    }
}

impl NativeRepresentation<SkRect> for Rect {
    fn into_native(self) -> SkRect {
        let br = self.bottom_right();
        SkRect {
            fLeft: self.origin.x,
            fTop: self.origin.y,
            fRight: br.x,
            fBottom: br.y
        }
    }

    fn from_native(native: SkRect) -> Self {
        Rect::new(
            Point::new(native.fLeft, native.fTop),
            Size::new(
                unsafe { native.width() },
                unsafe { native.height() }))
    }
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

impl Liftable<(i32, i32)> for IPoint {
    fn lift_from(source: (i32, i32)) -> Self {
        IPoint::new(source.0, source.1)
    }
}

impl Liftable<(i32, i32)> for ISize {
    fn lift_from(source: (i32, i32)) -> Self {
        ISize::new(source.0, source.1)
    }
}

impl Liftable<(IPoint, ISize)> for IRect {
    fn lift_from(source: (IPoint, ISize)) -> Self {
        IRect::new(source.0, source.1)
    }
}

impl Liftable<(i32, i32, i32, i32)> for IRect {
    fn lift_from(source: (i32, i32, i32, i32)) -> Self {
        ((source.0, source.1).lift(), (source.2, source.3).lift()).lift()
    }
}

impl Liftable<(f32, f32)> for Point {
    fn lift_from(source: (f32, f32)) -> Self {
        Point::new(source.0, source.1)
    }
}

impl Liftable<(f32, f32)> for Size {
    fn lift_from(source: (f32, f32)) -> Self {
        Size::new(source.0, source.1)
    }
}

impl Liftable<(Point, Size)> for Rect {
    fn lift_from(source: (Point, Size)) -> Self {
        Rect::new(source.0, source.1)
    }
}

impl Liftable<(f32, f32, f32, f32)> for Rect {
    fn lift_from(source: (f32, f32, f32, f32)) -> Self {
        ((source.0, source.1).lift(), (source.2, source.3).lift()).lift()
    }
}


impl Liftable<(f32, f32, f32)> for Point3 {
    fn lift_from(source: (f32, f32, f32)) -> Self {
        Point3::new(source.0, source.1, source.2)
    }
}
