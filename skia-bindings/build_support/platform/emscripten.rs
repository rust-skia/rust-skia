use std::path::Path;

use super::{generic, prelude::*};

pub struct Emscripten;

impl PlatformDetails for Emscripten {
    fn uses_freetype(&self) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let features = &config.features;
        let emsdk = emsdk_layout();

        builder
            .arg("skia_gl_standard", quote("webgl"))
            .arg("skia_use_webgl", yes_if(features.ganesh()))
            .arg("target_cpu", quote("wasm"));

        // The rust-skia wasm toolchain in Skia derives `ar`/`cc`/`cxx` from
        // `skia_emsdk_dir` in the classic `<emsdk>/upstream/emscripten` layout.
        // For all other layouts, `skia_emsdk_dir` is left unset, so the
        // toolchain inherits the top-level `ar`/`cc`/`cxx` GN arguments that
        // are written below.
        if let Some(emcc_dir) = &emsdk.emcc_dir {
            // `cc`/`cxx`/`ar` are also set from CC/CXX by
            // `FinalBuildConfiguration::from_build_configuration`, the
            // arguments written here win because they come later and
            // duplicates are dropped by `platform::gn_args`.
            let cc = format!("{emcc_dir}/emcc");
            let cxx = format!("{emcc_dir}/em++");
            let ar = format!("{emcc_dir}/emar");
            builder.arg("ar", quote(&ar));
            builder.arg("cc", quote(&cc));
            builder.arg("cxx", quote(&cxx));

            // Without `skia_emsdk_dir`, Skia's `config("wasm")` does not add
            // the sysroot either, so supply it explicitly.
            let sysroot = format!("{emcc_dir}/cache/sysroot");
            builder.cflags(vec![format!("--sysroot={sysroot}")]);
            builder.arg("extra_ldflags", format!("[\"--sysroot={sysroot}\"]"));
        } else {
            builder.arg("skia_emsdk_dir", quote(&emsdk.base_dir));
        }

        // The custom embedded font manager is enabled by default on WASM, but depends
        // on the undefined symbol `SK_EMBEDDED_FONTS`. Enable the custom empty font
        // manager instead so typeface creation still works.
        // See https://github.com/rust-skia/rust-skia/issues/648
        builder
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes());
    }

    fn bindgen_args(&self, _target: &cargo::Target, builder: &mut BindgenArgsBuilder) {
        builder.arg("-nobuiltininc");

        // visibility=default, otherwise some types may be missing:
        // <https://github.com/rust-lang/rust-bindgen/issues/751#issuecomment-555735577>
        builder.arg("-fvisibility=default");

        let emsdk = emsdk_layout();

        let emscripten_dir = emsdk
            .emcc_dir
            .clone()
            .unwrap_or_else(|| format!("{}/upstream/emscripten", emsdk.base_dir));

        let sysroot_include = format!("{emscripten_dir}/cache/sysroot/include");
        if Path::new(&sysroot_include).is_dir() {
            // Newer emsdk versions expose headers via cache/sysroot and reject direct includes
            // from upstream/emscripten/system.
            let libcxx_include = format!("{sysroot_include}/c++/v1");
            if Path::new(&libcxx_include).is_dir() {
                builder.arg(format!("-isystem{libcxx_include}"));
            }
            builder.arg(format!("-isystem{sysroot_include}"));

            // For xlocale.h
            let compat_include = format!("{sysroot_include}/compat");
            if Path::new(&compat_include).is_dir() {
                builder.arg(format!("-isystem{compat_include}"));
            }
            return;
        }

        // Add C++ includes (otherwise build will fail with <cmath> not found)
        let mut add_sys_include = |path: &str| {
            builder.arg(format!("-isystem{emscripten_dir}/system/{path}"));
        };

        add_sys_include("lib/libc/musl/arch/emscripten");
        add_sys_include("lib/libc/musl/arch/generic");
        add_sys_include("lib/libcxx/include");
        add_sys_include("lib/libc/musl/include");
        add_sys_include("include");
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        generic::link_libraries(features)
    }

    fn filter_platform_features(
        &self,
        _use_system_libraries: bool,
        mut features: Features,
    ) -> Features {
        features += feature::EMBED_FREETYPE;
        features
    }
}

/// Resolved Emscripten SDK layout.
///
/// Classic emsdk installations (up to and including 5.x installed with the
/// `emsdk` tool) place the toolchain below `upstream/emscripten`; bare
/// emscripten release archives (newer emsdk versions) unpack the compiler
/// directly below their own root. Nothing needs to point into the SDK with a
/// symlink: the layout is detected from the filesystem, and `cc`/`cxx`/`ar`
/// are passed to Skia's GN explicitly when the toolchain is not in the classic
/// location.
struct EmsdkLayout {
    /// The `EMSDK` environment variable value, as passed to `skia_emsdk_dir`.
    base_dir: String,
    /// Set when the toolchain is *not* in the classic
    /// `<base_dir>/upstream/emscripten` location: the directory containing
    /// `emcc`, `em++`, `emar`, and the `cache/sysroot` directory.
    emcc_dir: Option<String>,
}

fn emsdk_layout() -> EmsdkLayout {
    let base_dir = emsdk_base_dir();

    // Classic emsdk layout: <emsdk>/upstream/emscripten/...
    let upstream = format!("{}/upstream/emscripten", base_dir);
    if Path::new(&format!("{upstream}/emcc")).exists() {
        return EmsdkLayout {
            base_dir,
            emcc_dir: None,
        };
    }

    // Flat release layout: <root>/emscripten/...
    let flat = format!("{base_dir}/emscripten");
    if Path::new(&format!("{flat}/emcc")).exists() {
        let emcc_dir = flat;
        return EmsdkLayout {
            base_dir,
            emcc_dir: Some(emcc_dir),
        };
    }

    // The activated emscripten directory itself.
    if Path::new(&format!("{base_dir}/emcc")).exists() {
        let emcc_dir = base_dir.clone();
        return EmsdkLayout {
            base_dir,
            emcc_dir: Some(emcc_dir),
        };
    }

    panic!(
        "`emcc` not found below the directory $EMSDK points to (`{base_dir}`). \
         Please activate the Emscripten SDK, for example by sourcing \
         `emsdk_env.sh`, or set $EMSDK to the emscripten root directory."
    )
}

fn emsdk_base_dir() -> String {
    match std::env::var("EMSDK") {
        Ok(val) => val,
        Err(_e) => panic!(
            "please set the EMSDK environment variable to the root of your Emscripten installation"
        ),
    }
}
