use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use super::{apple, prelude::*};

pub struct MacOs;

impl PlatformDetails for MacOs {
    fn uses_freetype(&self) -> bool {
        false
    }

    fn gn_args(&self, _config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        builder.target_os_and_default_cpu("mac");
        builder.cflags(flags());
    }

    fn bindgen_args(&self, _target: &Target, builder: &mut BindgenArgsBuilder) {
        // macOS uses `-isysroot/path/to/sysroot`, but this doesn't appear
        // to work for other targets. `--sysroot=` works for all targets,
        // to my knowledge, but doesn't seem to be idiomatic for macOS
        // compilation. To capture this, we allow manually setting sysroot
        // on any platform, but we use `-isysroot` for OSX builds and `--sysroot`
        // elsewhere. If you don't manually set the sysroot, we can automatically
        // detect it, but this is only possible for macOS.
        builder.sysroot_prefix("-isysroot");

        if builder.sysroot().is_none() {
            if let Some(macos_sdk) = get_sdk_path("macosx") {
                let sdk = macos_sdk;
                builder.set_sysroot(
                    sdk.to_str()
                        .expect("macOS SDK path could not be converted to string"),
                );
            } else {
                cargo::warning("failed to get macosx SDK path")
            }
        }

        if let Some(sdk) = builder.sysroot().map(PathBuf::from) {
            apple::use_sdk_libcxx(builder, &sdk);
        }

        builder.args(flags());
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        let mut libs = vec!["c++", "framework=ApplicationServices"];
        if features[feature::GL] {
            libs.push("framework=OpenGL");
        }
        if features[feature::METAL] {
            libs.push("framework=Metal");
            // MetalKit was added in m87 BUILD.gn.
            libs.push("framework=MetalKit");
            libs.push("framework=Foundation");
        }

        libs.iter().map(|l| l.to_string()).collect()
    }
}

fn flags() -> Vec<String> {
    let deployment_target = cargo::env_var("MACOSX_DEPLOYMENT_TARGET");

    if let Some(deployment_target) = deployment_target {
        return vec![format!("-mmacosx-version-min={deployment_target}")];
    }
    Vec::new()
}

/// Returns the current SDK path.
pub fn get_sdk_path(sdk: impl AsRef<str>) -> Option<PathBuf> {
    let mut cmd = Command::new("xcrun");
    cmd.arg("--sdk").arg(sdk.as_ref()).arg("--show-sdk-path");
    let output = cmd.stderr(Stdio::inherit()).output().ok()?;
    if output.status.code() != Some(0) {
        return None;
    }
    Some({
        let str = String::from_utf8(output.stdout).unwrap();
        PathBuf::from(str.trim())
    })
}
