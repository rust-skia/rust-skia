use crate::build_support::clang;
use crate::build_support::features::Features;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// TODO: add support for 32 bit devices and simulators.
pub fn extra_skia_cflags(arch: &str, abi: Option<&str>, flags: &mut Vec<&str>) {
    if is_simulator(arch, abi) {
        flags.push("-mios-simulator-version-min=10.0");
    } else {
        flags.push("-miphoneos-version-min=10.0");
    }
}

pub fn additional_clang_args(arch: &str, abi: Option<&str>) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    if is_simulator(arch, abi) {
        args.push("-mios-simulator-version-min=10.0".into());
        args.push("-m64".into());
    } else {
        args.push("-miphoneos-version-min=10.0".into());
        args.push("-arch".into());
        args.push(clang::target_arch(arch).into());
    }

    args.push("-isysroot".into());
    args.push(sdk_path(arch, abi).to_str().unwrap().into());
    args.push("-fembed-bitcode".into());

    args
}

/// Resolve the iOS SDK path by starting `xcrun`.
fn sdk_path(arch: &str, abi: Option<&str>) -> PathBuf {
    let sdk_path = Command::new("xcrun")
        .arg("--show-sdk-path")
        .arg("--sdk")
        .arg(sdk_name(arch, abi))
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to invoke xcrun")
        .stdout;

    let string = String::from_utf8(sdk_path).expect("failed to resolve iOS SDK path");
    PathBuf::from(string.trim())
}

/// Returns `true` if the target architecture indicates that a simulator build is needed.
fn is_simulator(arch: &str, abi: Option<&str>) -> bool {
    sdk_name(arch, abi) == "iphonesimulator"
}

fn sdk_name(arch: &str, abi: Option<&str>) -> &'static str {
    match () {
        () if abi == Some("macabi") => "macosx",
        () if arch == "x86_64" => "iphonesimulator",
        () => "iphoneos",
    }
}

pub(crate) fn link_libraries(abi: Option<&str>, features: &Features) -> Vec<&'static str> {
    let mut libs = vec![
        "c++",
        "framework=CoreFoundation",
        "framework=CoreGraphics",
        "framework=CoreText",
        "framework=ImageIO",
    ];

    if abi != Some("macabi") {
        libs.extend(["framework=MobileCoreServices", "framework=UIKit"]);
    }

    if features.metal {
        libs.push("framework=Metal");
    }

    libs
}
