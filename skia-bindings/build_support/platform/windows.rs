use super::prelude::*;
use crate::build_support::{
    cargo, clang,
    skia::{llvm, vs, BuildConfiguration},
};

pub fn msvc_args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    if let Some(win_vc) = vs::resolve_win_vc() {
        builder.skia(
            "win_vc",
            quote(
                win_vc
                    .to_str()
                    .expect("Failed to convert Windows compiler path to UTF-8"),
            ),
        );
    }

    // Tell Skia's build system where LLVM is supposed to be located.
    if let Some(llvm_home) = llvm::win::find_llvm_home() {
        builder.skia("clang_win", quote(&llvm_home));
    } else {
        panic!("Unable to locate LLVM installation. skia-bindings can not be built.");
    }

    // Setting `target_cpu` to `i686` or `x86`, nightly builds would lead to
    // > 'C:/Program' is not recognized as an internal or external command
    //
    // Without it, the executables pop out just fine. See the GH job
    // `supplemental-builds/windows-x86`.
    {
        let arch = &config.target.architecture;
        if arch != "i686" {
            builder.skia("target_cpu", quote(clang::target_arch(arch)));
        }
    }

    // Code on MSVC needs to be compiled differently (e.g. with /MT or /MD)
    // depending on the runtime being linked. (See
    // <https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes>)
    // When static feature is enabled (target-feature=+crt-static) the C runtime
    // should be statically linked and the compiler has to place the library name
    // LIBCMT.lib into the .obj See
    // <https://docs.microsoft.com/en-us/cpp/build/reference/md-mt-ld-use-run-time-library?view=vs-2019>
    let runtime_library = if cargo::target_crt_static() {
        "/MT"
    } else {
        // otherwise the C runtime should be linked dynamically
        "/MD"
    };

    builder.skia_cflag(runtime_library);
}

pub fn generic_args(_config: &BuildConfiguration, builder: &mut ArgBuilder) {
    builder.skia_target_os_and_default_cpu("win");
}
