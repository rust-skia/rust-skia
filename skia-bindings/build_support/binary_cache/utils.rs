use std::io;
use std::io::Read;

/// Download a file from the given URL and return the data.
pub fn download(url: impl AsRef<str>) -> io::Result<Vec<u8>> {
    let resp = ureq::get(url.as_ref()).call();
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
