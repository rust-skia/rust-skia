use super::prelude::*;

pub struct Generic;

impl PlatformDetails for Generic {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        args(config, builder)
    }

    fn link_libraries(&self, features: &Features, builder: &mut LinkLibrariesBuilder) {
        link_libraries(features, builder)
    }
}

pub fn args(config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
    builder.skia_target_os_and_default_cpu(&config.target.system);
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    if features.gl {
        builder.link_library("GL");
    }
}
