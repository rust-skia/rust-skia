use super::{DebugAttributes, HasBase, Iri, Length, XmlSpace};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

type SvgTextContainer = RCHandle<sb::SkSVGTextContainer>;

impl NativeRefCountedBase for sb::SkSVGTextContainer {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTextContainer {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for SvgTextContainer {
    const NAME: &'static str = "TextContainer";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
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

impl SvgTextContainer {
    pub fn get_x(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getX(self.native())),
                self.get_x_count(),
            )
        }
    }

    pub fn get_x_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getXCount(self.native()) }
    }

    pub fn get_y(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getY(self.native())),
                self.get_y_count(),
            )
        }
    }

    pub fn get_y_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getYCount(self.native()) }
    }

    pub fn get_dx(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getDx(self.native())),
                self.get_dx_count(),
            )
        }
    }

    pub fn get_dx_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getDxCount(self.native()) }
    }

    pub fn get_dy(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getDy(self.native())),
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
        SkSVGTextContainer => {
            xml_space: XmlSpace [get(value) => value, set(value) => value]
        }
    }
}

pub type Text = RCHandle<sb::SkSVGText>;

impl NativeRefCountedBase for sb::SkSVGText {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGText {
    type Base = sb::SkSVGTextContainer;
}

impl DebugAttributes for Text {
    const NAME: &'static str = "Text";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}

pub type TSpan = RCHandle<sb::SkSVGTSpan>;

impl NativeRefCountedBase for sb::SkSVGTSpan {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTSpan {
    type Base = sb::SkSVGTextContainer;
}

impl DebugAttributes for TSpan {
    const NAME: &'static str = "TSpan";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}

pub type TextPath = RCHandle<sb::SkSVGTextPath>;

impl NativeRefCountedBase for sb::SkSVGTextPath {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTextPath {
    type Base = sb::SkSVGTextContainer;
}

impl DebugAttributes for TextPath {
    const NAME: &'static str = "TextPath";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("href", &self.get_href())
                .field("start_offset", &self.get_start_offset()),
        );
    }
}

impl TextPath {
    skia_macros::attrs! {
        SkSVGTextPath => {
            href: Iri [get(value) => Iri::from_native_ref(value), set(value) => value.into_native()],
            start_offset: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}

pub type TextLiteral = RCHandle<sb::SkSVGTextLiteral>;

impl NativeRefCountedBase for sb::SkSVGTextLiteral {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTextLiteral {
    type Base = sb::SkSVGTransformableNode;
}

impl DebugAttributes for TextLiteral {
    const NAME: &'static str = "TextLiteral";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder.field("text", &self.get_text()));
    }
}

impl TextLiteral {
    skia_macros::attrs! {
        SkSVGTextLiteral => {
            text: crate::interop::String [get(value) => crate::interop::String::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
