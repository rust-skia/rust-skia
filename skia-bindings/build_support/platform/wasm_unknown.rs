#[cfg(feature = "binary-cache")]
use crate::build_support::binary_cache;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::prelude::*;

const MALLOC_UNKNOWN_UNKNOWN_SHIM_SOURCE: &str = "src/wasm_unknown_unknown_malloc_shim.cpp";
const WASM_UNKNOWN_SYSROOT_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_SYSROOT";
const WASI_SDK_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK";
const WASI_SDK_BIN_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK_BIN";
const WASI_SDK_URL_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_WASI_SDK_URL";
const WASI_RUNTIME_CACHE_DIR: &str = "skia-wasm-runtime";
const WASI_SDK_RELEASE: &str = "32";
const WASI_SDK_VERSION: &str = "32.0";

pub struct WasmUnknown;

impl PlatformDetails for WasmUnknown {
    fn uses_freetype(&self) -> bool {
        true
    }

    fn gn_args(&self, _config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let sysroot = wasi_sysroot();
        let ar = locate_wasi_sdk_bin().join(exe_name("llvm-ar"));

        builder
            .arg("target_cpu", quote("wasm"))
            .arg("target_os", quote("wasm"))
            .arg("ar", quote(&ar.display().to_string()))
            .arg("skia_gl_standard", quote("gles"))
            .arg("skia_use_webgl", no())
            .arg("malloc", quote("malloc-shim"));
        builder.cflag("-DSK_BUILD_FOR_UNIX");
        builder.cflag("-D_WASI_EMULATED_MMAN");
        builder.cflag("-D_WASI_EMULATED_SIGNAL");
        builder.cflag("-D__wasi__=1");
        builder.cflag("-D_LIBCPP_DISABLE_AVAILABILITY");
        builder.cflag("-DU_HAVE_TZSET=0");
        builder.cflag("-DU_HAVE_TIMEZONE=0");
        builder.cflag("-DU_HAVE_TZNAME=0");
        builder.cflag("-mllvm");
        builder.cflag("-wasm-enable-sjlj");
        builder.cflag(format!("--sysroot={}", sysroot.display()));
        builder.cflags(common_clang_args(&sysroot));

        builder
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes());
    }

    fn bindgen_args(&self, _target: &Target, builder: &mut BindgenArgsBuilder) {
        let sysroot = wasi_sysroot();
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

        assert!(
            clang.is_file(),
            "WASI SDK clang not found at {}",
            clang.display()
        );
        assert!(
            clangxx.is_file(),
            "WASI SDK clang++ not found at {}",
            clangxx.display()
        );
        assert!(
            ar.is_file(),
            "WASI SDK llvm-ar not found at {}",
            ar.display()
        );

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
    }

    fn prepare_build_support(&self, output_directory: &Path) {
        compile_malloc_shim(output_directory);
    }

    fn link_libraries(&self, _features: &Features) -> Vec<String> {
        let mut libraries = Vec::new();
        libraries.extend([
            "static=malloc-shim".into(),
            "static=c".into(),
            "static=c++".into(),
            "static=c++abi".into(),
            "static=setjmp".into(),
            "static=wasi-emulated-mman".into(),
            "static=wasi-emulated-signal".into(),
        ]);
        libraries
    }

    fn link_search_paths(&self) -> Vec<String> {
        let sysroot = wasi_sysroot();
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

pub fn compile_malloc_shim(output_directory: &Path) {
    compile_shim(
        output_directory,
        MALLOC_UNKNOWN_UNKNOWN_SHIM_SOURCE,
        "malloc-shim",
        true,
    );
}

fn compile_shim(output_directory: &Path, source: &str, output: &str, cxx: bool) {
    cargo::rerun_if_file_changed(source);

    let sdk_bin = locate_wasi_sdk_bin();
    let sysroot = wasi_sysroot();
    let clang = sdk_bin.join(exe_name(if cxx { "clang++" } else { "clang" }));
    assert!(
        clang.is_file(),
        "WASI SDK clang not found at {}",
        clang.display()
    );

    let mut build = cc::Build::new();
    if cxx {
        build.cpp(true).flag("-std=c++20");
    }

    build
        .compiler(clang)
        .cargo_metadata(false)
        .file(source)
        .flag("--target=wasm32-unknown-unknown")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag(format!("--sysroot={}", sysroot.display()))
        .flags(common_clang_args(&sysroot))
        .out_dir(output_directory)
        .compile(output);
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

fn wasi_sysroot() -> PathBuf {
    wasi_sysroot_from(&cargo::output_directory())
}

fn wasi_sysroot_from(output_directory: &Path) -> PathBuf {
    if let Some(path) = cargo::env_var(WASM_UNKNOWN_SYSROOT_ENV) {
        let path = PathBuf::from(path);
        assert!(
            path.is_dir(),
            "{WASM_UNKNOWN_SYSROOT_ENV} does not point to an existing directory: {}",
            path.display()
        );
        return path;
    }

    if let Some(sdk_root) = wasi_sdk_root_from_env() {
        return ensure_wasi_sysroot_dir(&sdk_root.join("share/wasi-sysroot"), WASI_SDK_ENV);
    }

    if let Some(bin) = wasi_sdk_bin_from_env() {
        let Some(sdk_root) = bin.parent() else {
            panic!(
                "{WASI_SDK_BIN_ENV} must point to a WASI SDK bin directory so the sysroot can be inferred"
            );
        };
        return ensure_wasi_sysroot_dir(&sdk_root.join("share/wasi-sysroot"), WASI_SDK_BIN_ENV);
    }

    ensure_wasi_sysroot_dir(
        &ensure_wasi_sdk(output_directory).join("share/wasi-sysroot"),
        "downloaded WASI SDK",
    )
}

fn locate_wasi_sdk_bin_from(output_directory: &Path) -> PathBuf {
    if let Some(bin) = wasi_sdk_bin_from_env() {
        return bin;
    }

    if let Some(path) = wasi_sdk_root_from_env() {
        return path.join("bin");
    }

    ensure_wasi_sdk(output_directory).join("bin")
}

fn wasi_sdk_bin_from_env() -> Option<PathBuf> {
    let bin = cargo::env_var(WASI_SDK_BIN_ENV).map(PathBuf::from)?;
    assert!(
        bin.is_dir(),
        "{WASI_SDK_BIN_ENV} does not point to a directory: {}",
        bin.display()
    );
    Some(bin)
}

fn wasi_sdk_root_from_env() -> Option<PathBuf> {
    let path = cargo::env_var(WASI_SDK_ENV).map(PathBuf::from)?;
    assert!(
        path.is_dir(),
        "{WASI_SDK_ENV} does not point to an existing directory: {}",
        path.display()
    );
    Some(path)
}

fn ensure_wasi_sdk(output_directory: &Path) -> PathBuf {
    let cache_dir = output_directory.join(".cache").join(WASI_RUNTIME_CACHE_DIR);
    let sdk_root = cache_dir.join(wasi_sdk_dir_name());
    if sdk_root.join("bin").is_dir() && sdk_root.join("share/wasi-sysroot").is_dir() {
        return sdk_root;
    }

    fs::create_dir_all(&cache_dir).expect("failed to create wasm runtime cache directory");

    let archive_name = wasi_sdk_archive_name();
    let archive_path = cache_dir.join(&archive_name);
    if !archive_path.is_file() {
        let url = cargo::env_var(WASI_SDK_URL_ENV).unwrap_or_else(wasi_sdk_download_url);
        download_to_path(&url, &archive_path, "WASI SDK");
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
    assert!(
        sdk_root.join("share/wasi-sysroot").is_dir(),
        "WASI SDK sysroot missing after unpacking: {}",
        sdk_root.join("share/wasi-sysroot").display()
    );
    sdk_root
}

fn ensure_wasi_sysroot_dir(path: &Path, source: &str) -> PathBuf {
    assert!(
        path.is_dir(),
        "WASI sysroot not found for {source}: {}",
        path.display()
    );
    path.to_path_buf()
}

fn wasi_sdk_download_url() -> String {
    format!(
        "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-{WASI_SDK_RELEASE}/{}",
        wasi_sdk_archive_name()
    )
}

fn wasi_sdk_archive_name() -> String {
    if cargo::host().is_windows() {
        format!("wasi-sdk-{WASI_SDK_VERSION}-{}-windows.tar.gz", host_arch())
    } else if cfg!(target_os = "macos") {
        format!("wasi-sdk-{WASI_SDK_VERSION}-{}-macos.tar.gz", host_arch())
    } else if cfg!(target_os = "linux") {
        format!("wasi-sdk-{WASI_SDK_VERSION}-{}-linux.tar.gz", host_arch())
    } else {
        panic!("unsupported host OS for WASI SDK download");
    }
}

fn wasi_sdk_dir_name() -> String {
    if cargo::host().is_windows() {
        format!("wasi-sdk-{WASI_SDK_VERSION}-{}-windows", host_arch())
    } else if cfg!(target_os = "macos") {
        format!("wasi-sdk-{WASI_SDK_VERSION}-{}-macos", host_arch())
    } else if cfg!(target_os = "linux") {
        format!("wasi-sdk-{WASI_SDK_VERSION}-{}-linux", host_arch())
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

fn download_to_path(url: &str, destination: &Path, artifact_name: &str) {
    #[cfg(feature = "binary-cache")]
    {
        let bytes = binary_cache::utils::download(url, true)
            .unwrap_or_else(|e| panic!("failed to download {artifact_name} from {url}: {e}"));
        fs::write(destination, bytes)
            .unwrap_or_else(|e| panic!("failed to write {artifact_name} archive: {e}"));
    }

    #[cfg(not(feature = "binary-cache"))]
    {
        let status = Command::new("curl")
            .args([
                "-L",
                "-f",
                "-sS",
                url,
                "--output",
                destination
                    .to_str()
                    .expect("non-utf8 archive destination path"),
            ])
            .status()
            .expect("failed to run curl while downloading artifact");
        assert!(
            status.success(),
            "failed to download {artifact_name} from {url}, status: {status}"
        );
    }
}

fn exe_name(name: &str) -> String {
    if cargo::host().is_windows() {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}
