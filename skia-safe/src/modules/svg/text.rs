use super::{DebugAttributes, HasBase, Iri, Length, XmlSpace};
use crate::{impl_default_make, interop, prelude::*, scalar};
use skia_bindings as sb;

type TextContainer = RCHandle<sb::SkSVGTextContainer>;

impl NativeRefCountedBase for sb::SkSVGTextContainer {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTextContainer {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for TextContainer {
    const NAME: &'static str = "TextContainer";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x", &self.x())
                .field("y", &self.y())
                .field("dx", &self.dx())
                .field("dy", &self.dy())
                .field("rotate", &self.rotate())
                .field("xml_space", &self.xml_space()),
        );
    }
}

impl TextContainer {
    pub fn x(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getX(self.native())),
                self.x_count(),
            )
        }
    }

    pub(crate) fn x_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getXCount(self.native()) }
    }

    pub fn y(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getY(self.native())),
                self.y_count(),
            )
        }
    }

    pub(crate) fn y_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getYCount(self.native()) }
    }

    pub fn dx(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getDx(self.native())),
                self.dx_count(),
            )
        }
    }

    pub(crate) fn dx_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getDxCount(self.native()) }
    }

    pub fn dy(&self) -> &[Length] {
        unsafe {
            safer::from_raw_parts(
                Length::from_native_ptr(sb::C_SkSVGTextContainer_getDy(self.native())),
                self.dy_count(),
            )
        }
    }

    pub(crate) fn dy_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getDyCount(self.native()) }
    }

    pub fn rotate(&self) -> &[scalar] {
        unsafe {
            safer::from_raw_parts(
                sb::C_SkSVGTextContainer_getRotate(self.native()),
                self.rotate_count(),
            )
        }
    }

    pub(crate) fn rotate_count(&self) -> usize {
        unsafe { sb::C_SkSVGTextContainer_getRotateCount(self.native()) }
    }

    skia_svg_macros::attrs! {
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

impl_default_make!(Text, sb::C_SkSVGText_Make);

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

impl_default_make!(TSpan, sb::C_SkSVGTSpan_Make);

impl DebugAttributes for TSpan {
    const NAME: &'static str = "TSpan";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder);
    }
}

pub type TextLiteral = RCHandle<sb::SkSVGTextLiteral>;

impl NativeRefCountedBase for sb::SkSVGTextLiteral {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTextLiteral {
    type Base = sb::SkSVGTransformableNode;
}

impl_default_make!(TextLiteral, sb::C_SkSVGTextLiteral_Make);

impl DebugAttributes for TextLiteral {
    const NAME: &'static str = "TextLiteral";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(builder.field("text", &self.text()));
    }
}

impl TextLiteral {
    skia_svg_macros::attrs! {
        SkSVGTextLiteral => {
            text: str [
                get(value) => interop::String::from_native_ref(value).as_str(),
                set(&value) => interop::String::from_str(value).into_native()
            ]
        }
    }
}

pub type TextPath = RCHandle<sb::SkSVGTextPath>;

impl NativeRefCountedBase for sb::SkSVGTextPath {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGTextPath {
    type Base = sb::SkSVGTextContainer;
}

impl_default_make!(TextPath, sb::C_SkSVGTextPath_Make);

impl DebugAttributes for TextPath {
    const NAME: &'static str = "TextPath";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("href", &self.href())
                .field("start_offset", &self.start_offset()),
        );
    }
}

impl TextPath {
    skia_svg_macros::attrs! {
        SkSVGTextPath => {
            href: Iri [get(value) => Iri::from_native_ref(value), set(value) => value.into_native()],
            start_offset: Length [get(value) => Length::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}
