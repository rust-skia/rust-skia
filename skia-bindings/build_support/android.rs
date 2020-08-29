use crate::build_support::cargo;
use crate::build_support::skia::Features;

/// API level Android 8, Oreo (the first one with full Vulkan support)
pub const API_LEVEL: &str = "26";

pub fn ndk() -> String {
    cargo::env_var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}

pub fn additional_clang_args(target: &str, target_arch: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    match target_arch {
        "i686" => args.push("-m32".into()),
        "x86_64" => args.push("-m64".into()),
        _ => {}
    };

    let ndk = ndk();
    // this is what's done in the skia.ninja file:
    args.push(format!("--sysroot={}/sysroot", ndk));
    args.push(format!("-I{}/sources/android/cpufeatures", ndk));
    // note: Adding C++ includes messes with Apple's CLang 11 in the binding generator,
    // Which means that only we support the official LLVM versions for Android builds on macOS.
    args.push(format!(
        "-isystem{}/sources/cxx-stl/llvm-libc++/include",
        ndk
    ));
    args.push(format!("--target={}", target));
    args
}

pub fn link_libraries(features: &Features) -> Vec<&str> {
    let mut libs = vec!["log", "android", "c++_static", "c++abi"];
    if features.gl {
        libs.extend(vec!["EGL", "GLESv2"])
    };
    libs
}
