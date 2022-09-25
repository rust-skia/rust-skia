use super::prelude::*;

pub struct Alpine;

impl TargetDetails for Alpine {
    fn args(&self, config: &BuildConfiguration, builder: &mut ArgBuilder) {
        let arch = &config.target.architecture;
        let cpp = "10.3.1";
        builder
            .cflag(format!("-I/usr/include/c++/{cpp}"))
            .cflag(format!("-I/usr/include/c++/{cpp}/{arch}-alpine-linux-musl",));

        builder.skia_target_os_and_default_cpu(&config.target.system);
    }
}
