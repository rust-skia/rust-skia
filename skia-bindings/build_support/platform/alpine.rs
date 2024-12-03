use super::{linux, prelude::*};
use std::process::{Command, Stdio};

pub struct Musl;

impl PlatformDetails for Musl {
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        linux::gn_args(config, builder);
        let target = &config.target;

        builder.cflags(flags(target));
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(flags(target))
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        linux::link_libraries(features)
    }
}

fn get_stdlib_version() -> Option<String> {
    let mut cmd = Command::new("g++");
    cmd.arg("-dumpversion");
    let output = cmd.stderr(Stdio::inherit()).output().ok()?;
    if output.status.code() != Some(0) {
        return None;
    }
    match String::from_utf8(output.stdout).unwrap().trim() {
        "" => None,
        v => Some(String::from(v)),
    }
}

fn flags(target: &Target) -> Vec<String> {
    let arch = &target.architecture;
    match get_stdlib_version() {
        None => {
            cargo::warning("unable to determine g++ stdlib version");
            vec![]
        }
        Some(cpp) => vec![
            format!("-I/usr/include/c++/{cpp}"),
            format!("-I/usr/include/c++/{cpp}/{arch}-alpine-linux-musl"),
        ],
    }
}
