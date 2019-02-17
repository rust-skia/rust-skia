use rust_skia::{
    SkIRect,
    SkIPoint
};
use crate::prelude::*;
use rust_skia::SkISize;
use rust_skia::SkPoint;
use rust_skia::SkSize;
use rust_skia::SkRect;

pub struct Skia;

pub type IPoint = euclid::Point2D<i32>;
pub type IVector = IPoint;
pub type ISize = euclid::Size2D<i32>;
pub type IRect = euclid::Rect<i32>;

pub type Point = euclid::Point2D<f32>;
pub type Vector = Point;
pub type Size = euclid::Size2D<f32>;
pub type Rect = euclid::Rect<f32>;

pub trait SkiaPoint<S> : Sized {
    fn new(x: S, y: S) -> Self;
    fn x(&self) -> S;
    fn y(&self) -> S;
    fn is_zero(&self) -> bool;
    fn abs(&self) -> Self;
    fn negate(&self) -> Self;
}

pub trait SkiaPointFloat<S> {
    fn length(&self) -> S;
    fn normalize(&self) -> Self;
    fn scale(&self, scale: S) -> Self;
    fn is_finite(&self) -> bool;
    fn distance(a: &Self, b: &Self) -> S;
    fn dot_product(a: &Self, b: &Self) -> S;
    fn cross_product(a: &Self, b: &Self) -> S;
}

pub trait SkiaRect<S> : Sized {
    fn new_empty() -> Self;
    fn new_wh(w: S, h: S) -> Self;
    fn new_size(size: euclid::Size2D<S>) -> Self;
    fn new_ltrb(l: S, t: S, r: S, b: S) -> Self;
    fn new_xywh(x: S, y: S, w: S, h: S) -> Self;

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

    fn offset(&self, dx: S, dy: S) -> Self;
    fn offset_to(&self, new_x: S, new_y: S) -> Self;
    fn inset(&self, dx: S, dy: S) -> Self;
    fn outset(&self, dx: S, dy: S) -> Self;

    fn intersect(&self, r: &Self) -> Option<Self>;
    fn insersect_no_empty_check(&self, r: &Self) -> Option<Self>;
    fn intersects(a: &Self, b: &Self) -> bool;
    fn intersects_no_empty_check(a: &Self, b: &Self) -> bool;
    fn join(&self, r: &euclid::Rect<S>) -> Self;
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
    fn round(&self) -> Self;
    fn round_out(&self) -> Self;
    fn round_in(&self) -> Self;
}

pub trait SkiaRectContains<T, A> {
    fn contains(&self, arg: A) -> bool;
    fn contains_no_empty_check(&self, arg: A) -> bool;
}

impl NativeRepresentation<SkIPoint> for IPoint {
    fn to_native(&self) -> SkIPoint {
        SkIPoint { fX: self.x, fY: self.y }
    }

    fn from_native(native: &SkIPoint) -> Self {
        IPoint::new(native.fX, native.fY)
    }
}

impl NativeRepresentation<SkISize> for ISize {
    fn to_native(&self) -> SkISize {
        SkISize { fWidth: self.width, fHeight: self.height }
    }

    fn from_native(native: &SkISize) -> Self {
        ISize::new(native.fWidth, native.fHeight)
    }
}

impl NativeRepresentation<SkIRect> for IRect {
    fn to_native(&self) -> SkIRect {
        let br = self.bottom_right();
        SkIRect{
            fLeft: self.origin.x,
            fTop: self.origin.y,
            fRight: br.x,
            fBottom: br.y
        }
    }

    fn from_native(native: &SkIRect) -> Self {
        IRect::new(
            IPoint::new(native.fLeft, native.fTop),
            ISize::new(unsafe { native.width() }, unsafe { native.height() }))
    }
}

impl NativeRepresentation<SkPoint> for Point {
    fn to_native(&self) -> SkPoint {
        SkPoint { fX: self.x, fY: self.y }
    }

    fn from_native(native: &SkPoint) -> Self {
        Point::new(native.fX, native.fY)
    }
}

impl NativeRepresentation<SkSize> for Size {
    fn to_native(&self) -> SkSize {
        SkSize { fWidth: self.width, fHeight: self.height }
    }

    fn from_native(native: &SkSize) -> Self {
        Size::new(native.fWidth, native.fHeight)
    }
}

impl NativeRepresentation<SkRect> for Rect {
    fn to_native(&self) -> SkRect {
        let br = self.bottom_right();
        SkRect {
            fLeft: self.origin.x,
            fTop: self.origin.y,
            fRight: br.x,
            fBottom: br.y
        }
    }

    fn from_native(native: &SkRect) -> Self {
        Rect::new(
            Point::new(native.fLeft, native.fTop),
            Size::new(unsafe { native.width() }, unsafe { native.height() }))
    }
}