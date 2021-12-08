//! Additional VS detection support for Skia.
//! TODO: sophisticate this: https://github.com/alexcrichton/cc-rs/blob/master/src/windows_registry.rs

use crate::build_support::cargo;
use std::path::PathBuf;

pub fn resolve_win_vc() -> Option<PathBuf> {
    if let Some(install_dir) = cargo::env_var("VCINSTALLDIR") {
        return Some(PathBuf::from(install_dir));
    }

    let releases = [("Program Files", "2022"), ("Program Files (x86)", "2019")];
    let editions = ["BuildTools", "Enterprise", "Professional", "Community"];

    releases
        .iter()
        .flat_map(|r| editions.iter().map(move |e| (r, e)))
        .map(|((rp, r), ed)| format!("C:\\{}\\Microsoft Visual Studio\\{}\\{}\\VC", rp, r, ed))
        .map(PathBuf::from)
        .find(|pb| pb.exists())
}
