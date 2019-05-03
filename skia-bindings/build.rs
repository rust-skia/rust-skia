mod build_support;
use crate::build_support::skia::Configuration;
use crate::build_support::{binaries, git};
use build_support::skia;
use std::path::Path;
use std::{fs, io};

const SRC_BINDINGS_RS: &str = "src/bindings.rs";

fn main() {
    let config = skia::Configuration::from_cargo_env();

    //
    // download of prebuilt binaries possible?
    //

    let mut do_full_build = true;

    if let Some(key) = should_try_download_binaries(&config) {
        println!("TRYING TO DOWNLOAD AND INSTALL SKIA BINARIES: {}", key);
        if let Err(e) = download_and_install(&key, &config.output_directory) {
            println!("DOWNLOAD AND INSTALL FAILED: {}", e)
        } else {
            do_full_build = false;
        }
    }

    //
    // full build?
    //

    if do_full_build {
        println!("STARTING A FULL BUILD");
        skia::build(&config);
    }

    config.commit_to_cargo();

    //
    // publish binaries?
    //

    // TODO: we may not want to deliver binaries when we did a full build
    //       but how to inform azure if we don't want to?
    if let Some(staging_directory) = azure::artifact_staging_directory() {
        println!(
            "DETECTED AZURE, delivering binaries to {}",
            staging_directory.to_str().unwrap()
        );

        let branch = git::branch();
        // TODO: remove prebuilt-binaries branch!
        if branch == "master" || branch == "prebuilt-binaries" {
            azure::copy_binaries(&config, &staging_directory).expect("COPYING BINARIES FAILED")
        } else {
            azure::prepare_binaries(None, &staging_directory)
                .expect("PREPARING EMPTY BINARIES FAILED");
        }
    }
}

/// Returns the key if we should try to download binaries.
fn should_try_download_binaries(config: &Configuration) -> Option<String> {
    // currently we download binaries only on azure:
    if azure::is_active() {
        // and if we can resolve the hash and the key
        let hash_short = git::hash_short()?;
        Some(binaries::key(&hash_short, &config.features))
    } else {
        None
    }
}

fn download_and_install(key: &str, output_directory: &Path) -> io::Result<()> {
    binaries::download(key, output_directory)?;
    // TODO: verify key?
    // install bindings.rs
    fs::copy(output_directory.join("bindings.rs"), SRC_BINDINGS_RS)?;

    Ok(())
}

mod azure {
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
        let hash_short = git::hash_short().expect("failed to retrieve the git short hash");
        let key = binaries::key(&hash_short, &config.features);

        let binaries = prepare_binaries(Some(&key), artifacts)?;

        fs::copy(SRC_BINDINGS_RS, binaries.join("bindings.rs"))?;

        let libraries = &config.link_libraries;

        let target_is_windows = cargo::target().system == "windows";
        let (skia_lib, skia_bindings_lib) = if target_is_windows {
            ("skia.lib", "skia-bindings.lib")
        } else {
            ("libskia.a", "libskia-bindings.a")
        };

        fs::copy(libraries.join(skia_lib), binaries.join(skia_lib))?;
        fs::copy(
            libraries.join(skia_bindings_lib),
            binaries.join(skia_bindings_lib),
        )?;

        Ok(())
    }

    /// Prepares the binaries directory and sets the key.txt file to the key given.
    /// If no key is available, creates an empty key.txt file to inform azure
    /// that binaries should not be published.
    pub fn prepare_binaries(key: Option<&str>, artifacts: &Path) -> io::Result<PathBuf> {
        let binaries = artifacts.join("skia-binaries");
        fs::create_dir_all(&binaries)?;

        {
            // this is primarily for azure to know the key, but it can stay inside the
            // archive.
            let mut key_file = File::create(binaries.join("key.txt")).unwrap();
            if let Some(key) = key {
                key_file.write_all(key.as_bytes())?;
            }
        }

        Ok(binaries)
    }
}
