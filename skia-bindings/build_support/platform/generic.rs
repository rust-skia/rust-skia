use super::prelude::*;

pub struct Generic;

impl TargetDetails for Generic {
    fn args(&self, config: &BuildConfiguration, builder: &mut ArgBuilder) {
        builder.skia_target_os_and_default_cpu(&config.target.system);
    }
}
