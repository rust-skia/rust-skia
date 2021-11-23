use crate::build_support::{clang, features::Features};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};
use Platform::*;

#[derive(Copy, Clone)]
enum Platform {
    IOSSimulator,
    IOSDevice,
    Catalyst,
}

impl Platform {
    fn new(arch: &str, abi: Option<&str>) -> Self {
        match () {
            () if abi == Some("macabi") => Catalyst,
            () if arch == "x86_64" => IOSSimulator,
            () => IOSDevice,
        }
    }

    fn flags(self) -> &'static str {
        match self {
            IOSSimulator => "-mios-simulator-version-min=10.0",
            IOSDevice => "-miphoneos-version-min=10.0",
            // If we go below 13.0, the Skia build emits warnings.
            Catalyst => "-miphoneos-version-min=13.0",
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
            IOSSimulator => "iphonesimulator",
            IOSDevice => "iphoneos",
            Catalyst => "macosx",
        }
    }
}

// TODO: add support for 32 bit devices and simulators.
pub fn extra_skia_cflags(arch: &str, abi: Option<&str>, flags: &mut Vec<&str>) {
    flags.push(Platform::new(arch, abi).flags());
}

pub fn additional_clang_args(arch: &str, abi: Option<&str>) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    let platform = Platform::new(arch, abi);

    args.push(platform.flags().into());

    match platform {
        IOSSimulator => {
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
