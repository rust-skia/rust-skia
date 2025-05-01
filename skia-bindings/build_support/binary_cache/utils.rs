use std::{
    env,
    fs::{self, File},
    io::{self, Error, ErrorKind, Read},
    path::{Path, PathBuf},
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
    // Specify the directory where the downloaded files are stored.
    let mut file_path = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
    // push file name to the path
    file_path.push(url.split('/').next_back().unwrap_or_default());
    let resp = std::process::Command::new("curl")
        // follow redirects
        .arg("-L")
        // fail fast with no "error pages" output. more of a hint though, so we might still get error on stdout.
        // so make sure to check the actual status returned.
        .arg("-f")
        // no progress meter but keep error messages.
        .arg("--no-progress-meter")
        // resumed transfer offset
        .arg("-C")
        .arg("-")
        // directory to save files in
        .arg("--output")
        .arg(file_path.to_str().unwrap())
        // file url
        .arg(url)
        .output();
    match resp {
        Ok(out) => {
            // read bytes from the file
            if out.status.success() {
                let mut file = File::open(file_path)?;
                let mut result = Vec::<u8>::with_capacity(file.metadata()?.len() as usize);
                file.read_to_end(&mut result)?;
                Ok(result)
            } else {
                Err(io::Error::other(format!(
                    "curl error code: {:?}\ncurl stderr: {:?}",
                    out.status
                        .code()
                        .map(|i| i.to_string())
                        .unwrap_or(String::from("no status code")),
                    std::str::from_utf8(&out.stderr).unwrap_or("no stderr")
                )))
            }
        }
        Err(e) => Err(io::Error::other(format!("curl command error : {e:#?}"))),
    }
}
