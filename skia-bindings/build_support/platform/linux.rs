use super::{generic, prelude::*};
use crate::build_support::features::Features;

pub fn args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
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
