use super::{linux, prelude::*};

pub fn musl_args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    linux::args(config, builder);

    let arch = &config.target.architecture;
    let cpp = "10.3.1";

    let flags = [
        format!("-I/usr/include/c++/{cpp}"),
        format!("-I/usr/include/c++/{cpp}/{arch}-alpine-linux-musl"),
    ];

    builder.skia_cflags(flags.clone());
    builder.clang_args(flags);
}

pub fn musl_link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    linux::link_libraries(features, builder)
}
