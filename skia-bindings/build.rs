extern crate bindgen;
extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use bindgen::EnumVariation;
use cc::Build;

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
}

fn main() {

    prerequisites::require_python();

    assert!(Command::new("git")
                .arg("submodule")
                .arg("init")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status().unwrap().success(), "`git submodule init` failed");

    assert!(Command::new("git")
                .args(&["submodule", "update", "--depth", "1"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status().unwrap().success(), "`git submodule update` failed");

    assert!(Command::new("python")
                .arg("skia/tools/git-sync-deps")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status().unwrap().success(), "`skia/tools/git-sync-deps` failed");

    match cargo::target().as_str() {
        (_, "unknown", "linux", Some("gnu")) => {
            cargo::add_link_libs(&["stdc++", "bz2", "GL", "fontconfig", "freetype"]);
        },
        (_, _, "apple", Some("darwin")) => {
            cargo::add_link_libs(&["c++", "framework=OpenGL", "framework=ApplicationServices"]);
        },
        (_, _, "windows", abi) => {
            cargo::add_link_libs(&["usp10", "ole32", "user32", "gdi32", "fontsub", "opengl32"]);
            if abi == Some("gnu") {
                cargo::add_link_lib("stdc++");
            }
        },
        _ => {
            panic!("unsupported target: {:?}", cargo::target())
        }
    };

    let gn_args = {
        fn yes() -> String { "true".into() }
        fn no() -> String { "false".into() }

        fn quote(s: &str) -> String { format!("\"{}\"", s) };

        let mut args: Vec<(&str, String)> = vec![
            ("is_official_build", if build::SKIA_RELEASE { yes() } else { no() }),
            ("skia_use_expat", no()),
            ("skia_use_icu", no()),
            ("skia_use_system_libjpeg_turbo", no()),
            ("skia_use_system_libpng", no()),
            ("skia_use_libwebp", no()),
            ("skia_use_system_zlib", no()),
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
            // sadly, this also disables inlining completely and is probably a real performance bummer.
            if build::ON_WINDOWS {
                flags.push("/Ob0")
            } else {
                flags.push("-fno-inline-functions");
            }
        }

        if flags.len() != 0 {
            let flags: String = {
                let v: Vec<String> = flags.into_iter().map(quote).collect();
                v.join(",")
            };
            args.push(("extra_cflags", format!("[{}]", flags)));
        }

        args
    };

    let gn_args = gn_args.into_iter()
        .map(|(name, value)| name.to_owned() + "=" + &value)
        .collect::<Vec<String>>()
        .join(" ");

    let gn_command =
        if build::ON_WINDOWS {
            "skia/bin/gn"
        } else {
            "bin/gn"
        };

    let skia_out_dir : String =
        PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("skia/Static")
            .to_str().unwrap().into();

    let output = Command::new(gn_command)
        .args(&["gen", &skia_out_dir, &("--args=".to_owned() + &gn_args)])
        .envs(env::vars())
        .current_dir(PathBuf::from("./skia"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("gn error");

    if output.status.code() != Some(0) {
        panic!("{:?}", String::from_utf8(output.stdout).unwrap());
    }

    let ninja_command =
        if build::ON_WINDOWS {
            "depot_tools/ninja"
        } else {
            "../depot_tools/ninja"
        };

    assert!(Command::new(ninja_command)
                .current_dir(PathBuf::from("./skia"))
                .args(&["-C", &skia_out_dir])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("failed to run `ninja`, does the directory depot_tools/ exist?")
                .success(), "`ninja` returned an error, please check the output for details.");

    let current_dir = env::current_dir().unwrap();
    let current_dir_name = current_dir.to_str().unwrap();

    println!("cargo:rustc-link-search={}", &skia_out_dir);
    cargo::add_link_libs(&["static=skia", "static=skiabinding"]);

    bindgen_gen(&current_dir_name, &skia_out_dir)
}

fn bindgen_gen(current_dir_name: &str, skia_out_dir: &str) {

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

        .whitelist_type("SkColorSpacePrimaries")
        .whitelist_type("SkVector4")
        .whitelist_type("SkPictureRecorder")
        .whitelist_type("SkAutoCanvasRestore")

        .whitelist_type("SkPath1DPathEffect")
        .whitelist_type("SkLine2DPathEffect")
        .whitelist_type("SkPath2DPathEffect")
        .whitelist_type("SkCornerPathEffect")
        .whitelist_type("SkDashPathEffect")
        .whitelist_type("SkDiscretePathEffect")
        .whitelist_type("SkGradientShader")
        .whitelist_type("SkPerlinNoiseShader")
        .whitelist_type("SkTableColorFilter")

        .whitelist_type("GrGLBackendState")

        .whitelist_type("GrVkDrawableInfo")
        .whitelist_type("GrVkExtensionFlags")
        .whitelist_type("GrVkFeatureFlags")

        .whitelist_var("SK_Color.*")
        .whitelist_var("kAll_GrBackendState")

        .use_core()
        .clang_arg("-std=c++14");

    let mut cc_build = Build::new();

    let bindings_source = "src/bindings.cpp";
    cargo::add_dependent_path(bindings_source);

    builder = builder.header(bindings_source);

    for include_dir in fs::read_dir("skia/include").expect("Unable to read skia/include") {
        let dir = include_dir.unwrap();
        cargo::add_dependent_path(dir.path().to_str().unwrap());
        let include_path = format!("{}/{}", &current_dir_name, &dir.path().to_str().unwrap());
        builder = builder.clang_arg(format!("-I{}", &include_path));
        cc_build.include(&include_path);
    }

    if build::VULKAN {
        cc_build.define("SK_VULKAN", "1");
        builder = builder.clang_arg("-DSK_VULKAN");
        cc_build.define("SKIA_IMPLEMENTATION", "1");
        builder = builder.clang_arg("-DSKIA_IMPLEMENTATION=1");
    }

    if build::SKIA_RELEASE {
        cc_build.define("NDEBUG", "1");
        builder = builder.clang_arg("-DNDEBUG=1")
    }

    cc_build
        .cpp(true)
        .file(bindings_source)
        .out_dir(skia_out_dir);

    if !build::ON_WINDOWS {
        cc_build.flag("-std=c++14");
    }

    cc_build.compile("skiabinding");

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

mod cargo {
    use std::env;

    pub fn add_dependent_path(path: &str) {
        println!("cargo:rerun-if-changed={}", path);
    }

    pub fn add_link_libs<'a, L: IntoIterator<Item = &'a &'a str>>(libs: L) {
        libs.into_iter().for_each(|s| add_link_lib(*s))
    }

    pub fn add_link_lib(lib: &str) {
        println!("cargo:rustc-link-lib={}", lib);
    }

    #[derive(Clone, Debug)]
    pub struct Target(String, String, String, Option<String>);
    impl Target {
        pub fn as_str(&self) -> (&str, &str, &str, Option<&str>) {
            (self.0.as_str(), self.1.as_str(), self.2.as_str(), self.3.as_ref().map(|s| s.as_str()))
        }
    }

    pub fn target() -> Target {
        let target_str = env::var("TARGET").unwrap();

        let target : Vec<String> =
            target_str
                .split("-")
                .map(|s| s.into())
                .collect();
        if target.len() < 3 {
            panic!("Failed to parse TARGET {}", target_str);
        }

        Target(target[0].clone(), target[1].clone(), target[2].clone(), if target.len() > 3 { Some(target[3].clone()) } else { None })
    }
}

mod prerequisites {
    use std::process::{Command, Stdio};

    pub fn require_python() {
        Command::new("python")
            .arg("--version")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status().expect(">>>>> Please install python to build this crate. <<<<<");
    }
}
