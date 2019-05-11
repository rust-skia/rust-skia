mod build_support;
use crate::build_support::skia::Configuration;
use crate::build_support::{azure, binaries, cargo, git};
use build_support::skia;
use std::path::Path;
use std::{env, fs, io};

const SRC_BINDINGS_RS: &str = "src/bindings.rs";

fn main() {
    let config = skia::Configuration::from_cargo_env();

    //
    // download of prebuilt binaries possible?
    //

    let mut do_full_build = true;

    if let Some((tag, key)) = should_try_download_binaries(&config) {
        println!(
            "TRYING TO DOWNLOAD AND INSTALL SKIA BINARIES: {}/{}",
            tag, key
        );
        let url = binaries::download_url(tag, key);
        println!("  FROM: {}", url);
        if let Err(e) = download_and_install(url, &config.output_directory) {
            println!("DOWNLOAD AND INSTALL FAILED: {}", e);
        } else {
            println!("DOWNLOAD AND INSTALL SUCCEEDED");
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

        println!("COPYING BINARIES");
        azure::copy_binaries(&config, &staging_directory).expect("COPYING BINARIES FAILED")
    }
}

/// Returns the key if we should try to download binaries.
fn should_try_download_binaries(config: &Configuration) -> Option<(String, String)> {
    let tag = cargo::package_version();

    // for testing:
    if let Ok(_) = env::var("FORCE_SKIA_BINARIES_DOWNLOAD") {
        // retrieve the hash from the repository above us.
        let half_hash = git::half_hash()?;
        return Some((tag, binaries::key(&half_hash, &config.features)));
    }

    // are we building inside a package?
    if let Ok(ref full_hash) = cargo::package_repository_hash() {
        let half_hash = git::trim_hash(full_hash);
        return Some((tag, binaries::key(&half_hash, &config.features)));
    }

    if azure::is_active() {
        // and if we can resolve the hash and the key
        let half_hash = git::half_hash()?;
        return Some((tag, binaries::key(&half_hash, &config.features)));
    }

    None
}

fn download_and_install(url: impl AsRef<str>, output_directory: &Path) -> io::Result<()> {
    let archive = binaries::download(url)?;
    println!(
        "UNPACKING ARCHIVE INTO: {}",
        output_directory.to_str().unwrap()
    );
    binaries::unpack(archive, output_directory)?;
    // TODO: verify key?
    println!("INSTALLING BINDINGS");
    fs::copy(output_directory.join("bindings.rs"), SRC_BINDINGS_RS)?;

    Ok(())
}
