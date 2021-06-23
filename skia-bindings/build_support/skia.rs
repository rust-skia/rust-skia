//! Full build support for the Skia library, SkiaBindings library and bindings.rs file.

use crate::build_support::{android, cargo, clang, ios, llvm, vs};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The libraries to link with.
mod lib {
    pub const SKIA: &str = "skia";
    pub const SK_SHAPER: &str = "skshaper";
    pub const SK_PARAGRAPH: &str = "skparagraph";
}

/// Feature identifiers define the additional configuration parts of the binaries to download.
mod feature_id {
    pub const GL: &str = "gl";
    pub const VULKAN: &str = "vulkan";
    pub const METAL: &str = "metal";
    pub const D3D: &str = "d3d";
    pub const TEXTLAYOUT: &str = "textlayout";
    pub const WEBPE: &str = "webpe";
    pub const WEBPD: &str = "webpd";
    pub const EGL: &str = "egl";
    pub const X11: &str = "x11";
    pub const WAYLAND: &str = "wayland";
}

/// The defaults for the Skia build configuration.
impl Default for BuildConfiguration {
    fn default() -> Self {
        let skia_debug = matches!(cargo::env_var("SKIA_DEBUG"), Some(v) if v != "0");

        BuildConfiguration {
            on_windows: cargo::host().is_windows(),
            skia_debug,
            // `OPT_LEVEL` is set by Cargo itself.
            opt_level: cargo::env_var("OPT_LEVEL"),
            features: Features {
                gl: cfg!(feature = "gl"),
                egl: cfg!(feature = "egl"),
                wayland: cfg!(feature = "wayland"),
                x11: cfg!(feature = "x11"),
                vulkan: cfg!(feature = "vulkan"),
                metal: cfg!(feature = "metal"),
                d3d: cfg!(feature = "d3d"),
                text_layout: cfg!(feature = "textlayout"),
                webp_encode: cfg!(feature = "webp-encode"),
                webp_decode: cfg!(feature = "webp-decode"),
                animation: false,
                dng: false,
                particles: false,
            },
            cc: cargo::env_var("CC").unwrap_or_else(|| "clang".to_string()),
            cxx: cargo::env_var("CXX").unwrap_or_else(|| "clang++".to_string()),
        }
    }
}

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
    pub features: Features,

    /// C compiler to use
    cc: String,

    /// C++ compiler to use
    cxx: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Features {
    /// Build with OpenGL support?
    pub gl: bool,

    /// Build with EGL support? If you set X11, setting this to false will use LibGL (GLX)
    pub egl: bool,

    /// Build with Wayland support? This requires EGL, as GLX does not work on Wayland.
    pub wayland: bool,

    /// Build with X11 support?
    pub x11: bool,

    /// Build with Vulkan support?
    pub vulkan: bool,

    /// Build with Metal support?
    pub metal: bool,

    /// Build with Direct3D support?
    pub d3d: bool,

    /// Features related to text layout. Modules skshaper and skparagraph.
    pub text_layout: bool,

    /// Support the encoding of bitmap data to the WEBP image format.
    pub webp_encode: bool,

    /// Support the decoding of the WEBP image format to bitmap data.
    pub webp_decode: bool,

    /// Build with animation support (yet unsupported, no wrappers).
    pub animation: bool,

    /// Support DNG file format (currently unsupported because of build errors).
    pub dng: bool,

    /// Build the particles module (unsupported, no wrappers).
    pub particles: bool,
}

impl Features {
    pub fn gpu(&self) -> bool {
        self.gl || self.vulkan || self.metal || self.d3d
    }

    /// Feature Ids used to look up prebuilt binaries.
    pub fn ids(&self) -> Vec<&str> {
        let mut feature_ids = Vec::new();

        if self.gl {
            feature_ids.push(feature_id::GL);
        }
        if self.egl {
            feature_ids.push(feature_id::EGL);
        }
        if self.x11 {
            feature_ids.push(feature_id::X11);
        }
        if self.wayland {
            feature_ids.push(feature_id::WAYLAND);
        }
        if self.vulkan {
            feature_ids.push(feature_id::VULKAN);
        }
        if self.metal {
            feature_ids.push(feature_id::METAL);
        }
        if self.d3d {
            feature_ids.push(feature_id::D3D);
        }
        if self.text_layout {
            feature_ids.push(feature_id::TEXTLAYOUT);
        }
        if self.webp_encode {
            feature_ids.push(feature_id::WEBPE);
        }
        if self.webp_decode {
            feature_ids.push(feature_id::WEBPD);
        }

        feature_ids
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
}

impl FinalBuildConfiguration {
    pub fn from_build_configuration(
        build: &BuildConfiguration,
        use_system_libraries: bool,
        skia_source_dir: &Path,
    ) -> FinalBuildConfiguration {
        let features = &build.features;

        let gn_args = {
            fn quote(s: &str) -> String {
                format!("\"{}\"", s)
            }

            let mut args: Vec<(&str, String)> = vec![
                ("is_official_build", yes_if(!build.skia_debug)),
                ("is_debug", yes_if(build.skia_debug)),
                ("skia_enable_gpu", yes_if(features.gpu())),
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

            let mut use_expat = true;

            // target specific gn args.
            let target = cargo::target();
            let target_str: &str = &format!("--target={}", target.to_string());
            let mut set_target = true;
            let sysroot_arg;
            let opt_level_arg;
            let mut cflags: Vec<&str> = vec![];
            let mut asmflags: Vec<&str> = vec![];

            if let Some(sysroot) = cargo::env_var("SDKROOT") {
                sysroot_arg = format!("--sysroot={}", sysroot);
                cflags.push(&sysroot_arg);
            }

            let jpeg_sys_cflags: Vec<String>;
            if cfg!(feature = "use-system-jpeg-turbo") {
                let paths = cargo::env_var("DEP_JPEG_INCLUDE").expect("mozjpeg-sys include path");
                jpeg_sys_cflags = std::env::split_paths(&paths)
                    .map(|arg| format!("-I{}", arg.display()))
                    .collect();
                cflags.extend(jpeg_sys_cflags.iter().map(|x| -> &str { x.as_ref() }));
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
                    opt_level_arg = format!("-O{}", opt_level);
                    cflags.push(&opt_level_arg);
                }
            }

            match target.as_strs() {
                (_, _, "windows", Some("msvc")) if build.on_windows => {
                    if let Some(win_vc) = vs::resolve_win_vc() {
                        args.push(("win_vc", quote(win_vc.to_str().unwrap())))
                    }
                    // Code on MSVC needs to be compiled differently (e.g. with /MT or /MD) depending on the runtime being linked.
                    // (See https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes)
                    // When static feature is enabled (target-feature=+crt-static) the C runtime should be statically linked
                    // and the compiler has to place the library name LIBCMT.lib into the .obj
                    // See https://docs.microsoft.com/en-us/cpp/build/reference/md-mt-ld-use-run-time-library?view=vs-2019
                    if cargo::target_crt_static() {
                        cflags.push("/MT");
                    } else {
                        // otherwise the C runtime should be linked dynamically
                        cflags.push("/MD");
                    }
                    // Tell Skia's build system where LLVM is supposed to be located.
                    if let Some(llvm_home) = llvm::win::find_llvm_home() {
                        args.push(("clang_win", quote(&llvm_home)));
                    } else {
                        panic!(
                            "Unable to locate LLVM installation. skia-bindings can not be built."
                        );
                    }
                }
                (arch, "linux", "android", _) | (arch, "linux", "androideabi", _) => {
                    args.push(("ndk", quote(&android::ndk())));
                    // TODO: make API-level configurable?
                    args.push(("ndk_api", android::API_LEVEL.into()));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                    args.push(("skia_use_system_freetype2", yes_if(use_system_libraries)));
                    args.push(("skia_enable_fontmgr_android", yes()));
                    // Enabling fontmgr_android implicitly enables expat.
                    // We make this explicit to avoid relying on an expat installed
                    // in the system.
                    use_expat = true;
                }
                (arch, _, "ios", _) => {
                    args.push(("target_os", quote("ios")));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                    ios::extra_skia_cflags(arch, &mut cflags);
                }
                (arch, _, os, _) => {
                    let skia_target_os = match os {
                        "darwin" => {
                            // Skia will take care to set a specific `-target` for the current macOS
                            // version. So we don't push another target `--target` that may
                            // conflict.
                            set_target = false;
                            "mac"
                        }
                        "windows" => "win",
                        _ => os,
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
                cflags.push(target_str);
                asmflags.push(target_str);
            }

            if !cflags.is_empty() {
                let cflags = format!(
                    "[{}]",
                    cflags.into_iter().map(quote).collect::<Vec<_>>().join(",")
                );
                args.push(("extra_cflags", cflags));
            }

            if !asmflags.is_empty() {
                let asmflags = format!(
                    "[{}]",
                    asmflags
                        .into_iter()
                        .map(quote)
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

/// The configuration of the resulting binaries.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinariesConfiguration {
    /// The feature identifiers we built with.
    pub feature_ids: Vec<String>,

    /// The output directory of the libraries we build and we need to inform cargo about.
    pub output_directory: PathBuf,

    /// The TARGET specific link libraries we need to inform cargo about.
    pub link_libraries: Vec<String>,

    /// The static Skia libraries skia-bindings provides and dependent projects need to link with.
    pub ninja_built_libraries: Vec<String>,

    /// The static Skia libraries skia-bindings provides and dependent projects need to link with.
    pub other_built_libraries: Vec<String>,

    /// Additional files relative to the output_directory
    /// that are needed to build dependent projects.
    pub additional_files: Vec<PathBuf>,

    /// `true` if the skia libraries are built with debugging information.
    pub skia_debug: bool,
}

const SKIA_OUTPUT_DIR: &str = "skia";
const ICUDTL_DAT: &str = "icudtl.dat";

impl BinariesConfiguration {
    /// Build a binaries configuration based on the current environment cargo
    /// supplies us with and a Skia build configuration.
    pub fn from_cargo_env(build: &BuildConfiguration) -> Self {
        let features = &build.features;
        let target = cargo::target();

        let mut ninja_built_libraries = Vec::new();
        let mut additional_files = Vec::new();
        let feature_ids = features.ids();

        if features.text_layout {
            if target.is_windows() {
                additional_files.push(ICUDTL_DAT.into());
            }
            ninja_built_libraries.push(lib::SK_PARAGRAPH.into());
            ninja_built_libraries.push(lib::SK_SHAPER.into());
        }

        let mut link_libraries = Vec::new();

        match target.as_strs() {
            (_, "unknown", "linux", _) => {
                link_libraries.extend(vec!["stdc++", "fontconfig", "freetype"]);
                if features.gl {
                    if features.egl {
                        link_libraries.push("EGL");
                    }

                    if features.x11 {
                        link_libraries.push("GL");
                    }

                    if features.wayland {
                        link_libraries.push("wayland-egl");
                        link_libraries.push("GLESv2");
                    }
                }
            }
            (_, "apple", "darwin", _) => {
                link_libraries.extend(vec!["c++", "framework=ApplicationServices"]);
                if features.gl {
                    link_libraries.push("framework=OpenGL");
                }
                if features.metal {
                    link_libraries.push("framework=Metal");
                    // MetalKit was added in m87 BUILD.gn.
                    link_libraries.push("framework=MetalKit");
                    link_libraries.push("framework=Foundation");
                }
            }
            (_, _, "windows", Some("msvc")) => {
                link_libraries.extend(&["usp10", "ole32", "user32", "gdi32", "fontsub"]);
                if features.gl {
                    link_libraries.push("opengl32");
                }
                if features.d3d {
                    link_libraries.extend(&["d3d12", "dxgi", "d3dcompiler"]);
                }
            }
            (_, "linux", "android", _) | (_, "linux", "androideabi", _) => {
                link_libraries.extend(android::link_libraries(features));
            }
            (_, "apple", "ios", _) => {
                link_libraries.extend(ios::link_libraries(features));
            }
            _ => panic!("unsupported target: {:?}", cargo::target()),
        };

        let output_directory = cargo::output_directory()
            .join(SKIA_OUTPUT_DIR)
            .to_str()
            .unwrap()
            .into();

        ninja_built_libraries.push(lib::SKIA.into());

        BinariesConfiguration {
            feature_ids: feature_ids.into_iter().map(|f| f.to_string()).collect(),
            output_directory,
            link_libraries: link_libraries
                .into_iter()
                .map(|lib| lib.to_string())
                .collect(),
            ninja_built_libraries,
            other_built_libraries: vec![],
            additional_files,
            skia_debug: build.skia_debug,
        }
    }

    /// Inform cargo that the library files of the given configuration are available and
    /// can be used as dependencies.
    pub fn commit_to_cargo(&self) {
        cargo::add_link_search(self.output_directory.to_str().unwrap());

        // On Linux, the order is significant, first the static libraries we built, and then
        // the system libraries.

        let target = cargo::target();

        cargo::add_static_link_libs(&target, &self.ninja_built_libraries);
        cargo::add_static_link_libs(&target, &self.other_built_libraries);
        cargo::add_link_libs(&self.link_libraries);
    }
}

/// Orchestrates the entire build of Skia based on the arguments provided.
pub fn build(
    build: &FinalBuildConfiguration,
    config: &BinariesConfiguration,
    ninja_command: Option<PathBuf>,
    gn_command: Option<PathBuf>,
    offline: bool,
) {
    let python2 = &prerequisites::locate_python2_cmd();
    println!("Python 2 found: {:?}", python2);

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
            Command::new(python2)
                .arg("skia/tools/git-sync-deps")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success(),
            "`skia/tools/git-sync-deps` failed"
        );
    }

    configure_skia(build, config, python2, gn_command.as_deref());
    build_skia(config, &ninja);
}

/// Configures Skia by calling gn
pub fn configure_skia(
    build: &FinalBuildConfiguration,
    config: &BinariesConfiguration,
    python2: &Path,
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
            &format!("--script-executable={}", python2.to_str().unwrap()),
            &format!("--args={}", gn_args),
        ])
        .envs(env::vars())
        .current_dir(&build.skia_source_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("gn error");

    if output.status.code() != Some(0) {
        panic!("{:?}", String::from_utf8(output.stdout).unwrap());
    }
}

/// Builds Skia.
///
/// This function assumes that all prerequisites are in place and that the output directory
/// contains a fully configured Skia source tree generated by gn.
pub fn build_skia(config: &BinariesConfiguration, ninja_command: &Path) {
    let ninja_status = Command::new(ninja_command)
        .args(&config.ninja_built_libraries)
        .args(&["-C", config.output_directory.to_str().unwrap()])
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
    pub fn locate_python2_cmd() -> PathBuf {
        const PYTHON_CMDS: [&str; 2] = ["python", "python2"];
        for python in PYTHON_CMDS.as_ref() {
            println!("Probing '{}'", python);
            if let Some(true) = is_python_version_2(python) {
                return python.into();
            }
        }

        panic!(">>>>> Probing for Python 2 failed, please make sure that it's available in PATH, probed executables are: {:?} <<<<<", PYTHON_CMDS);
    }

    /// Returns true if the given python executable is python version 2.
    /// or None if the executable was not found.
    fn is_python_version_2(exe: impl AsRef<str>) -> Option<bool> {
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
                str.starts_with("Python 2.")
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
