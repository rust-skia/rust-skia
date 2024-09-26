use super::{DebugAttributes, Inherits, SvgContainer, SvgLength, SvgPreserveAspectRatio};
use crate::{prelude::*, Rect, Size};
use skia_bindings as sb;

pub type Svg = Inherits<sb::SkSVGSVG, SvgContainer>;

impl DebugAttributes for Svg {
    const NAME: &'static str = "Svg";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("width", &self.get_width())
                .field("height", &self.get_height())
                .field("preserve_aspect_ratio", self.get_preserve_aspect_ratio())
                .field("view_box", &self.get_view_box()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGSVG {
    type Base = sb::SkRefCntBase;
}

impl Svg {
    pub fn from_ptr(node: *mut sb::SkSVGSVG) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGSVG) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn intrinsic_size(&self) -> Size {
        unsafe { Size::from_native_c(sb::C_SkSVGSVG_intrinsicSize(self.native())) }
    }

    skia_macros::attrs! {
        SkSVGSVG[native, native_mut] => {
            x: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            y: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            width: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            height: SvgLength [get(value) => SvgLength::from_native_ref(value), set(value) => value.into_native()],
            preserve_aspect_ratio: SvgPreserveAspectRatio [get(value) => SvgPreserveAspectRatio::from_native_ref(value), set(value) => value.into_native()],
            view_box?: Rect [get(value) => value.map(Rect::from_native_ref), set(value) => value.into_native()]
        }
    }
}
