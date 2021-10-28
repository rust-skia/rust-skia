//! Full build support for the Skia library.

use super::{llvm, vs};
use crate::build_support::cargo::Target;
use crate::build_support::{android, binaries_config, cargo, clang, features, ios};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The build configuration for Skia.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BuildConfiguration {
    /// Do we build _on_ a Windows OS?
    on_windows: bool,

    /// Set the optimization level (0-3, s or z). Clang and GCC use the same notation
    /// as Rust, so we just pass this option through from Cargo.
    opt_level: Option<String>,

    /// Build Skia in a debug configuration?
    skia_debug: bool,

    /// The Skia feature set to compile.
    features: features::Features,

    /// C compiler to use
    cc: String,

    /// C++ compiler to use
    cxx: String,

    /// The target (arch-vendor-os-abi)
    target: Target,
}

/// Builds a Skia configuration from a Features set.
impl BuildConfiguration {
    pub fn from_features(features: features::Features, skia_debug: bool) -> Self {
        // Yocto SDKs set CLANGCC/CLANGCXX, which is a better choice to determine clang,
        // as CC/CXX are likely referring to gcc.
        let cc = cargo::env_var("CLANGCC")
            .or_else(|| cargo::env_var("CC"))
            .unwrap_or_else(|| "clang".to_string());
        let cxx = cargo::env_var("CLANGCXX")
            .or_else(|| cargo::env_var("CXX"))
            .unwrap_or_else(|| "clang++".to_string());

        // It's possible that the provided command line for the compiler already includes --target.
        // We assume that it's most specific/appropriate, extract and use is. It might for example include
        // a vendor infix, while cargo targets usually don't.
        let target = cc
            .find("--target=")
            .map(|target_option_offset| {
                let target_tail = &cc[(target_option_offset + "--target=".len())..];
                let target_str = target_tail
                    .split_once(' ')
                    .map_or(target_tail, |(target_str, ..)| target_str);
                cargo::parse_target(target_str)
            })
            .unwrap_or_else(cargo::target);

        BuildConfiguration {
            on_windows: cargo::host().is_windows(),
            // `OPT_LEVEL` is set by Cargo itself.
            opt_level: cargo::env_var("OPT_LEVEL"),
            features,
            skia_debug,
            cc,
            cxx,
            target,
        }
    }
}

/// This is the final, low level build configuration.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinalBuildConfiguration {
    /// The Skia source directory.
    pub skia_source_dir: PathBuf,

    /// The name value pairs passed as arguments to gn.
    pub gn_args: Vec<(String, String)>,

    /// Whether to use system libraries or not.
    pub use_system_libraries: bool,

    /// The target (arch-vendor-os-abi)
    pub target: Target,

    /// An optional target sysroot
    pub sysroot: Option<String>,
}

impl FinalBuildConfiguration {
    pub fn from_build_configuration(
        build: &BuildConfiguration,
        use_system_libraries: bool,
        skia_source_dir: &Path,
    ) -> FinalBuildConfiguration {
        let features = &build.features;

        // SDKROOT is the environment variable used on macOS to specify the sysroot. SDKTARGETSYSROOT is the environment
        // variable set in Yocto Linux SDKs when cross-compiling.
        let sysroot = cargo::env_var("SDKTARGETSYSROOT").or_else(|| cargo::env_var("SDKROOT"));

        let gn_args = {
            fn quote(s: &str) -> String {
                format!("\"{}\"", s)
            }

            let mut args: Vec<(&str, String)> = vec![
                ("is_official_build", yes_if(!build.skia_debug)),
                ("is_debug", yes_if(build.skia_debug)),
                ("skia_enable_svg", yes_if(features.svg)),
                ("skia_enable_gpu", yes_if(features.gpu())),
                ("skia_enable_skottie", no()),
                ("skia_use_gl", yes_if(features.gl)),
                ("skia_use_egl", yes_if(features.egl)),
                ("skia_use_x11", yes_if(features.x11)),
                ("skia_use_system_libpng", yes_if(use_system_libraries)),
                ("skia_use_libwebp_encode", yes_if(features.webp_encode)),
                ("skia_use_libwebp_decode", yes_if(features.webp_decode)),
                ("skia_use_system_zlib", yes_if(use_system_libraries)),
                ("skia_use_xps", no()),
                ("skia_use_dng_sdk", yes_if(features.dng)),
                ("cc", quote(&build.cc)),
                ("cxx", quote(&build.cxx)),
            ];

            if features.vulkan {
                args.push(("skia_use_vulkan", yes()));
                args.push(("skia_enable_spirv_validation", no()));
            }

            if features.metal {
                args.push(("skia_use_metal", yes()));
            }

            if features.d3d {
                args.push(("skia_use_direct3d", yes()))
            }

            // further flags that limit the components of Skia debug builds.
            if build.skia_debug {
                args.push(("skia_enable_spirv_validation", no()));
                args.push(("skia_enable_tools", no()));
                args.push(("skia_enable_vulkan_debug_layers", no()));
                args.push(("skia_use_libheif", no()));
                args.push(("skia_use_lua", no()));
            }

            if features.text_layout {
                args.extend(vec![
                    ("skia_enable_skshaper", yes()),
                    ("skia_use_icu", yes()),
                    ("skia_use_system_icu", yes_if(use_system_libraries)),
                    ("skia_use_harfbuzz", yes()),
                    ("skia_pdf_subset_harfbuzz", yes()),
                    ("skia_use_system_harfbuzz", yes_if(use_system_libraries)),
                    ("skia_use_sfntly", no()),
                    ("skia_enable_skparagraph", yes()),
                    // note: currently, tests need to be enabled, because modules/skparagraph
                    // is not included in the default dependency configuration.
                    // ("paragraph_tests_enabled", no()),
                ]);
            } else {
                args.push(("skia_use_icu", no()));
            }

            if features.webp_encode || features.webp_decode {
                args.push(("skia_use_system_libwebp", yes_if(use_system_libraries)))
            }

            if features.embed_freetype {
                args.push(("skia_use_system_freetype2", no()));
            }

            let mut use_expat = true;

            // target specific gn args.
            let target = &build.target;
            let mut target_str = format!("--target={}", target);
            let mut set_target = true;
            let mut cflags: Vec<String> = Vec::new();
            let mut asmflags: Vec<String> = Vec::new();

            if let Some(sysroot) = &sysroot {
                cflags.push(format!("--sysroot={}", sysroot));
            }

            let jpeg_sys_cflags: Vec<String>;
            if cfg!(feature = "use-system-jpeg-turbo") {
                let paths = cargo::env_var("DEP_JPEG_INCLUDE").expect("mozjpeg-sys include path");
                jpeg_sys_cflags = std::env::split_paths(&paths)
                    .map(|arg| format!("-I{}", arg.display()))
                    .collect();
                cflags.extend(jpeg_sys_cflags);
                args.push(("skia_use_system_libjpeg_turbo", yes()));
            } else {
                args.push((
                    "skia_use_system_libjpeg_turbo",
                    yes_if(use_system_libraries),
                ));
            }

            if let Some(opt_level) = &build.opt_level {
                /* LTO generates corrupt libraries on the host platforms when building with --release
                if opt_level.parse::<usize>() != Ok(0) {
                    cflags.push("-flto");
                }
                */
                // When targeting windows `-O` isn't supported.
                if !target.is_windows() {
                    cflags.push(format!("-O{}", opt_level));
                }
            }

            match target.as_strs() {
                (arch, _, "windows", Some("msvc")) if build.on_windows => {
                    if let Some(win_vc) = vs::resolve_win_vc() {
                        args.push(("win_vc", quote(win_vc.to_str().unwrap())))
                    }
                    // Code on MSVC needs to be compiled differently (e.g. with /MT or /MD)
                    // depending on the runtime being linked. (See
                    // https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes)
                    // When static feature is enabled (target-feature=+crt-static) the C runtime
                    // should be statically linked and the compiler has to place the library name
                    // LIBCMT.lib into the .obj See
                    // https://docs.microsoft.com/en-us/cpp/build/reference/md-mt-ld-use-run-time-library?view=vs-2019
                    if cargo::target_crt_static() {
                        cflags.push("/MT".into());
                    } else {
                        // otherwise the C runtime should be linked dynamically
                        cflags.push("/MD".into());
                    }
                    // Tell Skia's build system where LLVM is supposed to be located.
                    if let Some(llvm_home) = llvm::win::find_llvm_home() {
                        args.push(("clang_win", quote(&llvm_home)));
                    } else {
                        panic!(
                            "Unable to locate LLVM installation. skia-bindings can not be built."
                        );
                    }
                    // Setting `target_cpu` to `i686` or `x86`, nightly builds would lead to
                    // > 'C:/Program' is not recognized as an internal or external command
                    // Without it, the executables pop out just fine. See the GH job
                    // `supplemental-builds/windows-x86`.
                    if arch != "i686" {
                        args.push(("target_cpu", quote(clang::target_arch(arch))));
                    }
                }
                (arch, "linux", "android", _) | (arch, "linux", "androideabi", _) => {
                    args.push(("ndk", quote(&android::ndk())));
                    args.push(("ndk_api", android::API_LEVEL.into()));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                    if !features.embed_freetype {
                        args.push(("skia_use_system_freetype2", yes_if(use_system_libraries)));
                    }
                    args.push(("skia_enable_fontmgr_android", yes()));
                    // Enabling fontmgr_android implicitly enables expat.
                    // We make this explicit to avoid relying on an expat installed
                    // in the system.
                    use_expat = true;
                    cflags.extend(android::extra_skia_cflags())
                }
                (arch, _, "ios", abi) => {
                    args.push(("target_os", quote("ios")));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                    // m100: Needed for aarch64 simulators, requires cherry Skia pick
                    // 0361abf39d1504966799b1cdb5450e07f88b2bc2 (until milestone 102).
                    if ios::is_simulator(arch, abi) {
                        args.push(("ios_use_simulator", yes()));
                    }
                    if let Some(specific_target) = ios::specific_target(arch, abi) {
                        target_str = format!("--target={}", specific_target);
                    }
                    cflags.extend(ios::extra_skia_cflags(arch, abi));
                }
                ("wasm32", "unknown", "emscripten", _) => {
                    args.push(("cc", quote("emcc")));
                    args.push(("cxx", quote("em++")));
                    args.push(("skia_gl_standard", quote("webgl")));
                    args.push(("skia_use_freetype", yes()));
                    args.push(("skia_use_system_freetype2", no()));
                    args.push(("skia_use_webgl", yes_if(features.gpu())));
                    args.push(("target_cpu", quote("wasm")));

                    // The custom embedded font manager is enabled by default on WASM, but depends
                    // on the undefined symbol `SK_EMBEDDED_FONTS`. Enable the custom empty font
                    // manager instead so typeface creation still works.
                    // See https://github.com/rust-skia/rust-skia/issues/648
                    args.push(("skia_enable_fontmgr_custom_embedded", no()));
                    args.push(("skia_enable_fontmgr_custom_empty", yes()));
                }
                (arch, _, os, abi) => {
                    let skia_target_os = match (os, abi) {
                        ("darwin", _) => {
                            // Skia will take care to set a specific `-target` for the current macOS
                            // version. So we don't push another target `--target` that may
                            // conflict.
                            set_target = false;
                            // Add macOS specific environment variables that affect the output of a
                            // build.
                            cargo::rerun_if_env_var_changed("MACOSX_DEPLOYMENT_TARGET");
                            "mac"
                        }
                        ("windows", _) => "win",
                        ("linux", Some("musl")) => {
                            let cpp = "10.3.1";
                            cflags.push(format!("-I/usr/include/c++/{}", cpp));
                            cflags.push(format!(
                                "-I/usr/include/c++/{}/{}-alpine-linux-musl",
                                cpp, arch
                            ));
                            os
                        }
                        (_, _) => os,
                    };
                    args.push(("target_os", quote(skia_target_os)));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                }
            }

            if use_expat {
                args.push(("skia_use_expat", yes()));
                args.push(("skia_use_system_expat", yes_if(use_system_libraries)));
            } else {
                args.push(("skia_use_expat", no()));
            }

            if set_target {
                cflags.push(target_str.clone());
                asmflags.push(target_str);
            }

            if !cflags.is_empty() {
                let cflags = format!(
                    "[{}]",
                    cflags
                        .into_iter()
                        .map(|s| quote(&s))
                        .collect::<Vec<_>>()
                        .join(",")
                );
                args.push(("extra_cflags", cflags));
            }

            if !asmflags.is_empty() {
                let asmflags = format!(
                    "[{}]",
                    asmflags
                        .into_iter()
                        .map(|s| quote(&s))
                        .collect::<Vec<_>>()
                        .join(",")
                );
                args.push(("extra_asmflags", asmflags));
            }

            args.into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect()
        };

        FinalBuildConfiguration {
            skia_source_dir: skia_source_dir.into(),
            gn_args,
            use_system_libraries,
            target: build.target.clone(),
            sysroot,
        }
    }
}

fn yes() -> String {
    "true".into()
}
fn no() -> String {
    "false".into()
}
fn yes_if(y: bool) -> String {
    if y {
        yes()
    } else {
        no()
    }
}

/// Orchestrates the entire build of Skia based on the arguments provided.
pub fn build(
    build: &FinalBuildConfiguration,
    config: &binaries_config::BinariesConfiguration,
    ninja_command: Option<PathBuf>,
    gn_command: Option<PathBuf>,
    offline: bool,
) {
    let python = &prerequisites::locate_python3_cmd();
    println!("Python 3 found: {:?}", python);

    let ninja = ninja_command.unwrap_or_else(|| {
        env::current_dir()
            .unwrap()
            .join("depot_tools")
            .join(ninja::default_exe_name())
    });

    if !offline && !build.use_system_libraries {
        println!("Synchronizing Skia dependencies");
        #[cfg(feature = "binary-cache")]
        crate::build_support::binary_cache::resolve_dependencies();
        assert!(
            Command::new(python)
                // Explicitly providing `GIT_SYNC_DEPS_PATH` fixes a problem with `git-sync-deps`
                // accidentally resolving an absolute directory for `GIT_SYNC_DEPS_PATH` when MingW
                // Python 3 runs on Windows under MSys.
                .env("GIT_SYNC_DEPS_PATH", "skia/DEPS")
                .arg("skia/tools/git-sync-deps")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success(),
            "`skia/tools/git-sync-deps` failed"
        );
    }

    configure_skia(build, config, python, gn_command.as_deref());
    build_skia(config, &ninja);
}

/// Configures Skia by calling gn
pub fn configure_skia(
    build: &FinalBuildConfiguration,
    config: &binaries_config::BinariesConfiguration,
    python: &Path,
    gn_command: Option<&Path>,
) {
    let gn_args = build
        .gn_args
        .iter()
        .map(|(name, value)| name.clone() + "=" + value)
        .collect::<Vec<String>>()
        .join(" ");

    let gn_command = gn_command
        .map(|p| p.to_owned())
        .unwrap_or_else(|| build.skia_source_dir.join("bin").join("gn"));

    println!("Skia args: {}", &gn_args);

    let output = Command::new(gn_command)
        .args(&[
            "gen",
            config.output_directory.to_str().unwrap(),
            &format!("--script-executable={}", python.to_str().unwrap()),
            &format!("--args={}", gn_args),
        ])
        .envs(env::vars())
        .current_dir(&build.skia_source_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("gn error");

    assert!(
        output.status.code() == Some(0),
        "{:?}",
        String::from_utf8(output.stdout).unwrap()
    );
}

/// Builds Skia.
///
/// This function assumes that all prerequisites are in place and that the output directory
/// contains a fully configured Skia source tree generated by gn.
pub fn build_skia(config: &binaries_config::BinariesConfiguration, ninja_command: &Path) {
    let ninja_status = Command::new(ninja_command)
        // Order of arguments do matter here: See <https://github.com/rust-skia/rust-skia/pull/643>
        // for details.
        .args(&["-C", config.output_directory.to_str().unwrap()])
        .args(&config.ninja_built_libraries)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    assert!(
        ninja_status
            .expect("failed to run `ninja`, does the directory depot_tools/ exist?")
            .success(),
        "`ninja` returned an error, please check the output for details."
    );
}

mod prerequisites {
    use std::path::PathBuf;
    use std::process::Command;

    /// Resolves the full path
    pub fn locate_python3_cmd() -> PathBuf {
        const PYTHON_CMDS: [&str; 2] = ["python", "python3"];
        for python in PYTHON_CMDS.as_ref() {
            println!("Probing '{}'", python);
            if let Some(true) = is_python_version_3(python) {
                return python.into();
            }
        }

        panic!(">>>>> Probing for Python 3 failed, please make sure that it's available in PATH, probed executables are: {:?} <<<<<", PYTHON_CMDS);
    }

    /// Returns `true` if the given python executable identifies itself as a python version 3
    /// executable. Returns `None` if the executable was not found.
    fn is_python_version_3(exe: impl AsRef<str>) -> Option<bool> {
        Command::new(exe.as_ref())
            .arg("--version")
            .output()
            .map(|output| {
                let mut str = String::from_utf8(output.stdout).unwrap();
                if str.is_empty() {
                    // Python2 seems to push the version to stderr.
                    str = String::from_utf8(output.stderr).unwrap()
                }
                // Don't parse version output, for example output
                // might be "Python 2.7.15+"
                str.starts_with("Python 3.")
            })
            .ok()
    }
}

mod ninja {
    use std::path::PathBuf;

    pub fn default_exe_name() -> PathBuf {
        if cfg!(windows) { "ninja.exe" } else { "ninja" }.into()
    }
}
