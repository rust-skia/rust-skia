use super::{DebugAttributes, Inherits, SvgNode};
use crate::{prelude::*, Matrix};
use skia_bindings as sb;

pub type SvgTransformableNode = Inherits<sb::SkSVGTransformableNode, SvgNode>;

impl DebugAttributes for SvgTransformableNode {
    const NAME: &'static str = "TransformableNode";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base
            ._dbg(builder.field("transform", self.get_transform()));
    }
}

impl NativeRefCountedBase for sb::SkSVGTransformableNode {
    type Base = sb::SkRefCntBase;
}

impl SvgTransformableNode {
    pub fn from_ptr(node: *mut sb::SkSVGTransformableNode) -> Option<Self> {
        let base = SvgNode::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGTransformableNode) -> Option<Self> {
        let base = SvgNode::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        unsafe {
            sb::C_SkSVGTransformableNode_setTransform(self.native_mut(), transform.into_native())
        }
    }

    pub fn get_transform(&self) -> &Matrix {
        Matrix::from_native_ref(&self.native().fTransform)
    }
}
