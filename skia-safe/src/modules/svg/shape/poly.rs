use super::SvgShape;
use crate::{
    prelude::*,
    svg::{DebugAttributes, Inherits},
    Point,
};
use skia_bindings as sb;

pub type SvgPoly = Inherits<sb::SkSVGPoly, SvgShape>;

impl DebugAttributes for SvgPoly {
    const NAME: &'static str = "Poly";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder.field("points", &self.get_points()));
    }
}

impl NativeRefCountedBase for sb::SkSVGPoly {
    type Base = sb::SkRefCntBase;
}

impl SvgPoly {
    pub fn from_ptr(node: *mut sb::SkSVGPoly) -> Option<Self> {
        let base = SvgShape::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGPoly) -> Option<Self> {
        let base = SvgShape::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

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
