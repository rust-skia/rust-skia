mod circle;
mod ellipse;
mod line;
mod path;
mod poly;
mod rect;

pub use self::{
    circle::SvgCircle, ellipse::SvgEllipse, line::SvgLine, path::SvgPath, poly::SvgPoly,
    rect::SvgRect,
};

use super::{DebugAttributes, Inherits, SvgTransformableNode};
use crate::prelude::*;
use skia_bindings as sb;

pub type SvgShape = Inherits<sb::SkSVGShape, SvgTransformableNode>;

impl DebugAttributes for SvgShape {
    const NAME: &'static str = "Shape";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGShape {
    type Base = sb::SkRefCntBase;
}

impl SvgShape {
    pub fn from_ptr(node: *mut sb::SkSVGShape) -> Option<Self> {
        let base = SvgTransformableNode::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGShape) -> Option<Self> {
        let base = SvgTransformableNode::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }
}
