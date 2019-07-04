use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn sdk_path() -> PathBuf {
    let sdk_path = Command::new("xcrun")
        .arg("--show-sdk-path")
        .arg("--sdk")
        .arg("iphoneos")
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to invoke xcrun")
        .stdout;

    let string = String::from_utf8(sdk_path).expect("failed to resolve iOS SDK path");
    PathBuf::from(string.trim())
}
