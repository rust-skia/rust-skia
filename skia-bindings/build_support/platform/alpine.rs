use super::prelude::*;

pub fn musl_args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    let arch = &config.target.architecture;
    let cpp = "10.3.1";

    let flags = [
        format!("-I/usr/include/c++/{cpp}"),
        format!("-I/usr/include/c++/{cpp}/{arch}-alpine-linux-musl"),
    ];

    builder.skia_target_os_and_default_cpu(&config.target.system);

    builder.skia_cflags(flags.clone());
    builder.clang_args(flags);
}
