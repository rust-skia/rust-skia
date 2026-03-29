#[cfg(feature = "binary-cache")]
use crate::build_support::binary_cache;
/// Environment variables used for configuring the Skia build.
use crate::build_support::cargo;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// A boolean specifying whether to build Skia's dependencies or not. If not, the system's
/// provided libraries are used.
pub fn use_system_libraries() -> bool {
    cargo::env_var("SKIA_USE_SYSTEM_LIBRARIES").is_some()
}

/// The full path of the ninja command to run.
pub fn ninja_command() -> Option<PathBuf> {
    cargo::env_var("SKIA_NINJA_COMMAND").map(PathBuf::from)
}

/// The full path of the gn command to run.
pub fn gn_command() -> Option<PathBuf> {
    cargo::env_var("SKIA_GN_COMMAND").map(PathBuf::from)
}

const WASM_UNKNOWN_SYSROOT_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_SYSROOT";
const WASM_UNKNOWN_SYSROOT_URL_ENV: &str = "SKIA_WASM32_UNKNOWN_UNKNOWN_SYSROOT_URL";
const WASM_UNKNOWN_SYSROOT_DIR: &str = "wasi-sysroot-32.0";
const WASM_UNKNOWN_SYSROOT_DEFAULT_URL: &str = "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-32/wasi-sysroot-32.0.tar.gz";

pub fn wasm_unknown_unknown_sysroot() -> PathBuf {
    if let Some(path) = cargo::env_var(WASM_UNKNOWN_SYSROOT_ENV) {
        let path = PathBuf::from(path);
        assert!(
            path.is_dir(),
            "{WASM_UNKNOWN_SYSROOT_ENV} does not point to an existing directory: {}",
            path.display()
        );
        return path;
    }

    let url = cargo::env_var(WASM_UNKNOWN_SYSROOT_URL_ENV)
        .unwrap_or_else(|| WASM_UNKNOWN_SYSROOT_DEFAULT_URL.to_string());
    let cache_dir = cargo::output_directory()
        .join(".cache")
        .join("skia-wasm-runtime");

    ensure_wasm_unknown_unknown_sysroot_cached(&cache_dir, &url)
}

fn ensure_wasm_unknown_unknown_sysroot_cached(cache_dir: &Path, url: &str) -> PathBuf {
    let sysroot_dir = cache_dir.join(WASM_UNKNOWN_SYSROOT_DIR);
    if sysroot_dir.is_dir() {
        return sysroot_dir;
    }

    fs::create_dir_all(cache_dir).expect("failed to create wasm runtime cache directory");

    let archive = cache_dir.join(format!("{WASM_UNKNOWN_SYSROOT_DIR}.tar.gz"));
    if !archive.is_file() {
        download_to_path(url, &archive, "WASI sysroot");
    }

    let status = Command::new("tar")
        .args([
            "-xzf",
            archive.to_str().expect("non-utf8 archive path"),
            "-C",
            cache_dir.to_str().expect("non-utf8 cache directory path"),
        ])
        .status()
        .expect("failed to unpack WASI sysroot archive");
    assert!(
        status.success(),
        "failed to unpack WASI sysroot archive {}, status: {status}",
        archive.display()
    );

    assert!(
        sysroot_dir.is_dir(),
        "WASI sysroot directory is missing after unpacking: {}",
        sysroot_dir.display()
    );
    sysroot_dir
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
