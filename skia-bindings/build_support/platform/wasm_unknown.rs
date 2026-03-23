use std::{path::{Path, PathBuf}, process::Command};

use super::{generic, prelude::*};

const WASI_UNKNOWN_UNKNOWN_SHIM_SOURCE: &str = "src/wasm_unknown_unknown_wasi_shim.c";
const WASM_UNKNOWN_SYSROOT_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_SYSROOT";
const WASM_UNKNOWN_SYSROOT_URL_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_SYSROOT_URL";

pub struct WasmUnknown;

impl PlatformDetails for WasmUnknown {
    fn uses_freetype(&self) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let features = &config.features;
        let sysroot = skia::env::wasm_unknown_unknown_sysroot();
        let ar = locate_wasi_sdk_bin().join(exe_name("llvm-ar"));

        builder
            .arg("target_cpu", quote("wasm"))
            .arg("target_os", quote("wasm"))
            .arg("ar", quote(&ar.display().to_string()))
            .arg("skia_gl_standard", quote("webgl"))
            .arg("skia_use_webgl", yes_if(features.gpu()));
        builder.cflag("-DSK_BUILD_FOR_UNIX");
        builder.cflag("-D_WASI_EMULATED_MMAN");
        builder.cflag("-D__wasi__=1");
        builder.cflag("-D_LIBCPP_DISABLE_AVAILABILITY");
        builder.cflag("-mllvm");
        builder.cflag("-wasm-enable-sjlj");
        builder.cflag(format!("--sysroot={}", sysroot.display()));
        builder.cflags(common_clang_args(&sysroot));

        builder
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes());
    }

    fn bindgen_args(&self, _target: &Target, builder: &mut BindgenArgsBuilder) {
        let sysroot = skia::env::wasm_unknown_unknown_sysroot();
        builder.arg("-fvisibility=default");
        builder.arg("-DSK_BUILD_FOR_UNIX");
        builder.args(common_clang_args(&sysroot));
        builder.arg(format!("--sysroot={}", sysroot.display()));
        builder.arg("-mllvm");
        builder.arg("-wasm-enable-sjlj");
        builder.arg("-D_LIBCPP_DISABLE_AVAILABILITY");
        builder.arg("-D__wasi__=1");
    }

    fn configure_build_environment(&self, target: &Target, output_directory: &Path) {
        let bin = locate_wasi_sdk_bin_from(output_directory);
        let clang = bin.join(exe_name("clang"));
        let clangxx = bin.join(exe_name("clang++"));
        let ar = bin.join(exe_name("llvm-ar"));

        assert!(clang.is_file(), "WASI SDK clang not found at {}", clang.display());
        assert!(
            clangxx.is_file(),
            "WASI SDK clang++ not found at {}",
            clangxx.display()
        );
        assert!(ar.is_file(), "WASI SDK llvm-ar not found at {}", ar.display());

        let clang = clang.display().to_string();
        let clangxx = clangxx.display().to_string();
        let ar = ar.display().to_string();
        let target_var = target.to_string().replace('-', "_");
        std::env::set_var("CLANGCC", &clang);
        std::env::set_var("CLANGCXX", &clangxx);
        std::env::set_var("CC", &clang);
        std::env::set_var("CXX", &clangxx);
        std::env::set_var("AR", &ar);
        std::env::set_var(format!("CC_{target_var}"), &clang);
        std::env::set_var(format!("CXX_{target_var}"), &clangxx);
        std::env::set_var(format!("AR_{target_var}"), &ar);

        if std::env::var_os(WASM_UNKNOWN_SYSROOT_ENV).is_none()
            && std::env::var_os(WASM_UNKNOWN_SYSROOT_URL_ENV).is_none()
        {
            if let Some(sdk_root) = bin.parent() {
                let sysroot = sdk_root.join("share/wasi-sysroot");
                if sysroot.is_dir() {
                    std::env::set_var(WASM_UNKNOWN_SYSROOT_ENV, sysroot);
                }
            }
        }
    }

    fn prepare_build_support(&self, output_directory: &Path) {
        compile_wasi_shim(output_directory);
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        let mut libraries = generic::link_libraries(features);
        libraries.extend([
            "static=c".into(),
            "static=c++".into(),
            "static=c++abi".into(),
            "static=wasi-emulated-mman".into(),
            "static=wasi-shim".into(),
        ]);
        libraries
    }

    fn link_search_paths(&self) -> Vec<String> {
        let sysroot = skia::env::wasm_unknown_unknown_sysroot();
        vec![sysroot.join("lib/wasm32-wasip1").display().to_string()]
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

pub fn compile_wasi_shim(output_directory: &Path) {
    let sdk_bin = locate_wasi_sdk_bin();
    let clang = sdk_bin.join(if cargo::host().is_windows() {
        "clang.exe"
    } else {
        "clang"
    });
    assert!(
        clang.is_file(),
        "WASI SDK clang not found at {}",
        clang.display()
    );

    let mut build = cc::Build::new();
    build
        .compiler(clang)
        .cargo_metadata(false)
        .file(WASI_UNKNOWN_UNKNOWN_SHIM_SOURCE)
        .flag("--target=wasm32-unknown-unknown")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .out_dir(output_directory)
        .compile("wasi-shim");
}

fn common_clang_args(sysroot: &Path) -> Vec<String> {
    let include = sysroot.join("include");
    let target_include = include.join("wasm32-wasip1");
    let cxx_include = target_include.join("c++/v1");
    [
        format!("-isystem{}", cxx_include.display()),
        format!("-isystem{}", target_include.display()),
        format!("-isystem{}", include.display()),
    ]
    .into()
}

fn locate_wasi_sdk_bin() -> PathBuf {
    locate_wasi_sdk_bin_from(&cargo::output_directory())
}

fn locate_wasi_sdk_bin_from(output_directory: &Path) -> PathBuf {
    if let Some(bin) = cargo::env_var("SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK_BIN") {
        let bin = PathBuf::from(bin);
        assert!(
            bin.is_dir(),
            "SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK_BIN does not point to a directory: {}",
            bin.display()
        );
        return bin;
    }
    ensure_wasi_sdk(output_directory).join("bin")
}

fn ensure_wasi_sdk(output_directory: &Path) -> PathBuf {
    if let Some(path) = cargo::env_var("SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK") {
        let path = PathBuf::from(path);
        assert!(
            path.is_dir(),
            "SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK does not point to an existing directory: {}",
            path.display()
        );
        return path;
    }

    let cache_dir = output_directory
        .join(".cache")
        .join("skia-wasm-runtime");
    let sdk_root = cache_dir.join(wasi_sdk_dir_name());
    if sdk_root.join("bin").is_dir() && sdk_root.join("share/wasi-sysroot").is_dir() {
        return sdk_root;
    }

    std::fs::create_dir_all(&cache_dir).expect("failed to create wasm runtime cache directory");

    let archive_name = wasi_sdk_archive_name();
    let archive_path = cache_dir.join(&archive_name);
    if !archive_path.is_file() {
        let url = cargo::env_var("SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK_URL")
            .unwrap_or_else(|| format!("https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-32/{archive_name}"));
        let status = Command::new("curl")
            .args([
                "-L",
                "-f",
                "-sS",
                &url,
                "--output",
                archive_path
                    .to_str()
                    .expect("non-utf8 wasi-sdk archive path"),
            ])
            .status()
            .expect("failed to run curl while downloading WASI SDK");
        assert!(
            status.success(),
            "failed to download WASI SDK from {url}, status: {status}"
        );
    }

    let status = Command::new("tar")
        .args([
            "-xzf",
            archive_path
                .to_str()
                .expect("non-utf8 wasi-sdk archive path"),
            "-C",
            cache_dir.to_str().expect("non-utf8 cache directory path"),
        ])
        .status()
        .expect("failed to unpack WASI SDK archive");
    assert!(
        status.success(),
        "failed to unpack WASI SDK archive {}, status: {status}",
        archive_path.display()
    );

    assert!(
        sdk_root.join("bin").is_dir(),
        "WASI SDK bin directory missing after unpacking: {}",
        sdk_root.join("bin").display()
    );
    sdk_root
}

fn wasi_sdk_archive_name() -> String {
    if cargo::host().is_windows() {
        format!("wasi-sdk-32.0-{}-windows.tar.gz", host_arch())
    } else if cfg!(target_os = "macos") {
        format!("wasi-sdk-32.0-{}-macos.tar.gz", host_arch())
    } else if cfg!(target_os = "linux") {
        format!("wasi-sdk-32.0-{}-linux.tar.gz", host_arch())
    } else {
        panic!("unsupported host OS for WASI SDK download");
    }
}

fn wasi_sdk_dir_name() -> String {
    if cargo::host().is_windows() {
        format!("wasi-sdk-32.0-{}-windows", host_arch())
    } else if cfg!(target_os = "macos") {
        format!("wasi-sdk-32.0-{}-macos", host_arch())
    } else if cfg!(target_os = "linux") {
        format!("wasi-sdk-32.0-{}-linux", host_arch())
    } else {
        panic!("unsupported host OS for WASI SDK download");
    }
}

fn host_arch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "arm64",
        arch => panic!("unsupported host arch for WASI SDK download: {arch}"),
    }
}

fn exe_name(name: &str) -> String {
    if cargo::host().is_windows() {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}
