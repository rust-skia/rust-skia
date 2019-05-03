//! Support for building and deploying prebuilt binaries.

use crate::build_support::{cargo, git};

/// Key generation function. The resulting string will uniquely identify the generated binaries.
/// Parts of the key are separated by '-' and every part that contains individual separators is enclosed
/// in '[]'.
fn key(features: &[&str]) -> String {
    let mut components = Vec::new();

    // SHA hash of the rust-skia repository.
    components.push(git::hash_short());

    // The target architecture, vendor, system, and abi if specified.
    components.push(format!("[{}]", cargo::target().to_string()));

    // features, sorted and duplicates removed.
    if !features.is_empty() {
        let features: String = {
            let mut features = features.to_vec();
            features.sort();
            features.dedup();
            features.join(",")
        };

        components.push(format!("[{}]", features));
    };

    components.join("-")
}
