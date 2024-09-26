use crate::{
    prelude::*,
    scalar,
    svg::{DebugAttributes, Inherits, SvgContainer},
};
use skia_bindings as sb;

pub type SvgFeDistantLight = Inherits<sb::SkSVGFeDistantLight, SvgContainer>;

impl DebugAttributes for SvgFeDistantLight {
    const NAME: &'static str = "FeDistantLight";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("azimuth", &self.get_azimuth())
                .field("elevation", &self.get_elevation()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeDistantLight {
    type Base = sb::SkRefCntBase;
}

impl SvgFeDistantLight {
    pub fn from_ptr(node: *mut sb::SkSVGFeDistantLight) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeDistantLight) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeDistantLight[native, native_mut] => {
            *azimuth: scalar [get(value) => value, set(value) => value],
            *elevation: scalar [get(value) => value, set(value) => value]
        }
    }
}

pub type SvgFePointLight = Inherits<sb::SkSVGFePointLight, SvgContainer>;

impl DebugAttributes for SvgFePointLight {
    const NAME: &'static str = "FePointLight";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("z", &self.get_z()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFePointLight {
    type Base = sb::SkRefCntBase;
}

impl SvgFePointLight {
    pub fn from_ptr(node: *mut sb::SkSVGFePointLight) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFePointLight) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFePointLight[native, native_mut] => {
            *x: scalar [get(value) => value, set(value) => value],
            *y: scalar [get(value) => value, set(value) => value],
            *z: scalar [get(value) => value, set(value) => value]
        }
    }
}

pub type SvgFeSpotLight = Inherits<sb::SkSVGFeSpotLight, SvgContainer>;

impl DebugAttributes for SvgFeSpotLight {
    const NAME: &'static str = "FeSpotLight";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.base._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("z", &self.get_z())
                .field("points_at_x", &self.get_points_at_x())
                .field("points_at_y", &self.get_points_at_y())
                .field("points_at_z", &self.get_points_at_z())
                .field("specular_exponent", &self.get_specular_exponent())
                .field("limiting_cone_angle", &self.get_limiting_cone_angle()),
        );
    }
}

impl NativeRefCountedBase for sb::SkSVGFeSpotLight {
    type Base = sb::SkRefCntBase;
}

impl SvgFeSpotLight {
    pub fn from_ptr(node: *mut sb::SkSVGFeSpotLight) -> Option<Self> {
        let base = SvgContainer::from_ptr(node as *mut _)?;
        let data = RCHandle::from_ptr(node)?;

        Some(Self { base, data })
    }

    pub fn from_unshared_ptr(node: *mut sb::SkSVGFeSpotLight) -> Option<Self> {
        let base = SvgContainer::from_unshared_ptr(node as *mut _)?;
        let data = RCHandle::from_unshared_ptr(node)?;

        Some(Self { base, data })
    }

    skia_macros::attrs! {
        SkSVGFeSpotLight[native, native_mut] => {
            *x: scalar [get(value) => value, set(value) => value],
            *y: scalar [get(value) => value, set(value) => value],
            *z: scalar [get(value) => value, set(value) => value],
            *points_at_x: scalar [get(value) => value, set(value) => value],
            *points_at_y: scalar [get(value) => value, set(value) => value],
            *points_at_z: scalar [get(value) => value, set(value) => value],
            *specular_exponent: scalar [get(value) => value, set(value) => value],
            *limiting_cone_angle?: scalar [get(value) => value, set(value) => value]
        }
    }
}
