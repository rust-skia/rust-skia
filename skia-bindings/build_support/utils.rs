use std::io;
use std::io::Read;

/// Download a file from the given URL and return the data.
pub fn download(url: impl AsRef<str>) -> io::Result<Vec<u8>> {
    let resp = ureq::get(url.as_ref()).timeout_connect(10_000).call();
    if let Some(error) = resp.synthetic_error() {
        Err(io::Error::from_raw_os_error(error.status() as i32))
    } else {
        let mut reader = resp.into_reader();
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(data)
    }
}
