use super::prelude::*;
use crate::build_support::clang;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct Ios;

impl PlatformDetails for Ios {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let (arch, abi) = config.target.arch_abi();

        builder.target_os_and_default_cpu("ios");

        // m100: Needed for aarch64 simulators, requires cherry Skia pick
        // 0361abf39d1504966799b1cdb5450e07f88b2bc2 (until milestone 102).
        if is_simulator(arch, abi) {
            builder.arg("ios_use_simulator", yes());
        }

        builder.cflags(extra_skia_cflags(arch, abi));

        if let Some(specific_target) = specific_target(arch, abi) {
            builder.target(specific_target);
        }
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(additional_clang_args(
            &target.architecture,
            target.abi.as_deref(),
        ))
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        let abi = cargo::target().abi;
        let mut libs = vec![
            "c++",
            "framework=CoreFoundation",
            "framework=CoreGraphics",
            "framework=CoreText",
            "framework=ImageIO",
        ];

        if abi.as_deref() != Some("macabi") {
            libs.extend(vec!["framework=MobileCoreServices", "framework=UIKit"]);
        }

        if features.metal {
            libs.push("framework=Metal");
        }

        libs.iter().map(|s| s.to_string()).collect()
    }
}

fn is_simulator(arch: &str, abi: Option<&str>) -> bool {
    IosPlatform::new(arch, abi) == IosPlatform::Simulator
}

fn specific_target(arch: &str, abi: Option<&str>) -> Option<String> {
    (IosPlatform::new(arch, abi) == IosPlatform::M1Simulator)
        .then(|| "arm64-apple-ios14.0.0-simulator".into())
}

// TODO: add support for 32 bit devices and simulators.
fn extra_skia_cflags(arch: &str, abi: Option<&str>) -> Vec<String> {
    vec![IosPlatform::new(arch, abi).flags().into()]
}

fn additional_clang_args(arch: &str, abi: Option<&str>) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    let platform = IosPlatform::new(arch, abi);

    args.push(platform.flags().into());

    use IosPlatform::*;
    match platform {
        Simulator | M1Simulator => {
            args.push("-m64".into());
        }
        Device | Catalyst => {
            args.push("-arch".into());
            args.push(clang::target_arch(arch).into());
        }
    }

    args.push("-isysroot".into());
    args.push(platform.sdk_path().to_str().unwrap().into());
    args.push("-fembed-bitcode".into());

    args
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum IosPlatform {
    Simulator,
    M1Simulator,
    Device,
    Catalyst,
}

impl IosPlatform {
    fn new(arch: &str, abi: Option<&str>) -> Self {
        use IosPlatform::*;
        match () {
            () if abi == Some("macabi") => Catalyst,
            () if arch == "x86_64" => Simulator,
            () if arch == "aarch64" && abi == Some("sim") => M1Simulator,
            () => Device,
        }
    }

    fn flags(self) -> &'static str {
        use IosPlatform::*;
        match self {
            Device => "-miphoneos-version-min=10.0",
            Simulator => "-mios-simulator-version-min=10.0",
            M1Simulator => "-mios-simulator-version-min=14.0",
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
        use IosPlatform::*;
        match self {
            Simulator | M1Simulator => "iphonesimulator",
            Device => "iphoneos",
            Catalyst => "macosx",
        }
    }
}
