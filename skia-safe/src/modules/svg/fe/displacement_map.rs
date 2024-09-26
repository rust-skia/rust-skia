use super::{DebugAttributes, Inherits, SvgFe, SvgFeInput};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

pub type SvgChannelSelector = sb::SkSVGFeDisplacementMap_ChannelSelector;
pub type SvgFeDisplacementMap = Inherits<sb::SkSVGFeDisplacementMap, SvgFe>;

impl DebugAttributes for SvgFeDisplacementMap {
    const NAME: &'static str = "FeDisplacementMap";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("input2", self.get_input2())
                .field("x_channel_selector", self.get_x_channel_selector())
                .field("y_channel_selector", self.get_y_channel_selector())
                .field("scale", self.get_scale()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeDisplacementMap {
    type Base = sb::SkRefCntBase;
}

impl SvgFeDisplacementMap {
    pub fn from_ptr(node: *mut sb::SkSVGFeDisplacementMap) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeDisplacementMap) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeDisplacementMap[native, native_mut] => {
            "in2" as input2: SvgFeInput [get(value) => SvgFeInput::from_native_ref(value), set(value) => value.into_native()],
            x_channel_selector: SvgChannelSelector [get(value) => value, set(value) => value],
            y_channel_selector: SvgChannelSelector [get(value) => value, set(value) => value],
            scale: scalar [get(value) => value, set(value) => value]
        }
    }
}
