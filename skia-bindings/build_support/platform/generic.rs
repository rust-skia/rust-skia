use super::prelude::*;

pub fn args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    builder.skia_target_os_and_default_cpu(&config.target.system);
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    if features.gl {
        builder.link_library("GL");
    }
}
