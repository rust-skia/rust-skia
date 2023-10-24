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

    if skia::env::use_system_libraries() {
        libs.push("png16");
        libs.push("z");
        libs.push("icudata");
        libs.push("icui18n");
        libs.push("icuio");
        libs.push("icutest");
        libs.push("icutu");
        libs.push("icuuc");
        libs.push("harfbuzz");
        libs.push("expat");

        if features.webp_encode || features.webp_decode {
            libs.push("webp");
        }
    }

    if skia::env::use_system_libraries() || cfg!(feature = "use-system-jpeg-turbo") {
        libs.push("jpeg");
    }

    libs.iter().map(|l| l.to_string()).collect()
}
