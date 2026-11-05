use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use super::prelude::{BindgenArgsBuilder, cargo};

pub fn use_sdk_libcxx(builder: &mut BindgenArgsBuilder, sdk: &Path) {
    // Bindgen may load a non-Apple libclang, whose default C++ headers take precedence over the
    // Apple SDK selected by -isysroot. Mixing those headers with the SDK can fail when Xcode and
    // libclang ship incompatible libc++ versions, so make Bindgen use the SDK's libc++ explicitly.
    builder.bindgen_only_arg("-nostdinc++");
    builder.bindgen_only_arg(format!("-isystem{}/usr/include/c++/v1", sdk.display()));
}

/// Adds Xcode's clang runtime library for `platform` (`libclang_rt.<platform>.a`, for example
/// `osx` or `iossim`) to the link search path, and returns the link library entry for it.
///
/// Skia's Objective-C code checks the OS version with `@available`. When the deployment target is
/// lower than the checked version, clang compiles the check into a call to
/// `__isPlatformVersionAtLeast`, which the clang runtime library defines. Clang links that library
/// into every program it links, but rustc links with `-nodefaultlibs`, which leaves it out. Rust's
/// standard library defines a weak fallback of the function, but link-time optimization drops it,
/// so a release build with LTO fails to link without the runtime library.
///
/// The library is linked unbundled, so that it stays a system library of the final link instead
/// of being copied into the `skia-bindings` rlib. The linker only takes the archive members that
/// resolve undefined symbols, so the library adds nothing when no `@available` check needs it.
///
/// Returns `None` and emits a warning if `xcrun clang` does not report a runtime directory that
/// contains the library.
pub fn clang_runtime_library(platform: &str) -> Option<String> {
    let file_name = format!("libclang_rt.{platform}.a");
    let Some(runtime_dir) = clang_runtime_dir().filter(|dir| dir.join(&file_name).is_file()) else {
        cargo::warning(format!(
            "{file_name} was not found in the clang runtime directory, Skia's `@available` checks \
             may fail to link"
        ));
        return None;
    };
    cargo::add_link_search(format!("native={}", runtime_dir.display()));
    Some(format!("static:-bundle=clang_rt.{platform}"))
}

/// Returns the directory of the clang runtime libraries of Xcode's clang.
fn clang_runtime_dir() -> Option<PathBuf> {
    let output = Command::new("xcrun")
        .args(["clang", "--print-runtime-dir"])
        .stderr(Stdio::inherit())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let dir = String::from_utf8(output.stdout).ok()?;
    Some(PathBuf::from(dir.trim()))
}
