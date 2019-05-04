//! Full build support for the Skia library, SkiaBindings library and bindings.rs file.

use crate::build_support::cargo;
use bindgen::EnumVariation;
use cc::Build;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BINDINGS_LIB_NAME: &str = "skia-bindings";
const REPOSITORY_CLONE_URL: &str = "https://github.com/rust-skia/rust-skia.git";
const REPOSITORY_DIRECTORY: &str = "rust-skia";

mod build {
    /// Do we build _on_ a Windows OS?
    pub const ON_WINDOWS: bool = cfg!(windows);

    /// Build Skia in a release configuration?
    /// Note that currently, we don't support debug Skia builds.
    pub const SKIA_RELEASE: bool = true;

    /// Configure Skia builds to keep inline functions to
    /// prevent mean linker errors.
    pub const KEEP_INLINE_FUNCTIONS: bool = true;

    /// Build with Vulkan support?
    pub const VULKAN: bool = cfg!(feature = "vulkan");

    /// Build with SVG support?
    pub const SVG: bool = cfg!(feature = "svg");

    /// Build with animation support.
    pub const ANIMATION: bool = false;

    /// Support DNG file format.
    pub const DNG: bool = false;

    /// Build the particles module.
    pub const PARTICLES: bool = false;
}

pub struct Configuration {
    /// The features we build with.
    pub features: Vec<String>,

    /// The output directory of the libraries we build and we need to inform cargo about.
    pub output_directory: PathBuf,

    /// The TARGET specific link libraries we need to inform cargo about.
    pub link_libraries: Vec<String>,

    // TODO: this can be resolved in build() and is not relevant for prebuilt binaries.
    /// Build _all_ skia libraries just to avoid build script errors.
    pub build_all_skia_libs: bool,
}

impl Configuration {
    /// Build a configuration based on the current environment cargo supplies us with.
    pub fn from_cargo_env() -> Self {
        let mut features = Vec::new();
        if build::VULKAN {
            features.push("vulkan");
        }
        if build::SVG {
            features.push("svg")
        }

        let mut build_all_skia_libs: bool = false;
        let mut link_libraries = Vec::new();

        match cargo::target().as_str() {
            (_, "unknown", "linux", Some("gnu")) => {
                link_libraries.extend(vec!["stdc++", "bz2", "GL", "fontconfig", "freetype"]);
            }
            (_, "apple", "darwin", _) => {
                link_libraries.extend(vec![
                    "c++",
                    "framework=OpenGL",
                    "framework=ApplicationServices",
                ]);
                // m74: if we don't build the particles or the skottie library on macOS, the build fails with
                // for example:
                // [763/867] link libparticles.a
                // FAILED: libparticles.a
                build_all_skia_libs = true;
            }
            (_, _, "windows", Some("msvc")) => {
                link_libraries.extend(vec![
                    "usp10", "ole32", "user32", "gdi32", "fontsub", "opengl32",
                ]);
            }
            _ => panic!("unsupported target: {:?}", cargo::target()),
        };

        let output_directory = cargo::output_directory()
            .join("skia")
            .to_str()
            .unwrap()
            .into();

        Configuration {
            features: features.iter().map(|f| f.to_string()).collect(),
            output_directory,
            link_libraries: link_libraries.iter().map(|lib| lib.to_string()).collect(),
            build_all_skia_libs,
        }
    }

    /// Inform cargo that the output files of the given configuration are available and
    /// can be used as dependencies.
    pub fn commit_to_cargo(&self) {
        cargo::add_link_libs(&self.link_libraries);
        println!(
            "cargo:rustc-link-search={}",
            self.output_directory.to_str().unwrap()
        );
        cargo::add_link_lib("static=skia");
        cargo::add_link_lib(&format!("static={}", BINDINGS_LIB_NAME));
    }
}

/// The full build of Skia, SkiaBindings, and the generation of bindings.rs.
/// Returns the output directory of the libraries.
pub fn build(config: &Configuration) {
    prerequisites::require_python();
    prerequisites::get_skia();

    assert!(
        Command::new("python")
            .arg("skia/tools/git-sync-deps")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success(),
        "`skia/tools/git-sync-deps` failed"
    );

    let gn_args = {
        fn yes() -> String {
            "true".into()
        }
        fn no() -> String {
            "false".into()
        }

        fn quote(s: &str) -> String {
            format!("\"{}\"", s)
        };

        let force_build_libs = config.build_all_skia_libs;

        let mut args: Vec<(&str, String)> = vec![
            (
                "is_official_build",
                if build::SKIA_RELEASE { yes() } else { no() },
            ),
            ("skia_use_expat", if build::SVG { yes() } else { no() }),
            ("skia_use_system_expat", no()),
            ("skia_use_icu", no()),
            ("skia_use_system_libjpeg_turbo", no()),
            ("skia_use_system_libpng", no()),
            ("skia_use_libwebp", no()),
            ("skia_use_system_zlib", no()),
            (
                "skia_enable_skottie",
                if build::ANIMATION || force_build_libs {
                    yes()
                } else {
                    no()
                },
            ),
            ("skia_use_xps", no()),
            ("skia_use_dng_sdk", if build::DNG { yes() } else { no() }),
            (
                "skia_enable_particles",
                if build::PARTICLES || force_build_libs {
                    yes()
                } else {
                    no()
                },
            ),
            ("cc", quote("clang")),
            ("cxx", quote("clang++")),
        ];

        // further flags that limit the components of Skia debug builds.
        if !build::SKIA_RELEASE {
            args.push(("skia_enable_atlas_text", no()));
            args.push(("skia_enable_spirv_validation", no()));
            args.push(("skia_enable_tools", no()));
            args.push(("skia_enable_vulkan_debug_layers", no()));
            args.push(("skia_use_libheif", no()));
            args.push(("skia_use_lua", no()));
        }

        if build::VULKAN {
            args.push(("skia_use_vulkan", yes()));
            args.push(("skia_enable_spirv_validation", no()));
        }

        let mut flags: Vec<&str> = vec![];

        if build::ON_WINDOWS {
            // Rust's msvc toolchain supports uses msvcrt.dll by
            // default for release and _debug_ builds.
            flags.push("/MD");
            // Tell Skia's build system where LLVM is supposed to be located.
            // TODO: this should be checked as a prerequisite.
            args.push(("clang_win", quote("C:/Program Files/LLVM")));
        }

        if build::KEEP_INLINE_FUNCTIONS {
            // sadly, this also disables inlining and is probably a real performance bummer.
            if build::ON_WINDOWS {
                flags.push("/Ob0")
            } else {
                flags.push("-fno-inline-functions");
            }
        }

        if !flags.is_empty() {
            let flags: String = {
                let v: Vec<String> = flags.into_iter().map(quote).collect();
                v.join(",")
            };
            args.push(("extra_cflags", format!("[{}]", flags)));
        }

        args
    };

    let gn_args = gn_args
        .into_iter()
        .map(|(name, value)| name.to_owned() + "=" + &value)
        .collect::<Vec<String>>()
        .join(" ");

    let gn_command = if build::ON_WINDOWS {
        "skia/bin/gn"
    } else {
        "bin/gn"
    };

    let output_directory = config.output_directory.to_str().unwrap();

    let output = Command::new(gn_command)
        .args(&["gen", output_directory, &("--args=".to_owned() + &gn_args)])
        .envs(env::vars())
        .current_dir(PathBuf::from("./skia"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("gn error");

    if output.status.code() != Some(0) {
        panic!("{:?}", String::from_utf8(output.stdout).unwrap());
    }

    let ninja_command = if build::ON_WINDOWS {
        "depot_tools/ninja"
    } else {
        "../depot_tools/ninja"
    };

    assert!(
        Command::new(ninja_command)
            .current_dir(PathBuf::from("./skia"))
            .args(&["-C", output_directory])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("failed to run `ninja`, does the directory depot_tools/ exist?")
            .success(),
        "`ninja` returned an error, please check the output for details."
    );

    let current_dir = env::current_dir().unwrap();

    bindgen_gen(&current_dir, output_directory)
}

fn bindgen_gen(current_dir: &Path, output_directory: &str) {
    let mut builder = bindgen::Builder::default()
        .generate_inline_functions(true)
        .default_enum_style(EnumVariation::Rust)
        .constified_enum(".*Mask")
        .constified_enum(".*Flags")
        .constified_enum("SkCanvas_SaveLayerFlagsSet")
        .constified_enum("GrVkAlloc_Flag")
        .constified_enum("GrGLBackendState")
        .whitelist_function("C_.*")
        .whitelist_function("SkColorTypeBytesPerPixel")
        .whitelist_function("SkColorTypeIsAlwaysOpaque")
        .whitelist_function("SkColorTypeValidateAlphaType")
        .whitelist_function("SkRGBToHSV")
        // this function does not whitelist (probably because of inlining):
        .whitelist_function("SkColorToHSV")
        .whitelist_function("SkHSVToColor")
        .whitelist_function("SkPreMultiplyARGB")
        .whitelist_function("SkPreMultiplyColor")
        .whitelist_function("SkBlendMode_Name")
        // functions for which the doc generation fails.
        .blacklist_function("SkColorFilter_asComponentTable")
        // core/
        .whitelist_type("SkAutoCanvasRestore")
        .whitelist_type("SkColorSpacePrimaries")
        .whitelist_type("SkContourMeasure")
        .whitelist_type("SkContourMeasureIter")
        .whitelist_type("SkCubicMap")
        .whitelist_type("SkDocument")
        .whitelist_type("SkDynamicMemoryWStream")
        .whitelist_type("SkPathMeasure")
        .whitelist_type("SkVector4")
        .whitelist_type("SkPictureRecorder")
        .whitelist_type("SkVector4")
        // effects/
        .whitelist_type("SkPath1DPathEffect")
        .whitelist_type("SkLine2DPathEffect")
        .whitelist_type("SkPath2DPathEffect")
        .whitelist_type("SkCornerPathEffect")
        .whitelist_type("SkDashPathEffect")
        .whitelist_type("SkDiscretePathEffect")
        .whitelist_type("SkGradientShader")
        .whitelist_type("SkPerlinNoiseShader")
        .whitelist_type("SkTableColorFilter")
        // gpu/
        .whitelist_type("GrGLBackendState")
        // gpu/vk/
        .whitelist_type("GrVkDrawableInfo")
        .whitelist_type("GrVkExtensionFlags")
        .whitelist_type("GrVkFeatureFlags")
        // misc
        .whitelist_var("SK_Color.*")
        .whitelist_var("kAll_GrBackendState")
        //
        .use_core()
        .clang_arg("-std=c++14");

    let mut cc_build = Build::new();

    // note: we add dependent paths only if we do a full build,
    // otherwise these paths are not even existing.
    let bindings_source = "src/bindings.cpp";
    cargo::add_dependent_path(bindings_source);

    builder = builder.header(bindings_source);

    for include_dir in fs::read_dir("skia/include").expect("Unable to read skia/include") {
        let dir = include_dir.unwrap();
        cargo::add_dependent_path(dir.path().to_str().unwrap());
        let include_path = current_dir.join(dir.path());
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
        cc_build.include(include_path);
    }

    if build::VULKAN {
        cc_build.define("SK_VULKAN", "1");
        builder = builder.clang_arg("-DSK_VULKAN");
        cc_build.define("SKIA_IMPLEMENTATION", "1");
        builder = builder.clang_arg("-DSKIA_IMPLEMENTATION=1");
    }

    if build::SVG {
        cc_build.define("SK_XML", "1");
        builder = builder.clang_arg("-DSK_XML");

        // SkXMLWriter.h
        let include_path = current_dir.join(Path::new("skia/src/xml"));
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
        cc_build.include(include_path);
    }

    if build::SKIA_RELEASE {
        cc_build.define("NDEBUG", "1");
        builder = builder.clang_arg("-DNDEBUG=1")
    }

    cc_build
        .cpp(true)
        .file(bindings_source)
        .out_dir(output_directory);

    if !build::ON_WINDOWS {
        cc_build.flag("-std=c++14");
    }

    cc_build.compile(BINDINGS_LIB_NAME);

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

mod prerequisites {
    use crate::build_support::{cargo, git};
    use crate::build_support::skia::{REPOSITORY_CLONE_URL, REPOSITORY_DIRECTORY};
    use std::fs;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    pub fn require_python() {
        Command::new("python")
            .arg("--version")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect(">>>>> Please install python to build this crate. <<<<<");
    }

    /// Get the skia git repository, either be checkout out the submodule, or
    /// when the build.rs was called outside of the git repository,
    /// by checking out the original repository in a temporary directory and
    /// moving it over.
    pub fn get_skia() {
        match cargo::package_repository_hash() {
            Ok(hash) => {
                // we are in a package.
                resolve_skia_and_depot_tools_from_repo(&hash);
            }
            Err(_) => {
                // we are not in a package, assuming we are in our git repo.
                // so just update all submodules.
                assert!(
                    Command::new("git")
                        .args(&["submodule", "update", "--init", "--depth", "1"])
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .status()
                        .unwrap()
                        .success(),
                    "`git submodule update` failed"
                );
            }
        }
    }

    /// Extracts the submodules skia and depot_tools from the origin
    /// repository we were built with and moves them to the root directory of the crate.
    fn resolve_skia_and_depot_tools_from_repo(hash: &str) {
        let skia_dir = PathBuf::from("skia");
        let depot_tools_dir = PathBuf::from("depot_tools");

        // if these directories already exist, we do nothing here and assume
        // that everyhing is in place for the build.
        if skia_dir.is_dir() && depot_tools_dir.is_dir() {
            return;
        }

        let clone_url = REPOSITORY_CLONE_URL;

        let output_directory = cargo::output_directory();
        let repo_dir = &output_directory.join(REPOSITORY_DIRECTORY);

        if repo_dir.is_dir() {
            fs::remove_dir_all(repo_dir).expect("failed to remove rust-skia directory");
        }

        let exit_status = Command::new("git")
            .args(&["clone", clone_url])
            .current_dir(&output_directory)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("failed start git, is it missing or not in the PATH?");

        if !exit_status.success() {
            panic!("failed to clone repository: {}", clone_url);
        }

        let exit_status = Command::new("git")
            .current_dir(repo_dir)
            .args(&["checkout", hash])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("failed start git, is it missing or not in the PATH?");

        if !exit_status.success() {
            panic!(
                "failed to checkout repository: {}, commit: {}",
                repo_dir.to_str().unwrap(),
                hash
            );
        }

        git::run(&["submodule", "update", "--init", "--depth", "1"], repo_dir.as_path());

        if !exit_status.success() {
            panic!(
                "failed to init and update submodules in {}",
                repo_dir.to_str().unwrap()
            );
        }

        let skia_bindings_dir = repo_dir.join("skia-bindings");

        // now move the submodules over.
        fs::rename(skia_bindings_dir.join("depot_tools"), depot_tools_dir)
            .expect("failed to move depot_tools directory");
        fs::rename(skia_bindings_dir.join("skia"), skia_dir)
            .expect("failed to move skia directory");

        // cleanup
        fs::remove_dir_all(repo_dir).expect("failed to cleanup rust-skia directory");
    }
}
