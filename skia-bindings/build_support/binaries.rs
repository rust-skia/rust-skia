//! Support for building and deploying prebuilt binaries.

use crate::build_support::cargo;
use flate2::read::GzDecoder;
use std::io;
use std::path::Path;
use tar::Archive;

/// Key generation function.
/// The resulting string will uniquely identify the generated binaries.
/// Every part of the key is separated by '-' and no grouping / enclosing characters are used
/// because GitHub strips them from the filenames (tested "<>[]{}()" ).
/// TODO: May use Unicode characters for grouping.
pub fn key<F: AsRef<str>>(repository_short_hash: &str, features: &[F]) -> String {
    let mut components = Vec::new();

    fn group(str: impl AsRef<str>) -> String {
        // no grouping syntax ATM
        format!("{}", str.as_ref())
    }

    // SHA hash of the rust-skia repository.
    components.push(repository_short_hash.to_owned());

    // The target architecture, vendor, system, and abi if specified.
    components.push(group(cargo::target().to_string()));

    // features, sorted and duplicates removed.
    if !features.is_empty() {
        let features: String = {
            let mut features: Vec<String> =
                features.iter().map(|f| f.as_ref().to_string()).collect();
            features.sort();
            features.dedup();
            features.join("-")
        };

        components.push(group(features));
    };

    components.join("-")
}

/// Download the binaries and unpack the contents to the target directory.
/// Returns true if everything went as expected.
pub fn download(key: &str, target: &Path) -> io::Result<()> {
    let tag = cargo::package_version();

    let url = &format!(
        "https://github.com/rust-skia/skia-binaries/releases/download/{}/skia-binaries-{}.tar.gz",
        tag, key
    );
    let body = reqwest::get(url).unwrap();
    let tar = GzDecoder::new(body);
    Archive::new(tar).unpack(target)
}
