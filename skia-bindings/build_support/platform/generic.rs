use super::prelude::*;

pub fn args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    builder.skia_target_os_and_default_cpu(&config.target.system);
}
