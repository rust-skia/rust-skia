use crate::{
    prelude::*,
    svg::{DebugAttributes, HasBase},
    Point,
};
use skia_bindings as sb;

pub type Poly = RCHandle<sb::SkSVGPoly>;

impl NativeRefCountedBase for sb::SkSVGPoly {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGPoly {
    type Base = sb::SkSVGShape;
}

impl DebugAttributes for Poly {
    const NAME: &'static str = "Poly";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("points", &self.get_points()));
    }
}

impl Poly {
    pub fn get_points(&self) -> &[Point] {
        unsafe {
            safer::from_raw_parts(
                Point::from_native_ptr(sb::C_SkSVGPoly_getPoints(self.native())),
                self.get_points_count(),
            )
        }
    }

    pub fn get_points_count(&self) -> usize {
        unsafe { sb::C_SkSVGPoly_getPointsCount(self.native()) }
    }
}
