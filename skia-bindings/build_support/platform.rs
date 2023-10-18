//! All the configuration settings that can be resolved statically for a platform target. From the
//! environment, and current build configurations.

use self::prelude::quote;
use super::{
    cargo::{self, Target},
    clang,
    features::Features,
    skia::BuildConfiguration,
};

pub mod alpine;
pub mod android;
pub mod emscripten;
mod generic;
pub mod ios;
pub mod linux;
pub mod macos;
mod windows;

pub fn gn_args(config: &BuildConfiguration, mut builder: GnArgsBuilder) -> Vec<(String, String)> {
    details(&config.target).gn_args(config, &mut builder);
    builder.into_gn_args()
}

pub fn bindgen_and_cc_args(target: &Target, sysroot: Option<&str>) -> (Vec<String>, Vec<String>) {
    let mut builder = BindgenArgsBuilder::new(sysroot);
    details(target).bindgen_args(target, &mut builder);
    builder.into_bindgen_and_cc_args()
}

pub fn link_libraries(features: &Features, target: &Target) -> Vec<String> {
    details(target).link_libraries(features)
}

pub trait PlatformDetails {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder);
    fn bindgen_args(&self, _target: &Target, _builder: &mut BindgenArgsBuilder) {}
    fn link_libraries(&self, features: &Features) -> Vec<String>;
}

#[allow(clippy::type_complexity)]
fn details(target: &Target) -> Box<dyn PlatformDetails> {
    let host = cargo::host();
    match target.as_strs() {
        ("wasm32", "unknown", "emscripten", _) => Box::new(emscripten::Emscripten),
        (_, "linux", "android", _) | (_, "linux", "androideabi", _) => Box::new(android::Android),
        (_, "apple", "darwin", _) => Box::new(macos::MacOs),
        (_, "apple", "ios", _) => Box::new(ios::Ios),
        (_, _, "windows", Some("msvc")) if host.is_windows() => Box::new(windows::Msvc),
        (_, _, "windows", _) => Box::new(windows::Generic),
        (_, "unknown", "linux", Some("musl")) => Box::new(alpine::Musl),
        (_, _, "linux", _) => Box::new(linux::Linux),
        _ => Box::new(generic::Generic),
    }
}

#[derive(Debug)]
pub struct GnArgsBuilder {
    target_arch: String,
    use_system_libraries: bool,
    target_str: Option<String>,
    gn_args: Vec<(String, String)>,
    skia_cflags: Vec<String>,
}

impl GnArgsBuilder {
    pub fn new(target: &Target, use_system_libraries: bool) -> Self {
        Self {
            target_arch: target.architecture.clone(),
            use_system_libraries,
            target_str: Some(target.to_string()),
            gn_args: Vec::default(),
            skia_cflags: Vec::default(),
        }
    }

    pub fn use_system_libraries(&self) -> bool {
        self.use_system_libraries
    }

    /// Overwrite the default target.
    pub fn target(&mut self, target: impl Into<Option<String>>) {
        self.target_str = target.into();
    }

    /// Set a Skia GN arg.
    pub fn arg(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.gn_args.push((name.into(), value.into()));
        self
    }

    /// Set a Skia C flag.
    pub fn cflag(&mut self, flag: impl Into<String>) -> &mut Self {
        self.skia_cflags.push(flag.into());
        self
    }

    /// Set multiple Skia C flags.
    pub fn cflags(&mut self, flags: impl IntoIterator<Item = String>) {
        flags.into_iter().for_each(|s| {
            self.cflag(s);
        });
    }

    /// Explicitly set `target_os` to the value and `target_cpu` to clang's default. By default,
    /// none of them are set.
    pub fn target_os_and_default_cpu(&mut self, os: impl Into<String>) {
        self.arg("target_os", quote(&os.into()));
        self.arg("target_cpu", quote(clang::target_arch(&self.target_arch)));
    }

    pub fn into_gn_args(mut self) -> Vec<(String, String)> {
        let mut asmflags = Vec::new();

        if let Some(target) = &self.target_str {
            let target = format!("--target={target}");
            self.cflag(&target);
            asmflags.push(target);
        }

        if !self.skia_cflags.is_empty() {
            let cflags = format!(
                "[{}]",
                self.skia_cflags
                    .iter()
                    .map(|s| quote(s))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            self.arg("extra_cflags", cflags);
        }

        if !asmflags.is_empty() {
            let asmflags = format!(
                "[{}]",
                asmflags
                    .iter()
                    .map(|s| quote(s))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            self.arg("extra_asmflags", asmflags);
        }

        self.gn_args.into_iter().collect()
    }
}

#[derive(Debug)]
pub struct BindgenArgsBuilder {
    /// sysroot if set explicitly.
    sysroot: Option<String>,
    sysroot_prefix: String,
    bindgen_clang_args: Vec<String>,
}

impl BindgenArgsBuilder {
    pub fn new(sysroot: Option<&str>) -> Self {
        Self {
            sysroot: sysroot.map(|s| s.into()),
            sysroot_prefix: "--sysroot=".into(),
            bindgen_clang_args: Vec::new(),
        }
    }

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
    pub fn arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.bindgen_clang_args.push(arg.into());
        self
    }

    /// Set multiple Bindgen Clang arguments.
    pub fn args(&mut self, arguments: impl IntoIterator<Item = String>) {
        arguments.into_iter().for_each(|s| {
            self.arg(s);
        });
    }

    pub fn into_bindgen_and_cc_args(mut self) -> (Vec<String>, Vec<String>) {
        let mut cc_build_args = Vec::new();

        if let Some(sysroot) = &self.sysroot {
            let sysroot_arg = format!("{}{}", self.sysroot_prefix, sysroot);
            self.arg(&sysroot_arg);
            cc_build_args.push(sysroot_arg);
        }

        (self.bindgen_clang_args.into_iter().collect(), cc_build_args)
    }
}

pub mod prelude {
    pub use self::{cargo::Target, skia::BuildConfiguration};
    pub use super::{BindgenArgsBuilder, GnArgsBuilder, PlatformDetails};
    pub use crate::build_support::{cargo, clang, features::Features, skia};

    pub fn quote(s: &str) -> String {
        format!("\"{s}\"")
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
