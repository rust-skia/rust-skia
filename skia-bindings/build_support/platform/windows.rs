use super::prelude::*;
use crate::build_support::{cargo, clang};
use std::path::PathBuf;

pub struct Msvc;

impl PlatformDetails for Msvc {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        if let Some(win_vc) = resolve_win_vc() {
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
        if let Some(llvm_home) = llvm::find_home() {
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

    fn link_libraries(&self, features: &Features, builder: &mut LinkLibrariesBuilder) {
        generic_link_libraries(features, builder)
    }
}

pub struct Generic;

impl PlatformDetails for Generic {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        generic_args(config, builder)
    }

    fn link_libraries(&self, features: &Features, builder: &mut LinkLibrariesBuilder) {
        generic_link_libraries(features, builder);
    }
}

pub fn generic_args(_config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
    builder.skia_target_os_and_default_cpu("win");
}

pub fn generic_link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    builder.link_libraries(["usp10", "ole32", "user32", "gdi32", "fontsub"]);
    if features.gl {
        builder.link_library("opengl32");
    }
    if features.d3d {
        builder.link_libraries(["d3d12", "dxgi", "d3dcompiler"]);
    }
}

/// Visual Studio detection support
/// TODO: sophisticate: <https://github.com/alexcrichton/cc-rs/blob/master/src/windows_registry.rs0>
fn resolve_win_vc() -> Option<PathBuf> {
    if let Some(install_dir) = cargo::env_var("VCINSTALLDIR") {
        return Some(PathBuf::from(install_dir));
    }

    let releases = [("Program Files", "2022"), ("Program Files (x86)", "2019")];
    let editions = ["BuildTools", "Enterprise", "Professional", "Community"];

    releases
        .iter()
        .flat_map(|r| editions.iter().map(move |e| (r, e)))
        .map(|((rp, r), ed)| format!("C:\\{}\\Microsoft Visual Studio\\{}\\{}\\VC", rp, r, ed))
        .map(PathBuf::from)
        .find(|pb| pb.exists())
}

mod llvm {
    use std::path::PathBuf;

    use crate::build_support::cargo;

    /// Locate the LLVM installation which can be used to build skia.
    /// If the environment variable `LLVM_HOME` is present it will
    /// be used. Otherwise we search a set of common paths:
    ///   - C:\Program Files\LLVM
    ///   - C:\LLVM
    ///   - %USERPROFILE%\scoop\apps\llvm\current
    pub fn find_home() -> Option<String> {
        if let Some(llvm_home) = cargo::env_var("LLVM_HOME") {
            validate_home(&llvm_home)
        } else {
            // USERPROFILE *should* equate to HOME and always be defined.
            // TODO: If this isn't defined, should we try c:\Users\%USERNAME%?
            let userprofile =
                cargo::env_var("USERPROFILE").expect("Unable to resolve %USERPROFILE%");
            let common_roots = [
                String::from("C:\\Program Files\\LLVM"),
                String::from("C:\\LLVM"),
                format!("{}\\scoop\\apps\\llvm\\current", userprofile),
            ];
            for root in &common_roots {
                let root = validate_home(root);
                if root.is_some() {
                    return root;
                }
            }
            None
        }
    }

    fn validate_home(home: &str) -> Option<String> {
        let clang_cl: PathBuf = [home, "bin", "clang-cl.exe"].into_iter().collect();
        eprintln!("Checking for {:?}", clang_cl);
        clang_cl.exists().then(|| home.to_string())
    }
}
