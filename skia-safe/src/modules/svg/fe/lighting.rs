use super::{DebugAttributes, HasBase};
use crate::{prelude::*, scalar};
use skia_bindings as sb;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernelUnitLength {
    dx: scalar,
    dy: scalar,
}

impl KernelUnitLength {
    pub fn new(dx: scalar, dy: scalar) -> Self {
        Self { dx, dy }
    }

    pub fn new_all(value: scalar) -> Self {
        Self {
            dx: value,
            dy: value,
        }
    }
}

native_transmutable!(
    sb::SkSVGFeLighting_KernelUnitLength,
    KernelUnitLength,
    svg_kernel_unit_length_layout
);

pub type SvgFeLighting = RCHandle<sb::SkSVGFeLighting>;

impl NativeRefCountedBase for sb::SkSVGFeLighting {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeLighting {
    type Base = sb::SkSVGFe;
}

impl DebugAttributes for SvgFeLighting {
    const NAME: &'static str = "FeLighting";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("surface_scale", &self.get_surface_scale())
                .field("kernel_unit_length", &self.get_kernel_unit_length()),
        );
    }
}

impl SvgFeLighting {
    skia_macros::attrs! {
        SkSVGFeLighting => {
            *surface_scale: scalar [get(value) => value, set(value) => value],
            *kernel_unit_length?: KernelUnitLength [get(value) => value.map(KernelUnitLength::from_native_c), set(value) => value.into_native()]
        }
    }
}

pub type FeSpecularLighting = RCHandle<sb::SkSVGFeSpecularLighting>;

impl NativeRefCountedBase for sb::SkSVGFeSpecularLighting {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeSpecularLighting {
    type Base = sb::SkSVGFeLighting;
}

impl DebugAttributes for FeSpecularLighting {
    const NAME: &'static str = "FeSpecularLighting";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("specular_constant", &self.get_specular_constant())
                .field("specular_exponent", &self.get_specular_exponent()),
        );
    }
}

impl FeSpecularLighting {
    skia_macros::attrs! {
        SkSVGFeSpecularLighting => {
            *specular_constant: scalar [get(value) => value, set(value) => value],
            *specular_exponent: scalar [get(value) => value, set(value) => value]
        }
    }
}

pub type FeDiffuseLighting = RCHandle<sb::SkSVGFeDiffuseLighting>;

impl NativeRefCountedBase for sb::SkSVGFeDiffuseLighting {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeDiffuseLighting {
    type Base = sb::SkSVGFeLighting;
}

impl DebugAttributes for FeDiffuseLighting {
    const NAME: &'static str = "FeDiffuseLighting";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()
            ._dbg(builder.field("diffuse_constant", &self.get_diffuse_constant()));
    }
}

impl FeDiffuseLighting {
    skia_macros::attrs! {
        SkSVGFeDiffuseLighting => {
            *diffuse_constant: scalar [get(value) => value, set(value) => value]
        }
    }
}
