use std::fs;
use std::path::Path;

use super::generic;
use super::prelude::*;

pub struct Emscripten;

impl PlatformDetails for Emscripten {
    fn uses_freetype(&self) -> bool {
        true
    }

    fn provides_tools(&self) -> bool {
        // The emsdk compilers (`emcc`/`em++`/`emar`) must be used, host
        // compiler tools cannot build for the emscripten target.
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let features = &config.features;
        let emcc_dir = emscripten_dir();

        builder
            .arg("skia_gl_standard", quote("webgl"))
            .arg("skia_use_webgl", yes_if(features.ganesh()))
            .arg("target_cpu", quote("wasm"));

        // `skia_emsdk_dir` is deliberately not set. Skia's wasm toolchain and `config("wasm")`
        // derive the compiler and sysroot paths from it as `<dir>/upstream/emscripten/...` (see
        // `gn/toolchain/BUILD.gn` and `gn/skia/BUILD.gn`), a layout that flat emsdk releases do not
        // have. Setting it would therefore point Skia at paths that do not exist. With it unset,
        // the toolchain falls back to the top-level `ar`/`cc`/`cxx` arguments supplied below, and
        // the sysroot, which Skia otherwise only adds alongside `skia_emsdk_dir`, is passed
        // explicitly. Host `CC`/`CXX` are disregarded here (see `PlatformDetails::provides_tools`).
        let cc = format!("{emcc_dir}/emcc");
        let cxx = format!("{emcc_dir}/em++");
        let ar = format!("{emcc_dir}/emar");
        let sysroot = format!("{emcc_dir}/cache/sysroot");
        builder
            .arg("ar", quote(&ar))
            .arg("cc", quote(&cc))
            .arg("cxx", quote(&cxx));
        builder.cflags(vec![format!("--sysroot={sysroot}")]);
        builder.arg("extra_ldflags", format!("[\"--sysroot={sysroot}\"]"));

        // The custom embedded font manager is enabled by default on WASM, but depends on the
        // undefined symbol `SK_EMBEDDED_FONTS`. Enable the custom empty font manager instead so
        // typeface creation still works. See <https://github.com/rust-skia/rust-skia/issues/648>
        builder
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes());
    }

    fn bindgen_args(&self, _target: &cargo::Target, builder: &mut BindgenArgsBuilder) {
        builder.arg("-nobuiltininc");

        // visibility=default, otherwise some types may be missing:
        // <https://github.com/rust-lang/rust-bindgen/issues/751#issuecomment-555735577>
        builder.arg("-fvisibility=default");

        let emcc_dir = emscripten_dir();
        check_bindgen_clang_version(&emcc_dir);

        let sysroot_include = format!("{emcc_dir}/cache/sysroot/include");
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
            builder.arg(format!("-isystem{emcc_dir}/system/{path}"));
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
/// Classic emsdk installations (up to and including 5.x installed with the `emsdk` tool) place the
/// toolchain below `upstream/emscripten`; bare emscripten release archives (newer emsdk versions)
/// unpack the compiler directly below their own root. Nothing needs to point into the SDK with a
/// symlink: the layout is detected from the filesystem, and the compiler tools are passed to Skia's
/// GN explicitly for all layouts.
fn emscripten_dir() -> String {
    let base_dir = emsdk_base_dir();

    // Classic emsdk layout: <emsdk>/upstream/emscripten/...
    let upstream = format!("{base_dir}/upstream/emscripten");
    if Path::new(&format!("{upstream}/emcc")).exists() {
        return upstream;
    }

    // Flat release layout: <root>/emscripten/...
    let flat = format!("{base_dir}/emscripten");
    if Path::new(&format!("{flat}/emcc")).exists() {
        return flat;
    }

    // The activated emscripten directory itself.
    if Path::new(&format!("{base_dir}/emcc")).exists() {
        return base_dir;
    }

    panic!(
        "`emcc` not found below the directory $EMSDK points to (`{base_dir}`). \
         Please activate the Emscripten SDK, for example by sourcing \
         `emsdk_env.sh`, or set $EMSDK to the emscripten root directory."
    )
}

fn emsdk_base_dir() -> String {
    // `cargo::env_var` also notifies cargo to re-run the build script when
    // the environment variable changes.
    match cargo::env_var("EMSDK") {
        Some(val) => val,
        None => panic!(
            "please set the EMSDK environment variable to the root of your Emscripten installation"
        ),
    }
}

/// Warns if the libclang used by bindgen is older than the clang that is bundled with the emsdk.
///
/// The emsdk does not ship a libclang library, bindgen uses the host's installation. An older host
/// libclang may parse the emscripten headers differently than the compiler that will process them
/// later.
///
/// The expected clang version is read from emscripten's own `tools/shared.py`
/// (`EXPECTED_LLVM_VERSION`), the same constant emscripten uses to validate its compiler, so no
/// command output has to be parsed.
fn check_bindgen_clang_version(emcc_dir: &str) {
    let expected_llvm_version = fs::read_to_string(format!("{emcc_dir}/tools/shared.py"))
        .ok()
        .and_then(|shared_py| {
            shared_py
                .lines()
                .find(|line| line.starts_with("EXPECTED_LLVM_VERSION"))
                .and_then(|line| line.split('=').nth(1))
                .and_then(|value| value.trim().parse::<u32>().ok())
        });

    let Some(emsdk_major) = expected_llvm_version else {
        return;
    };

    let bindgen_version = bindgen::clang_version();
    if let Some((bindgen_major, _)) = bindgen_version.parsed {
        if bindgen_major < emsdk_major {
            cargo::warning(format!(
                "The libclang used by bindgen reports version {bindgen_major}, while the clang \
                 bundled with the emsdk has version {emsdk_major}. Consider updating the host \
                 LLVM installation (or point LIBCLANG_PATH to the emsdk compatible installation), \
                 the versions might generate bindings that are incompatible with the headers."
            ));
        }
    }
}
