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
            (_, _, "windows", Some("msvc")) if self.on_windows => Box::new(WindowsMsvc),
            (_, _, "windows", _) => Box::new(WindowsGeneric),
            (_, _, "darwin", _) => Box::new(MacOS),
            (_, _, "linux", Some("musl")) => Box::new(Alpine),
            (_, _, "ios", _) => Box::new(Ios),
            _ => Box::new(Generic),
        }
    }
}

#[derive(Debug)]
pub struct ArgBuilder {
    config_target: Platform,
    target: Option<String>,
    args: HashMap<String, String>,
    cflags: HashSet<String>,
    asmflags: HashSet<String>,
}

impl ArgBuilder {
    pub fn new(config: &BuildConfiguration) -> Self {
        Self {
            config_target: config.target.clone(),
            target: Some(config.target.to_string()),
            args: HashMap::default(),
            cflags: HashSet::default(),
            asmflags: HashSet::default(),
        }
    }

    /// Overwrite the default target.
    pub fn target(&mut self, target: impl Into<Option<String>>) -> &mut Self {
        self.target = target.into();
        self
    }

    /// Set a Skia GN arg.
    pub fn arg(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.args.insert(name.into(), value.into());
        self
    }

    /// Set a Skia c flag.
    pub fn cflag(&mut self, flag: impl Into<String>) -> &mut Self {
        self.cflags.insert(flag.into());
        self
    }

    /// Set multiple Skia c flags.
    pub fn cflags(&mut self, flags: impl IntoIterator<Item = String>) -> &mut Self {
        flags.into_iter().for_each(|s| {
            self.cflag(s);
        });
        self
    }

    /// Explicitly set `target_os` to the value and `target_cpu` to clang's default. By default,
    /// none of them are set.
    pub fn skia_target_os_and_default_cpu(&mut self, os: impl Into<String>) -> &mut Self {
        self.arg("target_os", quote(&os.into()));
        self.arg(
            "target_cpu",
            quote(clang::target_arch(&self.config_target.architecture)),
        )
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
