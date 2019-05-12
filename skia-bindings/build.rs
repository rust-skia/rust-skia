mod build_support;
use build_support::skia;
use build_support::{azure, binaries, cargo, git};
use std::path::Path;
use std::{fs, io};

/// Environment variables used by this build script.
mod env {
    use std::env;

    /// Returns true if the download should be forced, independent of the situation detected.
    pub fn force_skia_binaries_download() -> bool {
        env::var("FORCE_SKIA_BINARIES_DOWNLOAD").is_ok()
    }

    /// Force to build skia.
    pub fn force_skia_build() -> bool {
        env::var("FORCE_SKIA_BUILD").is_ok()
    }
}

const SRC_BINDINGS_RS: &str = "src/bindings.rs";

fn main() {
    let build_config = skia::BuildConfiguration::default();
    let binaries_config = skia::BinariesConfiguration::from_cargo_env(&build_config);

    //
    // is the download of prebuilt binaries possible?
    //

    let build_skia = env::force_skia_build() || {
        if let Some((tag, key)) = should_try_download_binaries(&binaries_config) {
            println!(
                "TRYING TO DOWNLOAD AND INSTALL SKIA BINARIES: {}/{}",
                tag, key
            );
            let url = binaries::download_url(tag, key);
            println!("  FROM: {}", url);
            if let Err(e) = download_and_install(url, &binaries_config.output_directory) {
                println!("DOWNLOAD AND INSTALL FAILED: {}", e);
                true
            } else {
                println!("DOWNLOAD AND INSTALL SUCCEEDED");
                false
            }
        } else {
            true
        }
    };

    //
    // full build?
    //

    if build_skia {
        println!("STARTING A FULL BUILD");
        skia::build(&build_config, &binaries_config);
    }

    binaries_config.commit_to_cargo();

    //
    // publish binaries?
    //

    // TODO: we may not want to deliver binaries when we downloaded the binaries
    //       but how to inform azure if we don't want to?
    if let Some(staging_directory) = azure::artifact_staging_directory() {
        println!(
            "DETECTED AZURE, delivering binaries to {}",
            staging_directory.to_str().unwrap()
        );

        println!("COPYING BINARIES");
        azure::copy_binaries(&binaries_config, &staging_directory).expect("COPYING BINARIES FAILED")
    }
}

/// Returns the key if we should try to download binaries.
fn should_try_download_binaries(config: &skia::BinariesConfiguration) -> Option<(String, String)> {
    let tag = cargo::package_version();

    // for testing:
    if env::force_skia_binaries_download() {
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
