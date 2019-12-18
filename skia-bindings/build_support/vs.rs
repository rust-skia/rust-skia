//! Additional VS detection support for Skia.
//! TODO: sophisticate this: https://github.com/alexcrichton/cc-rs/blob/master/src/windows_registry.rs

use std::path::PathBuf;

pub fn resolve_win_vc() -> Option<PathBuf> {
    [
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC",
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional\\VC",
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community\\VC",
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC",
    ]
    .iter()
    .map(PathBuf::from)
    .find(|pb| pb.exists())
}
