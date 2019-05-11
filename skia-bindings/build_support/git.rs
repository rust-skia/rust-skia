//! git build helper.

use std::path::Path;
use std::process::{Command, Stdio};

const HALF_HASH_LENGTH: usize = 20;

/// Returns a 20 digit hash of the repository located in the current directory.
pub fn half_hash() -> Option<String> {
    let mut cmd = Command::new("git");
    cmd.arg("rev-parse").arg("--short=20");
    let output = cmd.arg("HEAD").stderr(Stdio::inherit()).output().ok()?;
    if output.status.code() != Some(0) {
        None
    } else {
        // need to trim the string to remove newlines at the end.
        Some(String::from_utf8(output.stdout).unwrap().trim().to_string())
    }
}

pub fn trim_hash(hash: &str) -> String {
    hash[..HALF_HASH_LENGTH].into()
}

/// Run git with the given args in the given directory, print stderr to the current
/// process's terminal, and capture its stdout output.
pub fn run<'a, T: AsRef<str>, IOP: Into<Option<&'a Path>>>(args: &[T], dir: IOP) -> Vec<u8> {
    let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    let mut cmd = Command::new("git");
    cmd.args(&args).stderr(Stdio::inherit());

    if let Some(dir) = dir.into() {
        cmd.current_dir(dir);
    }

    let output = cmd.output().expect("running git failed, is it in PATH?");
    if output.status.code() != Some(0) {
        panic!("GIT command failed: git {}", args.join(" "));
    } else {
        output.stdout
    }
}
