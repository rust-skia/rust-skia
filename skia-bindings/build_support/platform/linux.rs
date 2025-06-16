use super::{generic, prelude::*};
use pkg_config;

pub struct Linux;

impl PlatformDetails for Linux {
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        gn_args(config, builder);

        let target = &config.target;
        builder.cflags(flags(target));
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(flags(target))
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        link_libraries(features)
    }
}

pub fn gn_args(config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
    generic::gn_args(config, builder);
}

pub fn link_libraries(features: &Features) -> Vec<String> {
    let mut libs = vec!["stdc++".to_string()];

    // Use pkg-config for system libraries when available
    add_pkg_config_libs(&mut libs, "freetype2", &["freetype"]);
    add_pkg_config_libs(&mut libs, "fontconfig", &["fontconfig"]);

    if features.gl {
        if features.egl {
            add_pkg_config_libs(&mut libs, "egl", &["EGL"]);
        }

        if features.x11 {
            add_pkg_config_libs(&mut libs, "gl", &["GL"]);
        }

        if features.wayland {
            add_pkg_config_libs(&mut libs, "wayland-egl", &["wayland-egl"]);
            libs.push("GLESv2".to_string()); // Fallback for GLESv2
        }
    }

    if skia::env::use_system_libraries() {
        // Use pkg-config for these libraries
        add_pkg_config_libs(&mut libs, "libpng", &["png16"]);
        add_pkg_config_libs(&mut libs, "zlib", &["z"]);
        add_pkg_config_libs(&mut libs, "harfbuzz", &["harfbuzz"]);
        add_pkg_config_libs(&mut libs, "expat", &["expat"]);

        // ICU libraries - try pkg-config first, fallback to manual linking
        add_pkg_config_libs(&mut libs, "icu-uc", &["icuuc"]);
        add_pkg_config_libs(&mut libs, "icu-i18n", &["icui18n"]);
        add_pkg_config_libs(&mut libs, "icu-io", &["icuio"]);
        // Note: removed icutest and icutu as they appear to be development/testing utilities

        if features.webp_encode || features.webp_decode {
            add_pkg_config_libs(&mut libs, "libwebp", &["webp"]);
        }
    }

    if skia::env::use_system_libraries() || cfg!(feature = "use-system-jpeg-turbo") {
        libs.push("jpeg".to_string());
    }

    libs
}

fn add_pkg_config_libs(libs: &mut Vec<String>, pkg_name: &str, fallback_libs: &[&str]) {
    match pkg_config::probe_library(pkg_name) {
        Ok(library) => {
            for lib in &library.libs {
                libs.push(lib.clone());
            }
        }
        Err(_) => {
            // Fallback to hardcoded library names
            for lib in fallback_libs {
                libs.push(lib.to_string());
            }
        }
    }
}

fn flags(target: &Target) -> Vec<String> {
    // Set additional includes when cross-compiling.
    // TODO: Resolve the C++ version. This is specific to Ubuntu 18.
    if *target != cargo::host() {
        let cpp = "7";
        let target_path_component = target.include_path_component();
        [
            format!("-I/usr/{target_path_component}/include/c++/{cpp}/{target_path_component}"),
            format!("-I/usr/{target_path_component}/include"),
        ]
        .into()
    } else {
        Vec::new()
    }
}
