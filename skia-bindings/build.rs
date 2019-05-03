extern crate bindgen;
extern crate cc;

mod build_support;
use build_support::skia;

fn main() {
    let configuration = skia::Configuration::from_cargo_env();
    skia::build(&configuration);
    configuration.commit_to_cargo();

    // deliver binaries to azure's staging directory?
    if let Some(staging_directory) = azure::artifact_staging_directory() {
        println!("DETECTED AZURE, delivering binaries to {}", staging_directory.to_str().unwrap());
        azure::deliver_binaries(&configuration, &staging_directory);
    }
}

mod azure {
    use crate::build_support::skia::Configuration;
    use std::path::{PathBuf, Path};
    use std::{env, fs};
    use crate::build_support::{git, binaries, cargo};
    use std::fs::File;
    use std::io::Write;

    pub fn artifact_staging_directory() -> Option<PathBuf> {
        env::var("BUILD_ARTIFACTSTAGINGDIRECTORY").map(|dir| PathBuf::from(dir)).ok()
    }

    pub fn deliver_binaries(config: &Configuration, artifacts: &Path) {
        let hash_short = git::hash_short();
        let key = binaries::key(&hash_short, &config.features);

        let binaries = artifacts.join("skia-binaries");

        {
            // this is primarily for azure to know the key, but it can stay inside the
            // archive.
            let mut key_file = File::create(binaries.join("key.txt")).unwrap();
            key_file.write_all(key.as_bytes());
        }

        fs::copy("src/bindings.rs", binaries.join("bindings.rs"));

        let libraries = &config.link_libraries;

        let target_is_windows = cargo::target().system == "windows";
        let (skia_lib, skia_bindings_lib) =
            if target_is_windows {
                ("skia.lib", "skia-bindings.lib")
            } else {
                ("libskia.a", "libskia-bindings.a")
            };

        fs::copy(libraries.join(skia_lib), binaries.join(skia_lib));
        fs::copy(libraries.join(skia_bindings_lib), binaries.join(skia_bindings_lib));
    }
}
