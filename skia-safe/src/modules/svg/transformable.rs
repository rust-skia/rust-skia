use super::{DebugAttributes, HasBase};
use crate::{prelude::*, Matrix};
use skia_bindings as sb;

pub type SvgTransformableNode = RCHandle<sb::SkSVGTransformableNode>;

impl NativeRefCountedBase for sb::SkSVGTransformableNode {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTransformableNode {
    type Base = sb::SkSVGNode;
}

impl DebugAttributes for SvgTransformableNode {
    const NAME: &'static str = "TransformableNode";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("transform", self.get_transform()));
    }
}

impl SvgTransformableNode {
    pub fn set_transform(&mut self, transform: &Matrix) {
        unsafe { sb::C_SkSVGTransformableNode_setTransform(self.native_mut(), transform.native()) }
    }

    pub fn get_transform(&self) -> &Matrix {
        Matrix::from_native_ref(&self.native().fTransform)
    }
}
