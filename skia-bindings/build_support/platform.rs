//! All the configuration settings that can be resolved statically for a platform target. From the
//! environment, and current build configurations.

use self::prelude::quote;
use super::{cargo::Platform, clang, skia::BuildConfiguration};
use std::collections::{HashMap, HashSet};

pub mod alpine;
pub mod android;
pub mod emscripten;
mod generic;
pub mod ios;
pub mod macos;
mod windows;

pub fn build_args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    match config.target.as_strs() {
        ("wasm32", "unknown", "emscripten", _) => emscripten::args(config, builder),
        (_, "linux", "android", _) | (_, "linux", "androideabi", _) => {
            android::args(config, builder)
        }
        (_, "apple", "darwin", _) => macos::args(config, builder),
        (_, "apple", "ios", _) => ios::args(config, builder),
        (_, _, "windows", Some("msvc")) if config.on_windows => windows::msvc_args(config, builder),
        (_, _, "windows", _) => windows::generic_args(config, builder),
        (_, "unknown", "linux", Some("musl")) => alpine::musl_args(config, builder),
        _ => generic::args(config, builder),
    }
}

#[derive(Debug)]
pub struct ArgBuilder {
    config_target: Platform,
    target: Option<String>,
    args: HashMap<String, String>,
    skia_cflags: HashSet<String>,
    skia_asmflags: HashSet<String>,

    /// sysroot if set explicitly.
    sysroot: Option<String>,
    sysroot_prefix: String,
    bindgen_clang_args: HashSet<String>,

    link_libraries: Vec<String>,
}

impl ArgBuilder {
    pub fn new(config: &BuildConfiguration, sysroot: Option<&str>) -> Self {
        Self {
            config_target: config.target.clone(),
            target: Some(config.target.to_string()),
            args: HashMap::default(),
            skia_cflags: HashSet::default(),
            skia_asmflags: HashSet::default(),

            sysroot: sysroot.map(|s| s.into()),
            sysroot_prefix: "--sysroot=".into(),
            bindgen_clang_args: HashSet::default(),

            link_libraries: Vec::new(),
        }
    }

    /// Overwrite the default target.
    pub fn target(&mut self, target: impl Into<Option<String>>) {
        self.target = target.into();
    }

    /// Set a Skia GN arg.
    pub fn skia(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.args.insert(name.into(), value.into());
        self
    }

    /// Set a Skia C flag.
    pub fn skia_cflag(&mut self, flag: impl Into<String>) -> &mut Self {
        self.skia_cflags.insert(flag.into());
        self
    }

    /// Set multiple Skia C flags.
    pub fn skia_cflags(&mut self, flags: impl IntoIterator<Item = String>) {
        flags.into_iter().for_each(|s| {
            self.skia_cflag(s);
        });
    }

    /// Explicitly set `target_os` to the value and `target_cpu` to clang's default. By default,
    /// none of them are set.
    pub fn skia_target_os_and_default_cpu(&mut self, os: impl Into<String>) {
        self.skia("target_os", quote(&os.into()));
        self.skia(
            "target_cpu",
            quote(clang::target_arch(&self.config_target.architecture)),
        );
    }

    /// Is the sysroot set explicitly?
    pub fn sysroot(&self) -> Option<&str> {
        self.sysroot.as_deref()
    }

    /// Set the sysroot.
    pub fn set_sysroot(&mut self, sysroot: impl Into<String>) {
        self.sysroot = Some(sysroot.into())
    }

    /// If a sysroot is set, we use the default prefix `--sysroot=` for setting it, but some
    /// platforms may object.
    pub fn sysroot_prefix(&mut self, prefix: impl Into<String>) {
        self.sysroot_prefix = prefix.into();
    }

    /// Set a Bindgen Clang arg.
    pub fn clang_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.bindgen_clang_args.insert(arg.into());
        self
    }

    /// Set multiple Bindgen Clang arguments.
    pub fn clang_args(&mut self, arguments: impl IntoIterator<Item = String>) {
        arguments.into_iter().for_each(|s| {
            self.clang_arg(s);
        });
    }
}

pub mod prelude {
    pub use self::skia::BuildConfiguration;
    pub use super::ArgBuilder;
    pub use crate::build_support::{cargo, clang, skia};

    pub fn quote(s: &str) -> String {
        format!("\"{}\"", s)
    }

    pub fn yes() -> String {
        "true".into()
    }

    pub fn no() -> String {
        "false".into()
    }

    pub fn yes_if(y: bool) -> String {
        if y {
            yes()
        } else {
            no()
        }
    }
}
