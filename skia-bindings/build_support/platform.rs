use std::collections::{HashMap, HashSet};

use self::{
    alpine::Alpine,
    android::Android,
    generic::Generic,
    ios::Ios,
    macos::MacOS,
    prelude::quote,
    wasm::Emscripten,
    windows::{WindowsGeneric, WindowsMsvc},
};
use super::{cargo::Platform, clang, skia::BuildConfiguration};

pub mod alpine;
pub mod android;
mod generic;
pub mod ios;
pub mod macos;
pub mod wasm;
mod windows;

/// All details that can be resolved statically for a platform target. From the environment, and
/// current build configurations.
pub trait TargetDetails {
    /// Additional Skia GN arguments.
    fn args(&self, config: &BuildConfiguration, builder: &mut ArgBuilder);
}

impl BuildConfiguration {
    pub fn target_details(&self) -> Box<dyn TargetDetails> {
        match self.target.as_strs() {
            ("wasm32", "unknown", "emscripten", _) => Box::new(Emscripten),
            (_, "linux", "android", _) | (_, "linux", "androideabi", _) => Box::new(Android),
            (_, "apple", "darwin", _) => Box::new(MacOS),
            (_, "apple", "ios", _) => Box::new(Ios),
            (_, _, "windows", Some("msvc")) if self.on_windows => Box::new(WindowsMsvc),
            (_, _, "windows", _) => Box::new(WindowsGeneric),
            (_, "unknown", "linux", Some("musl")) => Box::new(Alpine),
            _ => Box::new(Generic),
        }
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
    pub use super::{ArgBuilder, TargetDetails};
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
