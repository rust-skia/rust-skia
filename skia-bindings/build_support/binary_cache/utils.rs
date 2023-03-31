use std::{
    fs,
    io::{self, Error, ErrorKind, Read},
    path::Path,
};

/// Download a file from the given URL and return the data.
pub fn download(url: impl AsRef<str>) -> io::Result<Vec<u8>> {
    let url = url.as_ref();

    // `file` URL, empty hostname, absolute path
    if let Some(file_path) = url.strip_prefix("file://") {
        return fs::read(Path::new(file_path));
    }

    // `file` URLs with non-empty hostname or relative paths are unsupported.
    if url.starts_with("file:") {
        eprintln!("Unsupported file: URL {}", url);
        return Err(Error::from(ErrorKind::Unsupported));
    }

    let resp = ureq::get(url).call();
    match resp {
        Ok(resp) => {
            let mut reader = resp.into_reader();
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;
            Ok(data)
        }
        Err(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
    }
}
