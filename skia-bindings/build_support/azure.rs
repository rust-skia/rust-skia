use crate::build_support::skia::Configuration;
use crate::build_support::{binaries, cargo, git};
use crate::SRC_BINDINGS_RS;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn is_active() -> bool {
    artifact_staging_directory().is_some()
}

pub fn artifact_staging_directory() -> Option<PathBuf> {
    env::var("BUILD_ARTIFACTSTAGINGDIRECTORY")
        .map(|dir| PathBuf::from(dir))
        .ok()
}

pub fn copy_binaries(config: &Configuration, artifacts: &Path) -> io::Result<()> {
    let half_hash = git::half_hash().expect("failed to retrieve the git hash");
    let key = binaries::key(&half_hash, &config.features);

    let binaries = prepare_binaries(&key, artifacts)?;

    fs::copy(SRC_BINDINGS_RS, binaries.join("bindings.rs"))?;

    let output_directory = &config.output_directory;

    let target_is_windows = cargo::target().system == "windows";
    let (skia_lib, skia_bindings_lib) = if target_is_windows {
        ("skia.lib", "skia-bindings.lib")
    } else {
        ("libskia.a", "libskia-bindings.a")
    };

    fs::copy(output_directory.join(skia_lib), binaries.join(skia_lib))?;
    fs::copy(
        output_directory.join(skia_bindings_lib),
        binaries.join(skia_bindings_lib),
    )?;

    Ok(())
}

/// Prepares the binaries directory and sets the tag.txt and key.txt
/// file.
pub fn prepare_binaries(key: &str, artifacts: &Path) -> io::Result<PathBuf> {
    let binaries = artifacts.join("skia-binaries");
    fs::create_dir_all(&binaries)?;

    // this is primarily for azure to know the tag and the key of the binaries,
    // but they can stay inside the archive.

    {
        let mut tag_file = File::create(binaries.join("tag.txt")).unwrap();
        tag_file.write_all(cargo::package_version().as_bytes())?;
    }
    {
        let mut key_file = File::create(binaries.join("key.txt")).unwrap();
        key_file.write_all(key.as_bytes())?;
    }

    Ok(binaries)
}
