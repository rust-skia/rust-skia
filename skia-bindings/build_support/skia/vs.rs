//! Additional VS detection support for Skia.
//! TODO: sophisticate this: https://github.com/alexcrichton/cc-rs/blob/master/src/windows_registry.rs

use crate::build_support::cargo;
use std::path::PathBuf;

pub fn resolve_win_vc() -> Option<PathBuf> {
    if let Some(install_dir) = cargo::env_var("VCINSTALLDIR") {
        return Some(PathBuf::from(install_dir));
    }

    [
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC",
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC",
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional\\VC",
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community\\VC",
    ]
    .iter()
    .map(PathBuf::from)
    .find(|pb| pb.exists())
}
