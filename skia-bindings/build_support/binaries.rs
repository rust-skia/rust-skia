//! Support for building and deploying prebuilt binaries.

use crate::build_support::cargo;
use flate2::read::GzDecoder;
use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;

/// The name of the tar archive without any keys or file extensions. This is also the name
/// of the subdirectory that is created when the archive is unpacked.
pub const ARCHIVE_NAME: &str = "skia-binaries";

/// Key generation function.
/// The resulting string will uniquely identify the generated binaries.
/// Every part of the key is separated by '-' and no grouping / enclosing characters are used
/// because GitHub strips them from the filenames (tested "<>[]{}()",
/// and also Unicode characters seem to be stripped).
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
        tag.as_ref(),
        ARCHIVE_NAME,
        key.as_ref()
    )
}

/// Begin downloading the binaries from the given url.
pub fn begin_download(url: impl AsRef<str>) -> io::Result<impl Read> {
    match reqwest::get(url.as_ref()) {
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        Ok(response) => response
            .error_for_status()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
    }
}

pub fn unpack(archive: impl Read, target: &Path) -> io::Result<()> {
    let tar = GzDecoder::new(archive);
    // note: this creates the skia-bindings/ directory.
    Archive::new(tar).unpack(target)?;
    let binaries_dir = target.join(ARCHIVE_NAME);
    let paths: Vec<PathBuf> = fs::read_dir(binaries_dir)?
        .map(|e| e.unwrap().path())
        .collect();

    // pull out all nested files.
    for path in paths {
        let name = path.file_name().unwrap();
        let target_path = target.join(name);
        fs::rename(path, target_path)?
    }
    Ok(())
}
