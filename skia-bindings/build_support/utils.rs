use curl::easy::Easy;
use std::io;

/// Download a file from the given URL and return the data.
pub fn download(url: impl AsRef<str>) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url.as_ref())?;
    handle.fail_on_error(true)?;
    handle.follow_location(true)?;
    let curl_result = {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform()
    };
    match curl_result {
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        Ok(()) => Ok(data),
    }
}
