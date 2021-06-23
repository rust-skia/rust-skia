mod build_support;
use build_support::{binaries_config, bind_skia, cargo, features};

#[cfg(feature = "build-from-source")]
use build_support::build_skia;

/// Environment variables used by this build script.
mod env {
    use crate::build_support::cargo;
    use std::path::PathBuf;

    /// The path to the Skia source directory.
    pub fn source_dir() -> Option<PathBuf> {
        cargo::env_var("SKIA_SOURCE_DIR").map(PathBuf::from)
    }

    /// The path to where a pre-built Skia library can be found.
    pub fn skia_lib_search_path() -> Option<PathBuf> {
        cargo::env_var("SKIA_LIBRARY_SEARCH_PATH").map(PathBuf::from)
    }

    pub fn is_skia_debug() -> bool {
        matches!(cargo::env_var("SKIA_DEBUG"), Some(v) if v != "0")
    }
}

#[cfg(feature = "build-from-source")]
fn build_from_source(
    features: features::Features,
    binaries_config: &binaries_config::BinariesConfiguration,
    skia_source_dir: &std::path::Path,
    skia_debug: bool,
    offline: bool,
) {
    let build_config = build_skia::BuildConfiguration::from_features(features, skia_debug);
    let final_configuration = build_skia::FinalBuildConfiguration::from_build_configuration(
        &build_config,
        build_skia::env::use_system_libraries(),
        &skia_source_dir,
    );

    build_skia::build(
        &final_configuration,
        binaries_config,
        build_skia::env::ninja_command(),
        build_skia::env::gn_command(),
        offline,
    );
}

fn generate_bindings(
    features: &features::Features,
    definitions: Vec<bind_skia::Definition>,
    binaries_config: &binaries_config::BinariesConfiguration,
    skia_source_dir: &std::path::Path,
) {
    // Emit the ninja definitions, to aid build consistency.
    println!("ninja definitions:");
    for (name, value) in definitions.iter() {
        if let Some(value) = value {
            println!("  -D{}={}", name, value);
        } else {
            println!("  -D{}", name);
        }
    }
    println!();

    let bindings_config = bind_skia::FinalBuildConfiguration::from_build_configuration(
        features,
        definitions,
        skia_source_dir,
    );
    bind_skia::generate_bindings(&bindings_config, &binaries_config.output_directory);
}

fn main() {
    // since 0.25.0
    if cfg!(feature = "svg") {
        cargo::warning("The feature 'svg' has been removed. SVG and XML support is available in all build configurations.");
    }
    // since 0.25.0
    if cfg!(feature = "shaper") {
        cargo::warning("The feature 'shaper' has been removed. To use the SkShaper bindings, enable the feature 'textlayout'.");
    }

    let skia_debug = env::is_skia_debug();
    let features = features::Features::default();
    let binaries_config =
        binaries_config::BinariesConfiguration::from_features(&features, skia_debug);

    //
    // skip attempting to download?
    //
    if let Some(source_dir) = env::source_dir() {
        if let Some(search_path) = env::skia_lib_search_path() {
            println!("STARTING BIND AGAINST SYSTEM SKIA");

            cargo::add_link_search(&search_path.to_str().unwrap());

            let definitions = bind_skia::definitions::from_env();
            generate_bindings(&features, definitions, &binaries_config, &source_dir);
        } else {
            println!("STARTING OFFLINE BUILD");

            assert!(cfg!(feature = "build-from-source"));

            #[cfg(feature = "build-from-source")]
            {
                let definitions = bind_skia::definitions::from_ninja_features(
                    &features,
                    &binaries_config.output_directory,
                );
                build_from_source(
                    features.clone(),
                    &binaries_config,
                    &source_dir,
                    skia_debug,
                    true,
                );
                generate_bindings(&features, definitions, &binaries_config, &source_dir);
            }
        }
    } else {
        //
        // is the download of prebuilt binaries possible?
        //

        #[allow(unused_variables)]
        let build_skia = true;

        #[cfg(feature = "binary-cache")]
        let build_skia = build_support::binary_cache::try_prepare_download(&binaries_config);

        //
        // full build?
        //

        if build_skia {
            println!("STARTING A FULL BUILD");

            assert!(cfg!(feature = "build-from-source"));

            #[cfg(feature = "build-from-source")]
            {
                let source_dir = std::env::current_dir().unwrap().join("skia");
                let definitions = bind_skia::definitions::from_ninja_features(
                    &features,
                    &binaries_config.output_directory,
                );
                build_from_source(
                    features.clone(),
                    &binaries_config,
                    &source_dir,
                    skia_debug,
                    true,
                );
                generate_bindings(&features, definitions, &binaries_config, &source_dir);
            }
        }
    };

    binaries_config.commit_to_cargo();

    #[cfg(feature = "binary-cache")]
    if let Some(staging_directory) = build_support::binary_cache::should_export() {
        build_support::binary_cache::publish(&binaries_config, &*staging_directory);
    }
}
