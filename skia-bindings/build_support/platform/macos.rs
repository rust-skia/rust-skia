use super::prelude::*;
use crate::build_support::{macos, xcode};

pub fn args(_config: &BuildConfiguration, builder: &mut ArgBuilder) {
    // Skia will take care to set a specific `--target` for the current macOS version. So we
    // don't push another target `--target` that may conflict.
    builder.target(None);

    // Add macOS specific environment variables that affect the output of a
    // build.
    cargo::rerun_if_env_var_changed("MACOSX_DEPLOYMENT_TARGET");

    builder.skia_target_os_and_default_cpu("mac");

    // macOS uses `-isysroot/path/to/sysroot`, but this doesn't appear
    // to work for other targets. `--sysroot=` works for all targets,
    // to my knowledge, but doesn't seem to be idiomatic for macOS
    // compilation. To capture this, we allow manually setting sysroot
    // on any platform, but we use `-isysroot` for OSX builds and `--sysroot`
    // elsewhere. If you don't manually set the sysroot, we can automatically
    // detect it, but this is only possible for macOS.
    builder.sysroot_prefix("-isysroot");

    if builder.sysroot().is_none() {
        if let Some(macos_sdk) = xcode::get_sdk_path("macosx") {
            let sdk = macos_sdk;
            builder.set_sysroot(
                sdk.to_str()
                    .expect("macOS SDK path could not be converted to string"),
            );
        } else {
            cargo::warning("failed to get macosx SDK path")
        }
    }

    builder.skia_cflags(macos::extra_skia_cflags());
    builder.clang_args(macos::additional_clang_args());
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    builder.link_libraries(["c++", "framework=ApplicationServices"]);
    if features.gl {
        builder.link_library("framework=OpenGL");
    }
    if features.metal {
        builder.link_library("framework=Metal");
        // MetalKit was added in m87 BUILD.gn.
        builder.link_library("framework=MetalKit");
        builder.link_library("framework=Foundation");
    }
}
