use crate::build_support::{clang, features::Features};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};
use Platform::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Platform {
    IOSSimulator,
    IOSM1Simulator,
    IOSDevice,
    Catalyst,
}

impl Platform {
    fn new(arch: &str, abi: Option<&str>) -> Self {
        match () {
            () if abi == Some("macabi") => Catalyst,
            () if arch == "x86_64" => IOSSimulator,
            () if arch == "aarch64" && abi == Some("sim") => IOSM1Simulator,
            () => IOSDevice,
        }
    }

    fn flags(self) -> &'static str {
        match self {
            IOSDevice => "-miphoneos-version-min=10.0",
            IOSSimulator => "-mios-simulator-version-min=10.0",
            IOSM1Simulator => "-mios-simulator-version-min=14.0",
            // m100: XCode 13.2 fails to build with version 13
            Catalyst => "-miphoneos-version-min=14.0",
        }
    }

    /// Resolve the iOS SDK path by starting `xcrun`.
    fn sdk_path(self) -> PathBuf {
        let sdk_path = Command::new("xcrun")
            .arg("--show-sdk-path")
            .arg("--sdk")
            .arg(self.sdk_name())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to invoke xcrun")
            .stdout;

        let string = String::from_utf8(sdk_path).expect("failed to resolve iOS SDK path");
        PathBuf::from(string.trim())
    }

    fn sdk_name(self) -> &'static str {
        match self {
            IOSSimulator | IOSM1Simulator => "iphonesimulator",
            IOSDevice => "iphoneos",
            Catalyst => "macosx",
        }
    }
}

pub fn is_simulator(arch: &str, abi: Option<&str>) -> bool {
    Platform::new(arch, abi) == Platform::IOSSimulator
}

pub fn specific_target(arch: &str, abi: Option<&str>) -> Option<String> {
    if let IOSM1Simulator = Platform::new(arch, abi) {
        Some("arm64-apple-ios14.0.0-simulator".into())
    } else {
        None
    }
}

// TODO: add support for 32 bit devices and simulators.
pub fn extra_skia_cflags(arch: &str, abi: Option<&str>) -> Vec<String> {
    vec![Platform::new(arch, abi).flags().into()]
}

pub fn additional_clang_args(arch: &str, abi: Option<&str>) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    let platform = Platform::new(arch, abi);

    args.push(platform.flags().into());

    match platform {
        IOSSimulator | IOSM1Simulator => {
            args.push("-m64".into());
        }
        IOSDevice | Catalyst => {
            args.push("-arch".into());
            args.push(clang::target_arch(arch).into());
        }
    }

    args.push("-isysroot".into());
    args.push(platform.sdk_path().to_str().unwrap().into());
    args.push("-fembed-bitcode".into());

    args
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
        libs.extend(vec!["framework=MobileCoreServices", "framework=UIKit"]);
    }

    if features.metal {
        libs.push("framework=Metal");
    }

    libs
}
