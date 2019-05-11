//! Support for building and deploying prebuilt binaries.

use crate::build_support::cargo;
use flate2::read::GzDecoder;
use std::io;
use std::path::{Path, PathBuf};
use tar::Archive;
use std::io::Read;
use std::fs;

/// The name of the tar archive without any keys or file extensions. This is also the name
/// of the subdirectory that is created when the archive is unpacked.
pub const ARCHIVE_NAME: &str = "skia-binaries";

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

/// Create the download URL for the prebuilt binaries archive.
pub fn download_url(tag: impl AsRef<str>, key: impl AsRef<str>) -> String {
    format!(
        "https://github.com/rust-skia/skia-binaries/releases/download/{}/{}-{}.tar.gz",
        tag.as_ref(), ARCHIVE_NAME, key.as_ref()
    )
}

/// Download the binaries and unpack the contents to the target directory.
/// Returns true if everything went as expected.
pub fn download(url: impl AsRef<str>) -> io::Result<impl Read> {
    match reqwest::get(url.as_ref()) {
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        Ok(response) => Ok(response)
    }
}

pub fn unpack(archive: impl Read, target: &Path) -> io::Result<()> {
    let tar = GzDecoder::new(archive);
    // note: this creates skia-bindings/ directory.
    Archive::new(tar).unpack(target)?;
    let binaries_dir = target.join(ARCHIVE_NAME);
    let paths : Vec<PathBuf> = fs::read_dir(target)?.map(|e| e.unwrap().path()).collect();
    for path in paths {
        fs::rename(path, target)?
    }
    Ok(())
}
