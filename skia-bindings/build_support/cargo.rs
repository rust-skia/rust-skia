//! Support function for communicating with cargo's variables and outputs.

use std::fmt::{Display, Formatter};
use std::{env, fmt};

pub fn add_dependent_path(path: &str) {
    println!("cargo:rerun-if-changed={}", path);
}

pub fn add_link_libs<Lib: AsRef<str>>(libs: &[Lib]) {
    libs.into_iter().for_each(|s| add_link_lib(s.as_ref()))
}

pub fn add_link_lib(lib: &str) {
    println!("cargo:rustc-link-lib={}", lib);
}

#[derive(Clone, Debug)]
pub struct Target {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl Target {
    pub fn as_str(&self) -> (&str, &str, &str, Option<&str>) {
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

    Target {
        architecture: target[0].clone(),
        vendor: target[1].clone(),
        system: target[2].clone(),
        abi: if target.len() > 3 {
            Some(target[3].clone())
        } else {
            None
        },
    }
}

// We can not assume that the build profile of the build.rs script reflects the build
// profile that the target needs.
pub fn build_release() -> bool {
    match env::var("PROFILE").unwrap().as_str() {
        "release" => true,
        "debug" => false,
        _ => panic!("PROFILE '{}' is not supported by this build script",),
    }
}
