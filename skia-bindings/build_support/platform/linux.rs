use super::{generic, prelude::*};

pub struct Linux;

impl PlatformDetails for Linux {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        args(config, builder)
    }

    fn link_libraries(&self, features: &Features, builder: &mut LinkLibrariesBuilder) {
        link_libraries(features, builder)
    }
}

pub fn args(config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
    generic::args(config, builder);
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    builder.link_libraries(["stdc++", "fontconfig", "freetype"]);

    if features.gl {
        if features.egl {
            builder.link_library("EGL");
        }

        if features.x11 {
            builder.link_library("GL");
        }

        if features.wayland {
            builder.link_library("wayland-egl").link_library("GLESv2");
        }
    }
}
