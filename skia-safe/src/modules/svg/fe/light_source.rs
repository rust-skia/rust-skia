use crate::{
    prelude::*,
    scalar,
    svg::{DebugAttributes, HasBase},
};
use skia_bindings as sb;

pub type FeDistantLight = RCHandle<sb::SkSVGFeDistantLight>;

impl NativeRefCountedBase for sb::SkSVGFeDistantLight {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeDistantLight {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for FeDistantLight {
    const NAME: &'static str = "FeDistantLight";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("azimuth", &self.get_azimuth())
                .field("elevation", &self.get_elevation()),
        );
    }
}

impl FeDistantLight {
    skia_macros::attrs! {
        SkSVGFeDistantLight => {
            *azimuth: scalar [get(value) => value, set(value) => value],
            *elevation: scalar [get(value) => value, set(value) => value]
        }
    }
}

pub type FePointLight = RCHandle<sb::SkSVGFePointLight>;

impl NativeRefCountedBase for sb::SkSVGFePointLight {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFePointLight {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for FePointLight {
    const NAME: &'static str = "FePointLight";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
            builder
                .field("x", &self.get_x())
                .field("y", &self.get_y())
                .field("z", &self.get_z()),
        );
    }
}

impl FePointLight {
    skia_macros::attrs! {
        SkSVGFePointLight => {
            *x: scalar [get(value) => value, set(value) => value],
            *y: scalar [get(value) => value, set(value) => value],
            *z: scalar [get(value) => value, set(value) => value]
        }
    }
}

pub type FeSpotLight = RCHandle<sb::SkSVGFeSpotLight>;

impl NativeRefCountedBase for sb::SkSVGFeSpotLight {
    type Base = sb::SkRefCntBase;
}

impl HasBase for sb::SkSVGFeSpotLight {
    type Base = sb::SkSVGContainer;
}

impl DebugAttributes for FeSpotLight {
    const NAME: &'static str = "FeSpotLight";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        self.as_base()._dbg(
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

impl FeSpotLight {
    skia_macros::attrs! {
        SkSVGFeSpotLight => {
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
