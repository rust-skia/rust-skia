use super::{linux, prelude::*};

pub struct Musl;

impl PlatformDetails for Musl {
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        linux::gn_args(config, builder);
        let target = &config.target;

        builder.cflags(flags(target));
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(flags(target))
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        linux::link_libraries(features)
    }
}

fn flags(target: &Target) -> Vec<String> {
    let arch = &target.architecture;
    let cpp = "10.3.1";

    vec![
        format!("-I/usr/include/c++/{cpp}"),
        format!("-I/usr/include/c++/{cpp}/{arch}-alpine-linux-musl"),
    ]
}
