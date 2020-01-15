//! Full build support for the Skia library, SkiaBindings library and bindings.rs file.

use crate::build_support::{android, binaries, cargo, clang, git, ios, llvm, vs};
use cc::Build;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

/// The libraries to link with.
mod lib {
    pub const SKIA: &str = "skia";
    pub const SKIA_BINDINGS: &str = "skia-bindings";
    pub const SKSHAPER: &str = "skshaper";
    pub const SKPARAGRAPH: &str = "skparagraph";
}

/// Feature identifiers define the additional configuration parts of the binaries to download.
mod feature_id {
    pub const VULKAN: &str = "vulkan";
    pub const SVG: &str = "svg";
    pub const SHAPER: &str = "shaper";
    pub const TEXTLAYOUT: &str = "textlayout";
}

/// The defaults for the Skia build configuration.
impl Default for BuildConfiguration {
    fn default() -> Self {
        // m74: if we don't build the particles or the skottie library on macOS, the build fails with
        // for example:
        // [763/867] link libparticles.a
        // FAILED: libparticles.a
        let all_skia_libs = {
            match cargo::target().as_strs() {
                (_, "apple", "darwin", _) => true,
                (_, "apple", "ios", _) => true,
                _ => false,
            }
        };

        let text_layout = {
            match (cfg!(feature = "textlayout"), cfg!(feature = "shaper")) {
                (false, false) => TextLayout::None,
                (false, true) => TextLayout::ShaperOnly,
                (true, false) => panic!("invalid feature configuration, feature 'shaper' must be enabled for feature 'textlayout'"),
                (true, true) => TextLayout::ShaperAndParagraph,
            }
        };

        let skia_debug = {
            match cargo::env_var("SKIA_DEBUG") {
                Some(v) if v != "0" => true,
                _ => false,
            }
        };

        BuildConfiguration {
            on_windows: cargo::host().is_windows(),
            skia_debug,
            keep_inline_functions: true,
            features: Features {
                vulkan: cfg!(feature = "vulkan"),
                svg: cfg!(feature = "svg"),
                text_layout,
                animation: false,
                dng: false,
                particles: false,
            },
            all_skia_libs,
            definitions: Vec::new(),
        }
    }
}

/// The build configuration for Skia.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BuildConfiguration {
    /// Do we build _on_ a Windows OS?
    on_windows: bool,

    /// Build Skia in a debug configuration?
    skia_debug: bool,

    /// Configure Skia builds to keep inline functions to
    /// prevent linker errors.
    keep_inline_functions: bool,

    /// The Skia feature set to compile.
    features: Features,

    /// As of M74, There is a bug in the Skia macOS build
    /// that requires all libraries to be built, otherwise the build will fail.
    all_skia_libs: bool,

    /// Additional preprocessor definitions that will override predefined ones.
    definitions: Definitions,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Features {
    /// Build with Vulkan support?
    vulkan: bool,

    /// Build with SVG support?
    svg: bool,

    /// Features related to text layout.
    text_layout: TextLayout,

    /// Build with animation support (yet unsupported, no wrappers).
    animation: bool,

    /// Support DNG file format (currently unsupported because of build errors).
    dng: bool,

    /// Build the particles module (unsupported, no wrappers).
    particles: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TextLayout {
    /// No text shaping or layout features.
    None,
    /// Builds the skshaper module, compiles harfbuzz & icu support.
    ShaperOnly,
    /// Builds the skshaper and the skparagraph module.
    ShaperAndParagraph,
}

impl TextLayout {
    fn skia_args(&self) -> Vec<(&'static str, String)> {
        let mut args = Vec::new();
        let (shaper, paragraph) = match self {
            TextLayout::None => {
                args.push(("skia_use_icu", no()));
                (false, false)
            }
            TextLayout::ShaperOnly => (true, false),
            TextLayout::ShaperAndParagraph => (true, true),
        };

        if shaper {
            args.extend(vec![
                ("skia_enable_skshaper", yes()),
                ("skia_use_icu", yes()),
                ("skia_use_system_icu", no()),
                ("skia_use_harfbuzz", yes()),
                ("skia_pdf_subset_harfbuzz", yes()),
                ("skia_use_system_harfbuzz", no()),
                ("skia_use_sfntly", no()),
            ]);
        }

        if paragraph {
            args.extend(vec![
                ("skia_enable_skparagraph", yes()),
                // note: currently, tests need to be enabled, because modules/skparagraph
                // is not included in the default dependency configuration.
                // ("paragraph_tests_enabled", no()),
            ]);
        }

        args
    }

    fn sources(&self) -> Vec<PathBuf> {
        match self {
            TextLayout::None => Vec::new(),
            TextLayout::ShaperOnly => vec!["src/shaper.cpp".into()],
            TextLayout::ShaperAndParagraph => {
                vec!["src/shaper.cpp".into(), "src/paragraph.cpp".into()]
            }
        }
    }

    fn patches(&self) -> Vec<Patch> {
        match self {
            TextLayout::ShaperAndParagraph => vec![Patch {
                name: "skparagraph".into(),
                marked_file: "BUILD.gn".into(),
            }],
            _ => Vec::new(),
        }
    }

    fn ninja_files(&self) -> Vec<PathBuf> {
        match self {
            TextLayout::None => Vec::new(),
            TextLayout::ShaperOnly => vec!["obj/modules/skshaper/skshaper.ninja".into()],
            TextLayout::ShaperAndParagraph => vec![
                "obj/modules/skshaper/skshaper.ninja".into(),
                "obj/modules/skparagraph/skparagraph.ninja".into(),
            ],
        }
    }
}

/// This is the final, low level build configuration.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinalBuildConfiguration {
    /// Patches to be applied to the Skia repository.
    pub skia_patches: Vec<Patch>,

    /// The name value pairs passed as arguments to gn.
    pub gn_args: Vec<(String, String)>,

    /// ninja files that need to be parsed for further definitions.
    pub ninja_files: Vec<PathBuf>,

    /// The additional definitions (cloned from the definitions of
    /// the BuildConfiguration).
    pub definitions: Definitions,

    /// The binding source files to compile.
    pub binding_sources: Vec<PathBuf>,
}

impl FinalBuildConfiguration {
    pub fn from_build_configuration(build: &BuildConfiguration) -> FinalBuildConfiguration {
        let features = &build.features;

        let gn_args = {
            fn quote(s: &str) -> String {
                format!("\"{}\"", s)
            }

            let mut args: Vec<(&str, String)> = vec![
                (
                    "is_official_build",
                    if build.skia_debug { no() } else { yes() },
                ),
                ("is_debug", if build.skia_debug { yes() } else { no() }),
                ("skia_use_system_libjpeg_turbo", no()),
                ("skia_use_system_libpng", no()),
                ("skia_use_libwebp", no()),
                ("skia_use_system_zlib", no()),
                (
                    "skia_enable_skottie",
                    if features.animation || build.all_skia_libs {
                        yes()
                    } else {
                        no()
                    },
                ),
                ("skia_use_xps", no()),
                ("skia_use_dng_sdk", if features.dng { yes() } else { no() }),
                (
                    "skia_enable_particles",
                    if features.particles || build.all_skia_libs {
                        yes()
                    } else {
                        no()
                    },
                ),
                ("cc", quote("clang")),
                ("cxx", quote("clang++")),
            ];

            // further flags that limit the components of Skia debug builds.
            if build.skia_debug {
                args.push(("skia_enable_atlas_text", no()));
                args.push(("skia_enable_spirv_validation", no()));
                args.push(("skia_enable_tools", no()));
                args.push(("skia_enable_vulkan_debug_layers", no()));
                args.push(("skia_use_libheif", no()));
                args.push(("skia_use_lua", no()));
            }

            args.extend(features.text_layout.skia_args());

            if features.vulkan {
                args.push(("skia_use_vulkan", yes()));
                args.push(("skia_enable_spirv_validation", no()));
            }

            let mut flags: Vec<&str> = vec![];
            let mut use_expat = features.svg;

            // target specific gn args.
            let target = cargo::target();
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
                        flags.push("/MT");
                    }
                    // otherwise the C runtime should be linked dynamically
                    else {
                        flags.push("/MD");
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
                (arch, "linux", "android", _) => {
                    args.push(("ndk", quote(&android::ndk())));
                    // TODO: make API-level configurable?
                    args.push(("ndk_api", android::API_LEVEL.into()));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                    args.push(("skia_use_system_freetype2", no()));
                    args.push(("skia_enable_fontmgr_android", yes()));
                    // Enabling fontmgr_android implicitly enables expat.
                    // We make this explicit to avoid relying on an expat installed
                    // in the system.
                    use_expat = true;
                }
                (arch, "apple", "ios", _) => {
                    args.push(("target_os", quote("ios")));
                    args.push(("target_cpu", quote(clang::target_arch(arch))));
                }
                _ => {}
            }

            if use_expat {
                args.push(("skia_use_expat", yes()));
                args.push(("skia_use_system_expat", no()));
            } else {
                args.push(("skia_use_expat", no()));
            }

            if build.all_skia_libs {
                // m78: modules/particles forgets to set SKIA_IMPLEMENTATION=1 and so
                // expects system vulkan headers.
                flags.push("-DSKIA_IMPLEMENTATION=1");
            }

            if !flags.is_empty() {
                let flags: String = {
                    let v: Vec<String> = flags.into_iter().map(quote).collect();
                    v.join(",")
                };
                args.push(("extra_cflags", format!("[{}]", flags)));
            }

            args.into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect()
        };

        let ninja_files = {
            let mut files = Vec::new();
            files.push("obj/skia.ninja".into());
            files.extend(features.text_layout.ninja_files());
            files
        };

        let binding_sources = {
            let mut sources: Vec<PathBuf> = Vec::new();
            sources.push("src/bindings.cpp".into());
            sources.extend(features.text_layout.sources());
            if features.vulkan {
                sources.push("src/vulkan.cpp".into());
            }
            if features.svg {
                sources.push("src/svg.cpp".into())
            }
            sources
        };

        FinalBuildConfiguration {
            skia_patches: features.text_layout.patches(),
            gn_args,
            ninja_files,
            definitions: build.definitions.clone(),
            binding_sources,
        }
    }
}

fn yes() -> String {
    "true".into()
}
fn no() -> String {
    "false".into()
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
    pub built_libraries: Vec<String>,

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

        let mut built_libraries = Vec::new();
        let mut additional_files = Vec::new();
        let mut feature_ids = Vec::new();

        if features.vulkan {
            feature_ids.push(feature_id::VULKAN);
        }
        if features.svg {
            feature_ids.push(feature_id::SVG);
        }
        match features.text_layout {
            TextLayout::None => {}
            TextLayout::ShaperOnly => {
                feature_ids.push(feature_id::SHAPER);
                additional_files.push(ICUDTL_DAT.into());
                built_libraries.push(lib::SKSHAPER.into());
            }
            TextLayout::ShaperAndParagraph => {
                feature_ids.push(feature_id::TEXTLAYOUT);
                additional_files.push(ICUDTL_DAT.into());
                built_libraries.push(lib::SKPARAGRAPH.into());
                built_libraries.push(lib::SKSHAPER.into());
            }
        }

        let mut link_libraries = Vec::new();

        match target.as_strs() {
            (_, "unknown", "linux", Some("gnu")) => {
                link_libraries.extend(vec!["stdc++", "GL", "fontconfig", "freetype"]);
            }
            (_, "apple", "darwin", _) => {
                link_libraries.extend(vec![
                    "c++",
                    "framework=OpenGL",
                    "framework=ApplicationServices",
                ]);
            }
            (_, _, "windows", Some("msvc")) => {
                link_libraries.extend(vec![
                    "usp10", "ole32", "user32", "gdi32", "fontsub", "opengl32",
                ]);
            }
            (_, "linux", "android", _) => {
                link_libraries.extend(vec![
                    "log",
                    "android",
                    "EGL",
                    "GLESv2",
                    "c++_static",
                    "c++abi",
                ]);
            }
            (_, "apple", "ios", _) => {
                link_libraries.extend(vec![
                    "c++",
                    "framework=MobileCoreServices",
                    "framework=CoreFoundation",
                    "framework=CoreGraphics",
                    "framework=CoreText",
                    "framework=ImageIO",
                    "framework=UIKit",
                ]);
            }
            _ => panic!("unsupported target: {:?}", cargo::target()),
        };

        let output_directory = cargo::output_directory()
            .join(SKIA_OUTPUT_DIR)
            .to_str()
            .unwrap()
            .into();

        built_libraries.push(lib::SKIA.into());
        built_libraries.push(lib::SKIA_BINDINGS.into());

        BinariesConfiguration {
            feature_ids: feature_ids.into_iter().map(|f| f.to_string()).collect(),
            output_directory,
            link_libraries: link_libraries
                .into_iter()
                .map(|lib| lib.to_string())
                .collect(),
            built_libraries,
            additional_files,
            skia_debug: build.skia_debug,
        }
    }

    /// Inform cargo that the library files of the given configuration are available and
    /// can be used as dependencies.
    pub fn commit_to_cargo(&self) {
        println!(
            "cargo:rustc-link-search={}",
            self.output_directory.to_str().unwrap()
        );

        // On Linux, the order is significant, first the static libraries we built, and then
        // the system libraries.

        for lib in &self.built_libraries {
            cargo::add_link_lib(format!("static={}", lib));
        }

        cargo::add_link_libs(&self.link_libraries);
    }

    pub fn key(&self, repository_short_hash: &str) -> String {
        binaries::key(repository_short_hash, &self.feature_ids, self.skia_debug)
    }
}

/// The full build of Skia, SkiaBindings, and the generation of bindings.rs.
pub fn build(build: &FinalBuildConfiguration, config: &BinariesConfiguration) {
    prerequisites::resolve_dependencies();

    // call Skia's git-sync-deps

    let python2 = &prerequisites::locate_python2_cmd();
    println!("Python 2 found: {:?}", python2);

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

    // apply patches

    let patch_root = &PathBuf::from("skia");

    // if there is any patch to be applied, be sure there is a git repository in the skia
    // subdirectory, because otherwise git apply will silently fail.

    if !build.skia_patches.is_empty() {
        git::run(&["init"], Some(patch_root.as_path()));
    }

    for patch in &build.skia_patches {
        println!("applying patch: {}", patch.name);
        patch.apply(patch_root);
    }

    // configure Skia

    let gn_args = build
        .gn_args
        .iter()
        .map(|(name, value)| name.clone() + "=" + value)
        .collect::<Vec<String>>()
        .join(" ");

    let current_dir = env::current_dir().unwrap();
    let gn_command = current_dir.join("skia").join("bin").join("gn");

    let output_directory_str = config.output_directory.to_str().unwrap();

    println!("Skia args: {}", &gn_args);

    let output = Command::new(gn_command)
        .args(&[
            "gen",
            output_directory_str,
            &format!("--script-executable={}", python2),
            &format!("--args={}", gn_args),
        ])
        .envs(env::vars())
        .current_dir(PathBuf::from("./skia"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("gn error");

    if output.status.code() != Some(0) {
        panic!("{:?}", String::from_utf8(output.stdout).unwrap());
    }

    // build Skia

    let on_windows = cfg!(windows);

    let ninja_command =
        current_dir
            .join("depot_tools")
            .join(if on_windows { "ninja.exe" } else { "ninja" });

    let ninja_status = Command::new(ninja_command)
        .current_dir(PathBuf::from("./skia"))
        .args(&["-C", output_directory_str])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    // reverse patches
    //
    // Even though we patch only the gn configuration, we wait until the ninja build went through,
    // because ninja may regenerate its files by calling into gn again (happened once on the CI).

    for patch in &build.skia_patches {
        println!("reversing patch: {}", patch.name);
        patch.reverse(patch_root);
    }

    assert!(
        ninja_status
            .expect("failed to run `ninja`, does the directory depot_tools/ exist?")
            .success(),
        "`ninja` returned an error, please check the output for details."
    );

    bindgen_gen(build, &current_dir, &config.output_directory)
}

fn bindgen_gen(build: &FinalBuildConfiguration, current_dir: &Path, output_directory: &Path) {
    let mut builder = bindgen::Builder::default()
        .generate_comments(false)
        .layout_tests(true)
        // on macOS some arrays that are used in opaque types get too large to support Debug.
        // (for example High Sierra: [u16; 105])
        // TODO: may reenable when const generics land in stable.
        .derive_debug(false)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(ParseCallbacks))
        .raw_line("#![allow(clippy::all)]")
        // GrVkBackendContext contains u128 fields on macOS
        .raw_line("#![allow(improper_ctypes)]")
        .parse_callbacks(Box::new(ParseCallbacks))
        .whitelist_function("C_.*")
        .constified_enum(".*Mask")
        .constified_enum(".*Flags")
        .constified_enum(".*Bits")
        .constified_enum("SkCanvas_SaveLayerFlagsSet")
        .constified_enum("GrVkAlloc_Flag")
        .constified_enum("GrGLBackendState")
        // modules/skparagraph
        //   pulls in a std::map<>, which we treat as opaque, but bindgen creates wrong bindings for
        //   std::_Tree* types
        .blacklist_type("std::_Tree.*")
        .blacklist_type("std::map.*")
        //   debug builds:
        .blacklist_type("SkLRUCache")
        .blacklist_type("SkLRUCache_Entry")
        //   not used at all:
        .blacklist_type("std::vector.*")
        // Vulkan reexports that got swallowed by making them opaque.
        // (these can not be whitelisted by a extern "C" function)
        .whitelist_type("VkPhysicalDeviceFeatures")
        .whitelist_type("VkPhysicalDeviceFeatures2")
        // misc
        .whitelist_var("SK_Color.*")
        .whitelist_var("kAll_GrBackendState")
        //
        .use_core()
        .clang_arg("-std=c++14")
        // required for macOS LLVM 8 to pick up C++ headers:
        .clang_args(&["-x", "c++"])
        .clang_arg("-v");

    for function in WHITELISTED_FUNCTIONS {
        builder = builder.whitelist_function(function)
    }

    for opaque_type in OPAQUE_TYPES {
        builder = builder.opaque_type(opaque_type)
    }

    let mut cc_build = Build::new();

    for source in &build.binding_sources {
        cc_build.file(source);
        let source = source.to_str().unwrap();
        cargo::rerun_if_changed(source);
        builder = builder.header(source);
    }

    // TODO: may put the include paths into the FinalBuildConfiguration?

    let include_path = current_dir.join("skia");
    cargo::rerun_if_changed(include_path.join("include"));

    builder = builder.clang_arg(format!("-I{}", include_path.display()));
    cc_build.include(include_path);

    let definitions = {
        let mut definitions = Vec::new();

        for ninja_file in &build.ninja_files {
            let ninja_file = output_directory.join(ninja_file);
            let contents = fs::read_to_string(ninja_file).unwrap();
            definitions = definitions::combine(definitions, definitions::from_ninja(contents))
        }

        definitions::combine(definitions, build.definitions.clone())
    };

    for (name, value) in &definitions {
        match value {
            Some(value) => {
                cc_build.define(name, value.as_str());
                builder = builder.clang_arg(format!("-D{}={}", name, value));
            }
            None => {
                cc_build.define(name, "");
                builder = builder.clang_arg(format!("-D{}", name));
            }
        }
    }

    cc_build.cpp(true).out_dir(output_directory);

    if !cfg!(windows) {
        cc_build.flag("-std=c++14");
    }

    let target = cargo::target();
    match target.as_strs() {
        (arch, "linux", "android", _) => {
            let target = &target.to_string();
            cc_build.target(target);
            for arg in android::additional_clang_args(target, arch) {
                builder = builder.clang_arg(arg);
            }
        }
        (arch, "apple", "ios", _) => {
            for arg in ios::additional_clang_args(arch) {
                builder = builder.clang_arg(arg);
            }
        }
        _ => {}
    }

    println!("COMPILING BINDINGS: {:?}", build.binding_sources);
    cc_build.compile(lib::SKIA_BINDINGS);

    println!("GENERATING BINDINGS");
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

const WHITELISTED_FUNCTIONS: &[&str] = &[
    "SkAnnotateRectWithURL",
    "SkAnnotateNamedDestination",
    "SkAnnotateLinkToDestination",
    "SkColorTypeBytesPerPixel",
    "SkColorTypeIsAlwaysOpaque",
    "SkColorTypeValidateAlphaType",
    "SkRGBToHSV",
    // this function does not whitelist (probably because of inlining):
    "SkColorToHSV",
    "SkHSVToColor",
    "SkPreMultiplyARGB",
    "SkPreMultiplyColor",
    "SkBlendMode_AsCoeff",
    "SkBlendMode_Name",
    "SkSwapRB",
    // functions for which the doc generation fails
    "SkColorFilter_asComponentTable",
    // pathops/
    "Op",
    "Simplify",
    "TightBounds",
    "AsWinding",
    // utils/
    "Sk3LookAt",
    "Sk3Perspective",
    "Sk3MapPts",
    "SkUnitCubicInterp",
];

const OPAQUE_TYPES: &[&str] = &[
    // Types for which the binding generator pulls in stuff that can not be compiled.
    "SkDeferredDisplayList",
    "SkDeferredDisplayList_PendingPathsMap",
    // Types for which a bindgen layout is wrong causing types that contain
    // fields of them to fail their layout test.

    // Windows:
    "std::atomic",
    "std::function",
    "std::unique_ptr",
    "SkAutoTMalloc",
    "SkTHashMap",
    // Ubuntu 18 LLVM 6: all types derived from SkWeakRefCnt
    "SkWeakRefCnt",
    "GrContext",
    "GrContextThreadSafeProxy",
    "GrContext_Base",
    "GrGLInterface",
    "GrImageContext",
    "GrRecordingContext",
    "GrSurfaceProxy",
    "Sk2DPathEffect",
    "SkCornerPathEffect",
    "SkDataTable",
    "SkDiscretePathEffect",
    "SkDrawable",
    "SkLine2DPathEffect",
    "SkPath2DPathEffect",
    "SkPathRef_GenIDChangeListener",
    "SkPicture",
    "SkPixelRef",
    "SkSurface",
    // Types not needed (for now):
    "SkDeque",
    "SkDeque_Iter",
    "GrGLInterface_Functions",
    // SkShaper (m77) Trivial*Iterator classes create two vtable pointers.
    "SkShaper_TrivialBiDiRunIterator",
    "SkShaper_TrivialFontRunIterator",
    "SkShaper_TrivialLanguageRunIterator",
    "SkShaper_TrivialScriptRunIterator",
    // skparagraph
    "std::vector",
    "std::u16string",
    // skparagraph (m78), (layout fails on macOS and Linux, not sure why, looks like an obscure alignment problem)
    "skia::textlayout::FontCollection",
    // skparagraph (m79), std::map is used in LineMetrics
    "std::map",
    // Vulkan reexports with the wrong field naming conventions.
    "VkPhysicalDeviceFeatures",
    "VkPhysicalDeviceFeatures2",
    // Since Rust 1.39 beta (TODO: investigate why, and re-test when 1.39 goes stable).
    "GrContextOptions_PersistentCache",
    "GrContextOptions_ShaderErrorHandler",
    "Sk1DPathEffect",
    "SkBBHFactory",
    "SkBitmap_Allocator",
    "SkBitmap_HeapAllocator",
    "SkColorFilter",
    "SkDeque_F2BIter",
    "SkDrawLooper",
    "SkDrawLooper_Context",
    "SkDrawable_GpuDrawHandler",
    "SkFlattenable",
    "SkFontMgr",
    "SkFontStyleSet",
    "SkMaskFilter",
    "SkPathEffect",
    "SkPicture_AbortCallback",
    "SkPixelRef_GenIDChangeListener",
    "SkRasterHandleAllocator",
    "SkRefCnt",
    "SkShader",
    "SkStream",
    "SkStreamAsset",
    "SkStreamMemory",
    "SkStreamRewindable",
    "SkStreamSeekable",
    "SkTypeface_LocalizedStrings",
    "SkWStream",
    "GrVkMemoryAllocator",
    "SkShaper",
    "SkShaper_BiDiRunIterator",
    "SkShaper_FontRunIterator",
    "SkShaper_LanguageRunIterator",
    "SkShaper_RunHandler",
    "SkShaper_RunIterator",
    "SkShaper_ScriptRunIterator",
    "SkContourMeasure",
    "SkDocument",
];

#[derive(Debug)]
struct ParseCallbacks;

impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
    /// Allows to rename an enum variant, replacing `_original_variant_name`.
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        enum_name.and_then(|enum_name| {
            ENUM_TABLE
                .iter()
                .find(|n| n.0 == enum_name)
                .map(|(_, replacer)| replacer(enum_name, original_variant_name))
        })
    }
}

type EnumEntry = (&'static str, fn(&str, &str) -> String);

const ENUM_TABLE: &[EnumEntry] = &[
    ("SkBlendMode", replace::k_xxx),
    ("SkBlendModeCoeff", replace::k_xxx),
    // DartTypes.h
    ("Affinity", replace::k_xxx),
    ("RectHeightStyle", replace::k_xxx),
    ("RectWidthStyle", replace::k_xxx),
    ("TextAlign", replace::k_xxx),
    ("TextDirection", replace::k_xxx_uppercase),
    ("TextBaseline", replace::k_xxx),
    // TextStyle.h
    ("TextDecorationStyle", replace::k_xxx),
    ("StyleType", replace::k_xxx),
    ("PlaceholderAlignment", replace::k_xxx),
    // Vk*
    ("VkChromaLocation", replace::vk),
    ("VkFilter", replace::vk),
    ("VkFormat", replace::vk),
    ("VkImageLayout", replace::vk),
    ("VkImageTiling", replace::vk),
    ("VkSamplerYcbcrModelConversion", replace::vk),
    ("VkSamplerYcbcrRange", replace::vk),
    ("VkStructureType", replace::vk),
];

pub(crate) mod replace {
    use heck::ShoutySnakeCase;
    use regex::Regex;

    pub fn k_xxx_uppercase(name: &str, variant: &str) -> String {
        k_xxx(name, variant).to_uppercase()
    }

    pub fn k_xxx(name: &str, variant: &str) -> String {
        if variant.starts_with('k') {
            variant[1..].into()
        } else {
            panic!(
                "Variant name '{}' of enum type '{}' is expected to start with a 'k'",
                variant, name
            );
        }
    }

    pub fn _k_xxx_enum(name: &str, variant: &str) -> String {
        capture(variant, &format!("k(.*)_{}", name))
    }

    pub fn vk(name: &str, variant: &str) -> String {
        let prefix = name.to_shouty_snake_case();
        capture(variant, &format!("{}_(.*)", prefix))
    }

    fn capture(variant: &str, pattern: &str) -> String {
        let re = Regex::new(pattern).unwrap();
        re.captures(variant).unwrap()[1].into()
    }
}

mod prerequisites {
    use crate::build_support::{cargo, utils};
    use flate2::read::GzDecoder;
    use std::ffi::OsStr;
    use std::fs;
    use std::io::Cursor;
    use std::path::Component;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    pub fn locate_python2_cmd() -> &'static str {
        const PYTHON_CMDS: [&str; 2] = ["python", "python2"];
        for python in PYTHON_CMDS.as_ref() {
            println!("Probing '{}'", python);
            if let Some(true) = is_python_version_2(python) {
                return python;
            }
        }

        panic!(">>>>> Probing for Python 2 failed, please make sure that it's available in PATH, probed executables are: {:?} <<<<<", PYTHON_CMDS);
    }

    /// Returns true if the given python executable is python version 2.
    /// or None if the executable was not found.
    pub fn is_python_version_2(exe: impl AsRef<str>) -> Option<bool> {
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

    /// Resolve the skia and depot_tools subdirectory contents, either by checking out the
    /// submodules, or when the build.rs was called outside of the git repository,
    /// by downloading and unpacking them from GitHub.
    pub fn resolve_dependencies() {
        if cargo::is_crate() {
            // we are in a crate.
            download_dependencies();
        } else {
            // we are not in a crate, assuming we are in our git repo.
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

    /// Downloads the skia and depot_tools from their repositories.
    ///
    /// The hashes are taken from the Cargo.toml section [package.metadata].
    fn download_dependencies() {
        let metadata = cargo::get_metadata();

        for dep in dependencies() {
            let repo_url = dep.url;
            let repo_name = dep.repo;

            let dir = PathBuf::from(repo_name);

            // directory exists => assume that the download of the archive was successful.
            if dir.exists() {
                continue;
            }

            // hash available?
            let (_, short_hash) = metadata
                .iter()
                .find(|(n, _)| n == repo_name)
                .expect("metadata entry not found");

            // remove unpacking directory, github will format it to repo_name-hash
            let unpack_dir = &PathBuf::from(format!("{}-{}", repo_name, short_hash));
            if unpack_dir.is_dir() {
                fs::remove_dir_all(unpack_dir).unwrap();
            }

            // download
            let archive_url = &format!("{}/{}", repo_url, short_hash);
            println!("DOWNLOADING: {}", archive_url);
            let archive = utils::download(archive_url)
                .unwrap_or_else(|_| panic!("Failed to download {}", archive_url));

            // unpack
            {
                let tar = GzDecoder::new(Cursor::new(archive));
                let mut archive = tar::Archive::new(tar);
                let dir = std::env::current_dir().unwrap();
                for entry in archive.entries().expect("failed to iterate over archive") {
                    let mut entry = entry.unwrap();
                    let path = entry.path().unwrap();
                    let mut components = path.components();
                    let root = components.next().unwrap();
                    // skip pax headers.
                    if root.as_os_str() == unpack_dir.as_os_str()
                        && (dep.path_filter)(components.as_path())
                    {
                        entry.unpack_in(&dir).unwrap();
                    }
                }
            }

            // move unpack directory to the target repository directory
            fs::rename(unpack_dir, repo_name).expect("failed to move directory");
        }
    }

    // Specifies where to download Skia and Depot Tools archives from.
    //
    // We use codeload.github.com, otherwise the short hash will be expanded to a full hash as the root
    // directory inside the tar.gz, and we run into filesystem path length restrictions
    // with Skia.
    struct Dependency {
        pub repo: &'static str,
        pub url: &'static str,
        pub path_filter: fn(&Path) -> bool,
    }

    fn dependencies() -> &'static [Dependency] {
        return &[
            Dependency {
                repo: "skia",
                url: "https://codeload.github.com/google/skia/tar.gz",
                path_filter: filter_skia,
            },
            Dependency {
                repo: "depot_tools",
                url: "https://codeload.github.com/rust-skia/depot_tools/tar.gz",
                path_filter: filter_depot_tools,
            },
        ];

        // infra/ contains very long filenames which may hit the max path restriction on Windows.
        // https://github.com/rust-skia/rust-skia/issues/169
        fn filter_skia(p: &Path) -> bool {
            match p.components().next() {
                Some(Component::Normal(name)) if name == OsStr::new("infra") => false,
                _ => true,
            }
        }

        // we need only ninja from depot_tools.
        // https://github.com/rust-skia/rust-skia/pull/165
        fn filter_depot_tools(p: &Path) -> bool {
            p.to_str().unwrap().starts_with("ninja")
        }
    }
}

use bindgen::EnumVariation;
pub use definitions::{Definition, Definitions};

pub(crate) mod definitions {
    use std::collections::HashSet;

    /// A preprocessor definition.
    pub type Definition = (String, Option<String>);
    /// A container for a number of preprocessor definitions.
    pub type Definitions = Vec<Definition>;

    /// Parse a defines = line from a ninja build file.
    pub fn from_ninja(ninja_file: impl AsRef<str>) -> Definitions {
        let lines: Vec<&str> = ninja_file.as_ref().lines().collect();
        let defines = {
            let prefix = "defines = ";
            let defines = lines
                .into_iter()
                .find(|s| s.starts_with(prefix))
                .expect("missing a line with the prefix 'defines =' in a .ninja file");
            &defines[prefix.len()..]
        };
        let defines: Vec<&str> = {
            let prefix = "-D";
            defines
                .split_whitespace()
                .map(|d| {
                    if d.starts_with(prefix) {
                        &d[prefix.len()..]
                    } else {
                        panic!("missing '-D' prefix from a definition")
                    }
                })
                .collect()
        };
        defines
            .into_iter()
            .map(|d| {
                let items: Vec<&str> = d.splitn(2, '=').collect();
                match items.len() {
                    1 => (items[0].to_string(), None),
                    2 => (items[0].to_string(), Some(items[1].to_string())),
                    _ => panic!("internal error"),
                }
            })
            .collect()
    }

    pub fn combine(a: Definitions, b: Definitions) -> Definitions {
        remove_duplicates(a.into_iter().chain(b.into_iter()).collect())
    }

    pub fn remove_duplicates(mut definitions: Definitions) -> Definitions {
        let mut uniques = HashSet::new();
        definitions.retain(|e| uniques.insert(e.0.clone()));
        definitions
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Patch {
    name: String,
    marked_file: PathBuf,
}

impl Patch {
    fn apply(&self, root_dir: &Path) {
        if !self.is_applied(root_dir) {
            let patch_file = PathBuf::from("..").join(self.name.clone() + ".patch");
            git::run(&["apply", patch_file.to_str().unwrap()], Some(root_dir));
        }
    }

    fn reverse(&self, root_dir: &Path) {
        if self.is_applied(root_dir) {
            let patch_file = PathBuf::from("..").join(self.name.clone() + ".patch");
            git::run(
                &["apply", "--reverse", patch_file.to_str().unwrap()],
                Some(root_dir),
            );
        }
    }

    fn is_applied(&self, root_dir: &Path) -> bool {
        let build_gn = root_dir.join(&self.marked_file);
        let contents = fs::read_to_string(build_gn).unwrap();
        let patch_marker = format!(
            "**SKIA-BINDINGS-PATCH-MARKER-{}**",
            self.name.to_uppercase()
        );
        contents.contains(&patch_marker)
    }
}
