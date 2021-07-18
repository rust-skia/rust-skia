//! Full build support for the SkiaBindings library, and bindings.rs file.

use crate::build_support::{android, binaries_config, cargo, features, ios, xcode};
use bindgen::{CodegenConfig, EnumVariation, RustTarget};
use cc::Build;
use std::path::{Path, PathBuf};

pub mod env {
    use crate::build_support::cargo;

    pub fn skia_lib_definitions() -> Option<String> {
        cargo::env_var("SKIA_BUILD_DEFINES")
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinalBuildConfiguration {
    /// The binding source files to compile.
    pub binding_sources: Vec<PathBuf>,

    /// The Skia source directory.
    pub skia_source_dir: PathBuf,

    /// Further definitions needed for build consistency.
    pub definitions: Definitions,
}

impl FinalBuildConfiguration {
    pub fn from_build_configuration(
        features: &features::Features,
        definitions: Definitions,
        skia_source_dir: &Path,
    ) -> FinalBuildConfiguration {
        let binding_sources = {
            let mut sources: Vec<PathBuf> = vec!["src/bindings.cpp".into()];
            if features.gl {
                sources.push("src/gl.cpp".into());
            }
            if features.vulkan {
                sources.push("src/vulkan.cpp".into());
            }
            if features.metal {
                sources.push("src/metal.cpp".into());
            }
            if features.d3d {
                sources.push("src/d3d.cpp".into());
            }
            if features.gpu() {
                sources.push("src/gpu.cpp".into());
            }
            if features.text_layout {
                sources.extend(vec!["src/shaper.cpp".into(), "src/paragraph.cpp".into()]);
            }
            sources.push("src/svg.cpp".into());
            sources
        };

        FinalBuildConfiguration {
            skia_source_dir: skia_source_dir.into(),
            binding_sources,
            definitions,
        }
    }
}

pub fn generate_bindings(build: &FinalBuildConfiguration, output_directory: &Path) {
    let mut builder = bindgen::Builder::default()
        .generate_comments(false)
        .layout_tests(true)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(ParseCallbacks))
        .raw_line("#![allow(clippy::all)]")
        // https://github.com/rust-lang/rust-bindgen/issues/1651
        .raw_line("#![allow(unknown_lints)]")
        .raw_line("#![allow(deref_nullptr)]")
        // GrVkBackendContext contains u128 fields on macOS
        .raw_line("#![allow(improper_ctypes)]")
        .allowlist_function("C_.*")
        .constified_enum(".*Mask")
        .constified_enum(".*Flags")
        .constified_enum(".*Bits")
        .constified_enum("SkCanvas_SaveLayerFlagsSet")
        .constified_enum("GrVkAlloc_Flag")
        .constified_enum("GrGLBackendState")
        // not used:
        .blocklist_type("SkPathRef_Editor")
        .blocklist_function("SkPathRef_Editor_Editor")
        // private types that pull in inline functions that cannot be linked:
        // https://github.com/rust-skia/rust-skia/issues/318
        .raw_line("pub enum GrContext_Base {}")
        .blocklist_type("GrContext_Base")
        .blocklist_function("GrContext_Base_.*")
        .raw_line("pub enum GrImageContext {}")
        .blocklist_type("GrImageContext")
        .raw_line("pub enum GrImageContextPriv {}")
        .blocklist_type("GrImageContextPriv")
        .raw_line("pub enum GrContextThreadSafeProxy {}")
        .blocklist_type("GrContextThreadSafeProxy")
        .blocklist_type("GrContextThreadSafeProxyPriv")
        .raw_line("pub enum GrContextThreadSafeProxyPriv {}")
        .blocklist_type("GrRecordingContextPriv")
        .raw_line("pub enum GrRecordingContextPriv {}")
        .blocklist_function("GrRecordingContext_priv.*")
        .blocklist_function("GrDirectContext_priv.*")
        .blocklist_type("GrContextPriv")
        .raw_line("pub enum GrContextPriv {}")
        .blocklist_function("GrContext_priv.*")
        .blocklist_function("SkDeferredDisplayList_priv.*")
        .raw_line("pub enum SkVerticesPriv {}")
        .blocklist_type("SkVerticesPriv")
        .blocklist_function("SkVertices_priv.*")
        .blocklist_function("std::bitset_flip.*")
        // Vulkan reexports that got swallowed by making them opaque.
        // (these can not be allowlisted by a extern "C" function)
        .allowlist_type("VkPhysicalDeviceFeatures")
        .allowlist_type("VkPhysicalDeviceFeatures2").
        // m91: These functions are not actually implemented.
        blocklist_function("SkCustomTypefaceBuilder_setGlyph[123].*")
        // misc
        .allowlist_var("SK_Color.*")
        .allowlist_var("kAll_GrBackendState")
        .use_core()
        .clang_arg("-std=c++17")
        .clang_args(&["-x", "c++"])
        .clang_arg("-v");

    let target = cargo::target();

    // Don't generate destructors for Windows targets: https://github.com/rust-skia/rust-skia/issues/318
    if target.is_windows() {
        builder = builder.with_codegen_config({
            let mut config = CodegenConfig::default();
            config.remove(CodegenConfig::DESTRUCTORS);
            config
        });
    }

    // 32-bit Windows needs `thiscall` support.
    // https://github.com/rust-skia/rust-skia/issues/540
    if target.is_windows() && target.architecture == "i686" {
        builder = builder.rust_target(RustTarget::Nightly);
    }

    for function in ALLOWLISTED_FUNCTIONS {
        builder = builder.allowlist_function(function)
    }

    for opaque_type in OPAQUE_TYPES {
        builder = builder.opaque_type(opaque_type)
    }

    for t in BLOCKLISTED_TYPES {
        builder = builder.blocklist_type(t);
    }

    let mut cc_build = Build::new();

    for source in &build.binding_sources {
        cc_build.file(source);
        let source = source.to_str().unwrap();
        cargo::rerun_if_file_changed(source);
        builder = builder.header(source);
    }

    let include_path = &build.skia_source_dir;
    cargo::rerun_if_file_changed(include_path.join("include"));

    builder = builder.clang_arg(format!("-I{}", include_path.display()));
    cc_build.include(include_path);

    // Whether GIF decoding is supported,
    // is decided by BUILD.gn based on the existence of the libgifcodec directory:
    if !build
        .definitions
        .iter()
        .any(|(v, _)| v == "SK_USE_LIBGIFCODEC")
    {
        cargo::warning("GIF decoding support may be missing, does the directory skia/third_party/externals/libgifcodec/ exist?")
    }

    for (name, value) in &build.definitions {
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
        cc_build.flag("-std=c++17");
    }

    let target = cargo::target();

    let target_str = &target.to_string();
    cc_build.target(target_str);

    let sdk;
    let sysroot = cargo::env_var("SDKROOT");
    let mut sysroot: Option<&str> = sysroot.as_ref().map(AsRef::as_ref);
    let mut sysroot_flag = "--sysroot=";

    match target.as_strs() {
        (_, "apple", "darwin", _) => {
            // macOS uses `-isysroot/path/to/sysroot`, but this doesn't appear
            // to work for other targets. `--sysroot=` works for all targets,
            // to my knowledge, but doesn't seem to be idiomatic for macOS
            // compilation. To capture this, we allow manually setting sysroot
            // on any platform, but we use `-isysroot` for OSX builds and `--sysroot`
            // elsewhere. If you don't manually set the sysroot, we can automatically
            // detect it, but this is only possible for macOS.
            sysroot_flag = "-isysroot";

            if sysroot.is_none() {
                if let Some(macos_sdk) = xcode::get_sdk_path("macosx") {
                    sdk = macos_sdk;
                    sysroot = Some(
                        sdk.to_str()
                            .expect("macOS SDK path could not be converted to string"),
                    );
                } else {
                    cargo::warning("failed to get macosx SDK path")
                }
            }
        }
        (arch, "linux", "android", _) | (arch, "linux", "androideabi", _) => {
            for arg in android::additional_clang_args(target_str, arch) {
                builder = builder.clang_arg(arg);
            }
        }
        (arch, "apple", "ios", abi) => {
            for arg in ios::additional_clang_args(arch, abi) {
                builder = builder.clang_arg(arg);
            }
        }
        _ => {}
    }

    if let Some(sysroot) = sysroot {
        let sysroot = format!("{}{}", sysroot_flag, sysroot);
        builder = builder.clang_arg(&sysroot);
        cc_build.flag(&sysroot);
    }

    println!("COMPILING BINDINGS: {:?}", build.binding_sources);
    // we add skia-bindings later on.
    cc_build.cargo_metadata(false);
    cc_build.compile(binaries_config::lib::SKIA_BINDINGS);

    println!("GENERATING BINDINGS");
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

const ALLOWLISTED_FUNCTIONS: &[&str] = &[
    "SkAnnotateRectWithURL",
    "SkAnnotateNamedDestination",
    "SkAnnotateLinkToDestination",
    "SkColorTypeBytesPerPixel",
    "SkColorTypeIsAlwaysOpaque",
    "SkColorTypeValidateAlphaType",
    "SkRGBToHSV",
    // this function does not allowlist (probably because of inlining):
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
    "GrGLInterface",
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
    "SkBBoxHierarchy", // vtable
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
    // m81: tuples:
    "SkRuntimeEffect_EffectResult",
    "SkRuntimeEffect_ByteCodeResult",
    "SkRuntimeEffect_SpecializeResult",
    // m81: derives from std::string
    "SkSL::String",
    "std::basic_string",
    "std::basic_string_value_type",
    // m81: wrong size on macOS and Linux
    "SkRuntimeEffect",
    "GrShaderCaps",
    // more stuff we don't need that was tracked down fixing:
    // https://github.com/rust-skia/rust-skia/issues/318
    // referred from SkPath, but not used:
    "SkPathRef",
    "SkMutex",
    // m82: private
    "SkIDChangeListener",
    // m86:
    "GrRecordingContext",
    "GrDirectContext",
    // m87:
    "GrD3DAlloc",
    "GrD3DMemoryAllocator",
    // m87, yuva_pixmaps
    "std::tuple",
    // m93: private, exposed by Paint::asBlendMode(), fails layout tests.
    "skstd::optional",
];

const BLOCKLISTED_TYPES: &[&str] = &[
    // modules/skparagraph
    //   pulls in a std::map<>, which we treat as opaque, but bindgen creates wrong bindings for
    //   std::_Tree* types
    "std::_Tree.*",
    "std::map.*",
    //   debug builds:
    "SkLRUCache",
    "SkLRUCache_Entry",
    //   not used at all:
    "std::vector.*",
    // too much template magic:
    "SkRuntimeEffect_ConstIterable.*",
    // Linux LLVM9 c++17
    "std::_Rb_tree.*",
    // Linux LLVM9 c++17 with SKIA_DEBUG=1
    "std::__cxx.*",
    "std::array.*",
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
    //
    // codec/
    //
    ("DocumentStructureType", rewrite::k_xxx),
    ("ZeroInitialized", rewrite::k_xxx_name),
    ("SelectionPolicy", rewrite::k_xxx),
    //
    // core/ effects/
    //
    ("SkApplyPerspectiveClip", rewrite::k_xxx),
    ("SkBlendMode", rewrite::k_xxx),
    ("SkBlendModeCoeff", rewrite::k_xxx),
    ("SkBlurStyle", rewrite::k_xxx_name),
    ("SkClipOp", rewrite::k_xxx),
    ("SkColorChannel", rewrite::k_xxx),
    ("SkCoverageMode", rewrite::k_xxx),
    ("SkEncodedImageFormat", rewrite::k_xxx),
    ("SkEncodedOrigin", rewrite::k_xxx_name),
    ("SkFilterQuality", rewrite::k_xxx_name),
    ("SkFontHinting", rewrite::k_xxx),
    ("SkAlphaType", rewrite::k_xxx_name),
    ("SkYUVColorSpace", rewrite::k_xxx_name),
    ("SkPathFillType", rewrite::k_xxx),
    ("SkPathConvexityType", rewrite::k_xxx),
    ("SkPathDirection", rewrite::k_xxx),
    ("SkPathVerb", rewrite::k_xxx),
    ("SkPathOp", rewrite::k_xxx_name),
    ("SkTileMode", rewrite::k_xxx),
    // SkPaint_Style
    // SkStrokeRec_Style
    // SkPath1DPathEffect_Style
    ("Style", rewrite::k_xxx_name_opt),
    // SkPaint_Cap
    ("Cap", rewrite::k_xxx_name),
    // SkPaint_Join
    ("Join", rewrite::k_xxx_name),
    // SkStrokeRec_InitStyle
    ("InitStyle", rewrite::k_xxx_name),
    // SkBlurImageFilter_TileMode
    // SkMatrixConvolutionImageFilter_TileMode
    ("TileMode", rewrite::k_xxx_name),
    // SkCanvas_*
    ("PointMode", rewrite::k_xxx_name),
    ("SrcRectConstraint", rewrite::k_xxx_name),
    // SkCanvas_Lattice_RectType
    ("RectType", rewrite::k_xxx),
    // SkDisplacementMapEffect_ChannelSelectorType
    ("ChannelSelectorType", rewrite::k_xxx_name),
    // SkDropShadowImageFilter_ShadowMode
    ("ShadowMode", rewrite::k_xxx_name),
    // SkFont_Edging
    ("Edging", rewrite::k_xxx),
    // SkFont_Slant
    ("Slant", rewrite::k_xxx_name),
    // SkHighContrastConfig_InvertStyle
    ("InvertStyle", rewrite::k_xxx),
    // SkImage_*
    ("BitDepth", rewrite::k_xxx),
    ("CachingHint", rewrite::k_xxx_name),
    ("CompressionType", rewrite::k_xxx),
    // SkImageFilter_MapDirection
    ("MapDirection", rewrite::k_xxx_name),
    // SkCodec_Result
    // SkInterpolatorBase_Result
    ("Result", rewrite::k_xxx),
    // SkMatrix_ScaleToFit
    ("ScaleToFit", rewrite::k_xxx_name),
    // SkPath_*
    ("ArcSize", rewrite::k_xxx_name),
    ("AddPathMode", rewrite::k_xxx_name),
    // SkRegion_Op
    // TODO: remove kLastOp?
    ("Op", rewrite::k_xxx_name_opt),
    // SkRRect_*
    // TODO: remove kLastType?
    // SkRuntimeEffect_Variable_Type
    ("Type", rewrite::k_xxx_name_opt),
    ("Corner", rewrite::k_xxx_name),
    // SkShader_GradientType
    ("GradientType", rewrite::k_xxx_name),
    // SkSurface_*
    ("ContentChangeMode", rewrite::k_xxx_name),
    ("BackendHandleAccess", rewrite::k_xxx_name),
    // SkTextUtils_Align
    ("Align", rewrite::k_xxx_name),
    // SkTrimPathEffect_Mode
    ("Mode", rewrite::k_xxx),
    // SkTypeface_SerializeBehavior
    ("SerializeBehavior", rewrite::k_xxx),
    // SkVertices_VertexMode
    ("VertexMode", rewrite::k_xxx_name),
    // SkYUVAIndex_Index
    ("Index", rewrite::k_xxx_name),
    // SkRuntimeEffect_Variable_Qualifier
    ("Qualifier", rewrite::k_xxx),
    // private type that leaks through SkRuntimeEffect_Variable
    ("GrSLType", rewrite::k_xxx_name),
    //
    // gpu/
    //
    ("GrGLStandard", rewrite::k_xxx_name),
    ("GrGLFormat", rewrite::k_xxx),
    ("GrSurfaceOrigin", rewrite::k_xxx_name),
    ("GrBackendApi", rewrite::k_xxx),
    ("GrMipmapped", rewrite::k_xxx),
    ("GrRenderable", rewrite::k_xxx),
    ("GrProtected", rewrite::k_xxx),
    //
    // DartTypes.h
    //
    ("Affinity", rewrite::k_xxx),
    ("RectHeightStyle", rewrite::k_xxx),
    ("RectWidthStyle", rewrite::k_xxx),
    ("TextAlign", rewrite::k_xxx),
    ("TextDirection", rewrite::k_xxx_uppercase),
    ("TextBaseline", rewrite::k_xxx),
    ("TextHeightBehavior", rewrite::k_xxx),
    ("DrawOptions", rewrite::k_xxx),
    //
    // TextStyle.h
    //
    ("TextDecorationStyle", rewrite::k_xxx),
    ("TextDecorationMode", rewrite::k_xxx),
    ("StyleType", rewrite::k_xxx),
    ("PlaceholderAlignment", rewrite::k_xxx),
    //
    // Vk*
    //
    ("VkChromaLocation", rewrite::vk),
    ("VkFilter", rewrite::vk),
    ("VkFormat", rewrite::vk),
    ("VkImageLayout", rewrite::vk),
    ("VkImageTiling", rewrite::vk),
    ("VkSamplerYcbcrModelConversion", rewrite::vk),
    ("VkSamplerYcbcrRange", rewrite::vk),
    ("VkStructureType", rewrite::vk),
    // m84: SkPath::Verb
    ("Verb", rewrite::k_xxx_name),
    // m84: SkVertices::Attribute::Usage
    ("Usage", rewrite::k_xxx),
    ("GrSemaphoresSubmitted", rewrite::k_xxx),
    ("BackendSurfaceAccess", rewrite::k_xxx),
    // m85
    ("VkSharingMode", rewrite::vk),
    // m86:
    ("SkFilterMode", rewrite::k_xxx),
    ("SkMipmapMode", rewrite::k_xxx),
    ("Enable", rewrite::k_xxx),
    ("ShaderCacheStrategy", rewrite::k_xxx),
    // m87:
    // SkYUVAInfo_PlanarConfig
    ("PlanarConfig", rewrite::k_xxx),
    ("Siting", rewrite::k_xxx),
    // SkYUVAPixmapInfo
    ("DataType", rewrite::k_xxx),
    // m88:
    // SkYUVAInfo_*
    ("PlaneConfig", rewrite::k_xxx),
    // m89, SkImageFilters::Dither
    ("Dither", rewrite::k_xxx),
    ("SkScanlineOrder", rewrite::k_xxx_name),
];

pub(crate) mod rewrite {
    use heck::ShoutySnakeCase;
    use regex::Regex;

    pub fn k_xxx_uppercase(name: &str, variant: &str) -> String {
        k_xxx(name, variant).to_uppercase()
    }

    pub fn k_xxx(name: &str, variant: &str) -> String {
        if let Some(stripped) = variant.strip_prefix('k') {
            stripped.into()
        } else {
            panic!(
                "Variant name '{}' of enum type '{}' is expected to start with a 'k'",
                variant, name
            );
        }
    }

    pub fn _k_xxx_enum(name: &str, variant: &str) -> String {
        capture(name, variant, &format!("k(.*)_{}", name))
    }

    pub fn k_xxx_name_opt(name: &str, variant: &str) -> String {
        let suffix = &format!("_{}", name);
        if variant.ends_with(suffix) {
            capture(name, variant, &format!("k(.*){}", suffix))
        } else {
            capture(name, variant, "k(.*)")
        }
    }

    pub fn k_xxx_name(name: &str, variant: &str) -> String {
        capture(name, variant, &format!("k(.*)_{}", name))
    }

    pub fn vk(name: &str, variant: &str) -> String {
        let prefix = name.to_shouty_snake_case();
        capture(name, variant, &format!("{}_(.*)", prefix))
    }

    fn capture(name: &str, variant: &str, pattern: &str) -> String {
        let re = Regex::new(pattern).unwrap();
        re.captures(variant).unwrap_or_else(|| {
            panic!(
                "failed to match '{}' on enum variant '{}' of enum '{}'",
                pattern, variant, name
            )
        })[1]
            .into()
    }
}

pub use definitions::{Definition, Definitions};

pub(crate) mod definitions {
    use super::env;
    use crate::build_support::features;
    use std::collections::HashSet;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    /// A preprocessor definition.
    pub type Definition = (String, Option<String>);
    /// A container for a number of preprocessor definitions.
    pub type Definitions = Vec<Definition>;

    pub fn from_env() -> Definitions {
        let env_string =
            env::skia_lib_definitions().expect("must include library definition environment");
        from_defines_str(&env_string)
    }

    pub fn save_definitions(
        definitions: &[Definition],
        output_directory: impl AsRef<Path>,
    ) -> std::io::Result<()> {
        fs::create_dir_all(&output_directory)?;
        let mut file = fs::File::create(output_directory.as_ref().join("skia-defines.txt"))?;
        for (name, value) in definitions.iter() {
            if let Some(value) = value {
                writeln!(file, "-D{}={}", name, value)?;
            } else {
                writeln!(file, "-D{}", name)?;
            }
        }
        writeln!(file)
    }

    // Extracts definitions from ninja files that need to be parsed for build consistency.
    pub fn from_ninja_features(
        features: &features::Features,
        output_directory: &Path,
    ) -> Definitions {
        let ninja_files = ninja_files_for_features(features);
        from_ninja_files(ninja_files, output_directory)
    }

    fn from_ninja_files(ninja_files: Vec<PathBuf>, output_directory: &Path) -> Definitions {
        let mut definitions = Vec::new();

        for ninja_file in &ninja_files {
            let ninja_file = output_directory.join(ninja_file);
            let contents = fs::read_to_string(ninja_file).unwrap();
            definitions = combine(definitions, from_ninja_file_content(contents))
        }

        definitions
    }

    /// Parse a defines = line from a ninja build file.
    fn from_ninja_file_content(ninja_file: impl AsRef<str>) -> Definitions {
        let defines = {
            let prefix = "defines = ";
            let defines = ninja_file
                .as_ref()
                .lines()
                .find(|s| s.starts_with(prefix))
                .expect("missing a line with the prefix 'defines =' in a .ninja file");
            &defines[prefix.len()..]
        };
        from_defines_str(defines)
    }

    fn ninja_files_for_features(features: &features::Features) -> Vec<PathBuf> {
        let mut files = vec!["obj/skia.ninja".into()];
        if features.text_layout {
            files.extend(vec![
                "obj/modules/skshaper/skshaper.ninja".into(),
                "obj/modules/skparagraph/skparagraph.ninja".into(),
            ]);
        }
        files
    }

    fn combine(a: Definitions, b: Definitions) -> Definitions {
        remove_duplicates(a.into_iter().chain(b.into_iter()).collect())
    }

    fn remove_duplicates(mut definitions: Definitions) -> Definitions {
        let mut uniques = HashSet::new();
        definitions.retain(|e| uniques.insert(e.0.clone()));
        definitions
    }

    fn from_defines_str(defines: &str) -> Definitions {
        const PREFIX: &str = "-D";
        defines
            .split_whitespace()
            .map(|d| {
                if let Some(stripped) = d.strip_prefix(PREFIX) {
                    stripped
                } else {
                    panic!("missing '{}' prefix from a definition", PREFIX)
                }
            })
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
}
