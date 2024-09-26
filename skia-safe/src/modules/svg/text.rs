use super::{DebugAttributes, Inherits, SvgContainer, SvgIri, SvgLength, SvgTransformableNode};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgXmlSpace = sb::SkSVGXmlSpace;

type SvgTextContainer = Inherits<sb::SkSVGTextContainer, SvgContainer>;

impl DebugAttributes for SvgTextContainer {
    const NAME: &'static str = "TextContainer";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("dx", &self.get_dx())
                .field("dy", &self.get_dy())
                .field("rotate", &self.get_rotate())
                .field("xml_space", &self.get_xml_space()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGTextContainer {
    type Base = sb::SkRefCntBase;
}

impl SvgTextContainer {
    pub fn from_ptr(node: *mut sb::SkSVGTextContainer) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGTextContainer) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn get_x(&self) -> &[SvgLength] {
        unsafe {
            safer::from_raw_parts(
                SvgLength::from_native_ptr(sb::C_SkSVGTextContainer_getX(self.native())),
                self.get_x_count(),
            )
        }
    }

    pub fn get_x_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getXCount(self.native()) }
    }

    pub fn get_y(&self) -> &[SvgLength] {
        unsafe {
            safer::from_raw_parts(
                SvgLength::from_native_ptr(sb::C_SkSVGTextContainer_getY(self.native())),
                self.get_y_count(),
            )
        }
    }

    pub fn get_y_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getYCount(self.native()) }
    }

    pub fn get_dx(&self) -> &[SvgLength] {
        unsafe {
            safer::from_raw_parts(
                SvgLength::from_native_ptr(sb::C_SkSVGTextContainer_getDx(self.native())),
                self.get_dx_count(),
            )
        }
    }

    pub fn get_dx_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getDxCount(self.native()) }
    }

    pub fn get_dy(&self) -> &[SvgLength] {
        unsafe {
            safer::from_raw_parts(
                SvgLength::from_native_ptr(sb::C_SkSVGTextContainer_getDy(self.native())),
                self.get_dy_count(),
            )
        }
    }

    pub fn get_dy_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getDyCount(self.native()) }
    }

    pub fn get_rotate(&self) -> &[scalar] {
        unsafe {
            safer::from_raw_parts(
                sb::C_SkSVGTextContainer_getRotate(self.native()),
                self.get_rotate_count(),
            )
        }
    }

    pub fn get_rotate_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getRotateCount(self.native()) }
    }

    skia_macros::attrs! {
        SkSVGTextContainer[native, native_mut] => {
            xml_space: SvgXmlSpace [get(value) => value, set(value) => value]
        }
    }
}

pub type SvgText = Inherits<sb::SkSVGText, SvgTextContainer>;

impl DebugAttributes for SvgText {
    const NAME: &'static str = "Text";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGText {
    type Base = sb::SkRefCntBase;
}

impl SvgText {
    pub fn from_ptr(node: *mut sb::SkSVGText) -> Option<Self> {
        let base = SvgTextContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGText) -> Option<Self> {
        let base = SvgTextContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }
}

pub type SvgTSpan = Inherits<sb::SkSVGTSpan, SvgTextContainer>;

impl DebugAttributes for SvgTSpan {
    const NAME: &'static str = "TSpan";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder);
    }
}

impl NativeRefCountedBase for sb::SkSVGTSpan {
    type Base = sb::SkRefCntBase;
}

impl SvgTSpan {
    pub fn from_ptr(node: *mut sb::SkSVGTSpan) -> Option<Self> {
        let base = SvgTextContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGTSpan) -> Option<Self> {
        let base = SvgTextContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }
}

pub type SvgTextPath = Inherits<sb::SkSVGTextPath, SvgTextContainer>;

impl DebugAttributes for SvgTextPath {
    const NAME: &'static str = "TextPath";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("href", &self.get_href())
                .field("start_offset", &self.get_start_offset()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGTextPath {
    type Base = sb::SkRefCntBase;
}

impl SvgTextPath {
    pub fn from_ptr(node: *mut sb::SkSVGTextPath) -> Option<Self> {
        let base = SvgTextContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGTextPath) -> Option<Self> {
        let base = SvgTextContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGTextPath[native, native_mut] => {
            href: SvgIri [get(value) => SvgIri::from_native_ref(value), set(value) => value.into_native()],
            start_offset: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}

pub type SvgTextLiteral = Inherits<sb::SkSVGTextLiteral, SvgTransformableNode>;

impl DebugAttributes for SvgTextLiteral {
    const NAME: &'static str = "TextLiteral";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(builder.field("text", &self.get_text()));
    }
}

impl NativeRefCountedBase for sb::SkSVGTextLiteral {
    type Base = sb::SkRefCntBase;
}

impl SvgTextLiteral {
    pub fn from_ptr(node: *mut sb::SkSVGTextLiteral) -> Option<Self> {
        let base = SvgTransformableNode::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGTextLiteral) -> Option<Self> {
        let base = SvgTransformableNode::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGTextLiteral[native, native_mut] => {
            text: crate::interop::String [get(value) => crate::interop::String::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
