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
    let resp = std::process::Command::new("curl")
        // follow redirects
        .arg("-L")
        // fail fast with no "error pages" output. more of a hint though, so we might still get error on stdout.
        // so make sure to check the actual status returned.
        .arg("-f")
        // silent. no progress or error messages. only pure "response data"
        .arg("-s")
        .arg(url)
        .output();
    match resp {
        Ok(out) => {
            // ideally, we would redirect response to a file directly, but lets take it one step at a time.
            let result = out.stdout;
            if out.status.success() {
                Ok(result)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "curl error code: {:?}\ncurl stderr: {:?}",
                        out.status.code(),
                        std::str::from_utf8(&out.stderr)
                    ),
                ))
            }
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("curl command error : {e:#?}"),
        )),
    }
}
