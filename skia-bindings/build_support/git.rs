//! git build helper.

use std::process::{Command, Stdio};

/// Returns the has of the repository located in the current directory.
pub fn hash_short() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .ok()?;

    if output.status.code() != Some(0) {
        None
    } else {
        Some(String::from_utf8(output.stdout).unwrap())
    }
}

/// Returns the current branch of the repository.
#[allow(dead_code)]
pub fn branch() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("git failed");

    if output.status.code() != Some(0) {
        panic!("git rev-parse failed");
    } else {
        String::from_utf8(output.stdout).unwrap()
    }
}
