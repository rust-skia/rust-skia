use super::{linux, prelude::*};

pub struct Musl;

impl PlatformDetails for Musl {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        linux::args(config, builder);
        let target = &config.target;

        builder.skia_cflags(flags(target));
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.clang_args(flags(target))
    }

    fn link_libraries(&self, features: &Features, builder: &mut LinkLibrariesBuilder) {
        linux::link_libraries(features, builder)
    }
}

fn flags(target: &Target) -> Vec<String> {
    let arch = &target.architecture;
    let cpp = "10.3.1";

    let flags = [
        format!("-I/usr/include/c++/{cpp}"),
        format!("-I/usr/include/c++/{cpp}/{arch}-alpine-linux-musl"),
    ];

    flags.into_iter().collect()
}
