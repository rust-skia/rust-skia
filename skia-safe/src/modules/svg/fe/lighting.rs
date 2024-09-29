use super::{DebugAttributes, Inherits, SvgFe};
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

pub type SvgFeLighting = Inherits<sb::SkSVGFeLighting, SvgFe>;

impl DebugAttributes for SvgFeLighting {
    const NAME: &'static str = "FeLighting";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("surface_scale", &self.get_surface_scale())
                .field("kernel_unit_length", &self.get_kernel_unit_length()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeLighting {
    type Base = sb::SkRefCntBase;
}

impl SvgFeLighting {
    pub fn from_ptr(node: *mut sb::SkSVGFeLighting) -> Option<Self> {
        let base = SvgFe::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeLighting) -> Option<Self> {
        let base = SvgFe::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeLighting[native, native_mut] => {
            *surface_scale: scalar [get(value) => value, set(value) => value],
            *kernel_unit_length?: KernelUnitLength [get(value) => value.map(KernelUnitLength::from_native_c), set(value) => value.into_native()]
        }
    }
}

pub type SvgFeSpecularLighting = Inherits<sb::SkSVGFeSpecularLighting, SvgFeLighting>;

impl DebugAttributes for SvgFeSpecularLighting {
    const NAME: &'static str = "FeSpecularLighting";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("specular_constant", &self.get_specular_constant())
                .field("specular_exponent", &self.get_specular_exponent()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeSpecularLighting {
    type Base = sb::SkRefCntBase;
}

impl SvgFeSpecularLighting {
    pub fn from_ptr(node: *mut sb::SkSVGFeSpecularLighting) -> Option<Self> {
        let base = SvgFeLighting::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeSpecularLighting) -> Option<Self> {
        let base = SvgFeLighting::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeSpecularLighting[native, native_mut] => {
            *specular_constant: scalar [get(value) => value, set(value) => value],
            *specular_exponent: scalar [get(value) => value, set(value) => value]
        }
    }
}

pub type SvgFeDiffuseLighting = Inherits<sb::SkSVGFeDiffuseLighting, SvgFeLighting>;

impl DebugAttributes for SvgFeDiffuseLighting {
    const NAME: &'static str = "FeDiffuseLighting";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base
            ._dbg(builder.field("diffuse_constant", &self.get_diffuse_constant()));
    }
}

impl NativeRefCountedBase for sb::SkSVGFeDiffuseLighting {
    type Base = sb::SkRefCntBase;
}

impl SvgFeDiffuseLighting {
    pub fn from_ptr(node: *mut sb::SkSVGFeDiffuseLighting) -> Option<Self> {
        let base = SvgFeLighting::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeDiffuseLighting) -> Option<Self> {
        let base = SvgFeLighting::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeDiffuseLighting[native, native_mut] => {
            *diffuse_constant: scalar [get(value) => value, set(value) => value]
        }
    }
}
