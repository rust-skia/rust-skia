use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Returns the current SDK path.
pub fn get_sdk_path(sdk: impl AsRef<str>) -> Option<PathBuf> {
    let mut cmd = Command::new("xcrun");
    cmd.arg("--sdk").arg(sdk.as_ref()).arg("--show-sdk-path");
    let output = cmd.stderr(Stdio::inherit()).output().ok()?;
    if output.status.code() != Some(0) {
        return None;
    }
    Some({
        let str = String::from_utf8(output.stdout).unwrap();
        PathBuf::from(str.trim())
    })
}
