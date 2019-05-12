//! Support function for communicating with cargo's variables and outputs.

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::{env, fmt, fs, io};

pub fn output_directory() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub fn add_dependent_path(path: impl AsRef<str>) {
    println!("cargo:rerun-if-changed={}", path.as_ref());
}

pub fn add_link_libs(libs: &[impl AsRef<str>]) {
    libs.into_iter().for_each(|s| add_link_lib(s.as_ref()))
}

pub fn add_link_lib(lib: impl AsRef<str>) {
    println!("cargo:rustc-link-lib={}", lib.as_ref());
}

#[derive(Clone, Debug)]
pub struct Target {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl Target {
    pub fn as_strs(&self) -> (&str, &str, &str, Option<&str>) {
        (
            self.architecture.as_str(),
            self.vendor.as_str(),
            self.system.as_str(),
            self.abi.as_ref().map(|s| s.as_str()),
        )
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            &self.architecture, &self.vendor, &self.system
        )?;

        if let Some(ref abi) = self.abi {
            write!(f, "-{}", abi)
        } else {
            Result::Ok(())
        }
    }
}

pub fn target() -> Target {
    let target_str = env::var("TARGET").unwrap();

    let target: Vec<String> = target_str.split("-").map(|s| s.into()).collect();
    if target.len() < 3 {
        panic!("Failed to parse TARGET {}", target_str);
    }

    let abi = if target.len() > 3 {
        Some(target[3].clone())
    } else {
        None
    };

    Target {
        architecture: target[0].clone(),
        vendor: target[1].clone(),
        system: target[2].clone(),
        abi,
    }
}

// We can not assume that the build profile of the build.rs script reflects the build
// profile that the target needs.
#[allow(dead_code)]
pub fn build_release() -> bool {
    match env::var("PROFILE").unwrap().as_str() {
        "release" => true,
        "debug" => false,
        _ => panic!("PROFILE '{}' is not supported by this build script",),
    }
}

// If we are builing from within a packaged crate, return the full commit hash
// of the original repository we were packaged from.
pub fn package_repository_hash() -> io::Result<String> {
    let vcs_info = fs::read_to_string(".cargo_vcs_info.json")?;
    let value: serde_json::Value = serde_json::from_str(&vcs_info)?;
    let git = value.get("git").expect("failed to get 'git' property");
    let sha1 = git.get("sha1").expect("failed to get 'sha1' property");
    Ok(sha1.as_str().unwrap().into())
}

pub fn package_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap().as_str().into()
}
