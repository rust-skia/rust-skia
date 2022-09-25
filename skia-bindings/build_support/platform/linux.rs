use super::{generic, prelude::*};

pub struct Linux;

impl PlatformDetails for Linux {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        gn_args(config, builder)
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        link_libraries(features)
    }
}

pub fn gn_args(config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
    generic::gn_args(config, builder);
}

pub fn link_libraries(features: &Features) -> Vec<String> {
    let mut libs = vec!["stdc++", "fontconfig", "freetype"];

    if features.gl {
        if features.egl {
            libs.push("EGL");
        }

        if features.x11 {
            libs.push("GL");
        }

        if features.wayland {
            libs.push("wayland-egl");
            libs.push("GLESv2");
        }
    }

    libs.iter().map(|l| l.to_string()).collect()
}
