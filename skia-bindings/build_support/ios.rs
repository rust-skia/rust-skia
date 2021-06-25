use crate::build_support::clang;
use crate::build_support::features::Features;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// TODO: add support for 32 bit devices and simulators.
pub fn extra_skia_cflags(arch: &str, flags: &mut Vec<&str>) {
    if is_simulator(arch) {
        flags.push("-mios-simulator-version-min=10.0");
    } else {
        flags.push("-miphoneos-version-min=10.0");
    }
}

pub fn additional_clang_args(arch: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    if is_simulator(arch) {
        args.push("-mios-simulator-version-min=10.0".into());
        args.push("-m64".into());
    } else {
        args.push("-miphoneos-version-min=10.0".into());
        args.push("-arch".into());
        args.push(clang::target_arch(arch).into());
    }

    args.push("-isysroot".into());
    args.push(sdk_path(arch).to_str().unwrap().into());
    args.push("-fembed-bitcode".into());

    args
}

/// Resolve the iOS SDK path by starting xcrun.
fn sdk_path(arch: &str) -> PathBuf {
    let sdk_path = Command::new("xcrun")
        .arg("--show-sdk-path")
        .arg("--sdk")
        .arg(if is_simulator(arch) {
            "iphonesimulator"
        } else {
            "iphoneos"
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to invoke xcrun")
        .stdout;

    let string = String::from_utf8(sdk_path).expect("failed to resolve iOS SDK path");
    PathBuf::from(string.trim())
}

/// Returns true if the target architecture indicates that a simulator build is needed.
fn is_simulator(arch: &str) -> bool {
    matches!(arch, "x86_64")
}

pub(crate) fn link_libraries(features: &Features) -> Vec<&str> {
    let mut libs = vec![
        "c++",
        "framework=MobileCoreServices",
        "framework=CoreFoundation",
        "framework=CoreGraphics",
        "framework=CoreText",
        "framework=ImageIO",
        "framework=UIKit",
    ];

    if features.metal {
        libs.push("framework=Metal");
    }

    libs
}
