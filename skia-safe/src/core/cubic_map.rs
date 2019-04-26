use crate::prelude::*;
use crate::Point;
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
    pub fn new<P1: Into<Point>, P2: Into<Point>>(p1: P1, p2: P2) -> Self {
        Self::from_native(unsafe {
            SkCubicMap::new(p1.into().into_native(), p2.into().into_native())
        })
    }

    pub fn compute_y_from_x(&self, x: f32) -> f32 {
        unsafe { self.native().computeYFromX(x) }
    }

    pub fn compute_from_t(&self, t: f32) -> Point {
        Point::from_native(unsafe { self.native().computeFromT(t) })
    }
}
