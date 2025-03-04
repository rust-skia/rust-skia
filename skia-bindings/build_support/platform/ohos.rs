use std::path::PathBuf;

use super::{linux, prelude::*};

pub struct OpenHarmony;

/// For OpenHarmony, we recommend using API12 as the minimum API level
impl PlatformDetails for OpenHarmony {
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        linux::gn_args(config, builder);

        // disable fontconfig
        builder.arg("skia_use_fontconfig", no());

        builder.cflags(extra_skia_cflags());
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(additional_clang_args(
            &target.to_string(),
            &target.architecture,
        ));
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        link_libraries(features)
            .iter()
            .map(|l| l.to_string())
            .collect()
    }

    fn filter_platform_features(
        &self,
        use_system_libraries: bool,
        mut features: Features,
    ) -> Features {
        if !features.embed_freetype {
            features.embed_freetype = !use_system_libraries;
        }

        features
    }
}

fn ndk() -> String {
    // We can accpet the NDK path from the ohos_sdk_native or OHOS_NDK_HOME environment variable
    // OHOS_NDK_HOME should be the parent directory of the ndk directory
    if let Some(native) = cargo::env_var("ohos_sdk_native") {
        return native;
    }

    let home = cargo::env_var("OHOS_NDK_HOME").expect("ohos_sdk_native variable not set");
    PathBuf::from(home)
        .join("native")
        .to_string_lossy()
        .to_string()
}

pub fn additional_clang_args(_target: &str, target_arch: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    match target_arch {
        "arm" => {
            args.push("-march=armv7-a".into());
            args.push("-mfloat-abi=softfp".into());
            args.push("-mtune=generic-armv7-a".into());
            args.push("-mthumb".into());
        }
        "x86_64" => args.push("-m64".into()),
        _ => {}
    };

    let ndk = ndk();

    args.push(format!("-sysroot={ndk}/sysroot"));

    args.push(format!("--target=aarch64-linux-ohos"));
    args.extend(extra_skia_cflags());
    args
}

pub fn extra_skia_cflags() -> Vec<String> {
    vec![format!("-D__MUSL__")]
}

pub fn link_libraries(features: &Features) -> Vec<&str> {
    let mut libs = vec!["c++_static", "c++abi"];
    if features.gl {
        libs.extend(vec!["EGL", "GLESv3"])
    };
    libs
}
