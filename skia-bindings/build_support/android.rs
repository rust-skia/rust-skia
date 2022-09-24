use std::fs::File;
use std::io::Read;
use std::path::Path;

use regex::Regex;

use crate::build_support::cargo;
use crate::build_support::features::Features;

/// API level Android 8, Oreo (the first one with full Vulkan support)
pub const API_LEVEL: &str = "26";

pub fn ndk() -> String {
    cargo::env_var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}

fn host_tag() -> String {
    // Because this is part of build.rs, the target_os is actually the host system 
    if cfg!(target_os = "windows") {
        "windows-x86_64"
    }
    else if cfg!(target_os = "linux") {
        "linux-x86_64"
    }
    else if cfg!(target_os = "macos") {
        "darwin-x86_64"
    }
    else {
        panic!("host os is not supported")
    }.to_string()
}
/// Get NDK major version from source.properties
fn ndk_major_version(ndk_dir: &Path) -> u32 {
    // Capture version from the line with Pkg.Revision
    let re = Regex::new(r"Pkg.Revision = (\d+)\.(\d+)\.(\d+)").unwrap();
    // There's a source.properties file in the ndk directory, which contains 
    let mut source_properties = File::open(ndk_dir.join("source.properties")).expect("Couldn't open source.properties");
    let mut buf = "".to_string();
    source_properties.read_to_string(&mut buf).expect("Could not read source.properties");
    // Capture version info
    let captures = re.captures(&buf).expect("source.properties did not match the regex");
    // Capture 0 is the whole line of text
    let major = captures[1].parse().expect("could not parse major version");
    major
}

pub fn extra_skia_cflags() -> Vec<String> {
    vec![format!("-D__ANDROID_API__={}", API_LEVEL)]
}

pub fn additional_clang_args(target: &str, target_arch: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    match target_arch {
        "i686" => args.push("-m32".into()),
        "x86_64" => args.push("-m64".into()),
        _ => {}
    };

    let ndk = ndk();
    let major = ndk_major_version(Path::new(&ndk));
    // Version 22 is the first version that moved sysroot to toolchains folder
    if major < 22 {
        // sysroot is just in the ndk directory
        args.push(format!("--sysroot={}/sysroot", ndk));
        // note: Adding C++ includes messes with Apple's CLang 11 in the binding generator,
        // Which means that only we support the official LLVM versions for Android builds on macOS.
        args.push(format!(
            "-isystem{}/sources/cxx-stl/llvm-libc++/include",
            ndk
        ));
    } 
    else {
        // NDK versions >= 22 have the sysroot in the llvm prebuilt by 
        let host_toolchain = format!("{}/toolchains/llvm/prebuilt/{}", ndk, host_tag());
        // sysroot is stored in the prebuilt llvm, under the host 
        args.push(format!("--sysroot={}/sysroot", host_toolchain));
    };
    args.push(format!("-I{}/sources/android/cpufeatures", ndk));
    
    args.push(format!("--target={}", target));
    args.extend(extra_skia_cflags());
    args
}

pub fn link_libraries(features: &Features) -> Vec<&str> {
    let mut libs = vec!["log", "android", "c++_static", "c++abi"];
    if features.gl {
        libs.extend(vec!["EGL", "GLESv2"])
    };
    libs
}
