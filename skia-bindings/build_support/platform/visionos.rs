use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use super::prelude::*;

pub struct VisionOs;

// visionOS shipped at version 1.0; that is also the minimum deployment target.
const MIN_VISIONOS_VERSION: &str = "1.0";

impl PlatformDetails for VisionOs {
    fn uses_freetype(&self) -> bool {
        false
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let platform = VisionOsPlatform::new(&config.target);

        // Skia's GN build does not know about visionOS, so we build it as if it were iOS and
        // override the clang target triple (below) and the SDK sysroot to point at the visionOS
        // SDK. `target_os = "ios"` makes Skia take its Apple/iOS code paths, which are compatible
        // with visionOS.
        builder.target_os_and_default_cpu("ios");

        if platform.is_simulator() {
            builder.arg("ios_use_simulator", yes());
        }

        // Without this, Skia auto-resolves `xcode_sysroot` to the iphoneos/iphonesimulator SDK
        // (see `gn/skia/BUILD.gn`). Point it at the visionOS SDK instead.
        builder.arg(
            "xcode_sysroot",
            quote(platform.sdk_path().to_str().unwrap()),
        );

        // Override the target triple with the visionOS (`xros`) triple. This sets the
        // platform/version-min via the triple, so no separate `-m..-version-min` flag is needed.
        builder.target(platform.clang_target());
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        let platform = VisionOsPlatform::new(target);

        builder.arg("-isysroot");
        builder.arg(platform.sdk_path().to_str().unwrap().to_string());

        builder.override_target(&platform.clang_target());
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        let mut libs = vec![
            "c++",
            "framework=CoreFoundation",
            "framework=CoreGraphics",
            "framework=CoreText",
            "framework=ImageIO",
            "framework=UIKit",
        ];

        if features[feature::METAL] {
            libs.push("framework=Metal");
        }

        libs.iter().map(|s| s.to_string()).collect()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum VisionOsPlatform {
    Device,
    Simulator,
}

impl VisionOsPlatform {
    fn new(target: &Target) -> Self {
        // visionOS only runs on Apple Silicon, so the only simulator target is
        // `aarch64-apple-visionos-sim` (abi `sim`). There is no x86_64 visionOS simulator.
        match target.arch_abi() {
            (_, Some("sim")) => VisionOsPlatform::Simulator,
            _ => VisionOsPlatform::Device,
        }
    }

    fn is_simulator(self) -> bool {
        self == VisionOsPlatform::Simulator
    }

    /// The clang target triple for visionOS uses the `xros` OS name.
    fn clang_target(self) -> String {
        if self.is_simulator() {
            format!("arm64-apple-xros{MIN_VISIONOS_VERSION}-simulator")
        } else {
            format!("arm64-apple-xros{MIN_VISIONOS_VERSION}")
        }
    }

    fn sdk_name(self) -> &'static str {
        if self.is_simulator() {
            "xrsimulator"
        } else {
            "xros"
        }
    }

    /// Resolve the visionOS SDK path by starting `xcrun`.
    fn sdk_path(self) -> PathBuf {
        let sdk_path = Command::new("xcrun")
            .arg("--show-sdk-path")
            .arg("--sdk")
            .arg(self.sdk_name())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to invoke xcrun")
            .stdout;

        let string = String::from_utf8(sdk_path).expect("failed to resolve visionOS SDK path");
        PathBuf::from(string.trim())
    }
}
