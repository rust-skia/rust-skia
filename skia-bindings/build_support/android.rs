use crate::build_support::cargo;
use std::env;

/// API level Android 8, Oreo (the first one with full Vulkan support)
pub const API_LEVEL: &str = "26";

pub fn ndk() -> String {
    env::var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}

pub fn additional_clang_args(target: &str, target_arch: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    match target_arch {
        "i686" => args.push("-m32".into()),
        "x86_64" => args.push("-m64".into()),
        _ => {}
    };

    match (cargo::host().system.as_str(), target_arch) {
        // under Linux hosts and x68 targets, we do have better chances without
        // any further configuration adjustments.
        // ("linux", "i686") => {}
        // ("linux", "x86_64") => {}
        _ => {
            let ndk = ndk();

            args.push(format!("--sysroot={}/sysroot", ndk));
            args.push(format!("-I{}/sysroot/usr/include/{}", ndk, target));
            // Adding C++ includes messes with LLVM 11 on macOS.
            if cargo::host().system != "darwin" {
                // TODO: test & support other CLang versions on macOS
                args.push(format!(
                    "-isystem{}/sources/cxx-stl/llvm-libc++/include",
                    ndk
                ));
            }
            args.push(format!("--target={}", target));
        }
    }

    args
}
