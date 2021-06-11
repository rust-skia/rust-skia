mod build_support;
use build_support::{cargo, skia};

/// Environment variables used by this build script.
mod env {
    use crate::build_support::cargo;
    use std::path::PathBuf;

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

    /// The path to the Skia source directory.
    pub fn source_dir() -> Option<PathBuf> {
        cargo::env_var("SKIA_SOURCE_DIR").map(PathBuf::from)
    }
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

    let build_config = skia::BuildConfiguration::default();
    let binaries_config = skia::BinariesConfiguration::from_cargo_env(&build_config);

    //
    // skip attempting to download?
    //
    if let Some(source_dir) = env::source_dir() {
        println!("STARTING OFFLINE BUILD");

        let final_configuration = skia::FinalBuildConfiguration::from_build_configuration(
            &build_config,
            env::use_system_libraries(),
            &source_dir,
        );

        skia::build(
            &final_configuration,
            &binaries_config,
            env::ninja_command(),
            env::gn_command(),
            true,
        );
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
            let final_configuration = skia::FinalBuildConfiguration::from_build_configuration(
                &build_config,
                env::use_system_libraries(),
                &std::env::current_dir().unwrap().join("skia"),
            );
            skia::build(
                &final_configuration,
                &binaries_config,
                env::ninja_command(),
                env::gn_command(),
                false,
            );
        }
    };

    binaries_config.commit_to_cargo();

    #[cfg(feature = "binary-cache")]
    if let Some(staging_directory) = build_support::binary_cache::should_export() {
        build_support::binary_cache::publish(&binaries_config, &*staging_directory);
    }
}
