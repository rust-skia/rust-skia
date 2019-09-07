use crate::prelude::*;
use crate::{scalar, Point, Scalar};
use skia_bindings::SkCubicMap;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct CubicMap(SkCubicMap);

impl NativeTransmutable<SkCubicMap> for CubicMap {}

#[test]
fn test_cubic_map_layout() {
    CubicMap::test_layout()
}

impl CubicMap {
    pub fn new(p1: impl Into<Point>, p2: impl Into<Point>) -> Self {
        Self::from_native(unsafe {
            SkCubicMap::new(p1.into().into_native(), p2.into().into_native())
        })
    }

    pub fn is_linear(p1: impl Into<Point>, p2: impl Into<Point>) -> bool {
        let p1 = p1.into();
        let p2 = p2.into();
        scalar::nearly_equal(p1.x, p1.y, None) && scalar::nearly_equal(p2.x, p2.y, None)
    }

    pub fn compute_y_from_x(&self, x: f32) -> f32 {
        unsafe { self.native().computeYFromX(x) }
    }

    pub fn compute_from_t(&self, t: f32) -> Point {
        Point::from_native(unsafe { self.native().computeFromT(t) })
    }
}
