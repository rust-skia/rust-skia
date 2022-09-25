use super::prelude::*;
use crate::build_support::{ios, skia::BuildConfiguration};

pub fn args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    let (arch, abi) = config.target.arch_abi();

    builder.skia_target_os_and_default_cpu("ios");

    // m100: Needed for aarch64 simulators, requires cherry Skia pick
    // 0361abf39d1504966799b1cdb5450e07f88b2bc2 (until milestone 102).
    if ios::is_simulator(arch, abi) {
        builder.skia("ios_use_simulator", yes());
    }

    builder.skia_cflags(ios::extra_skia_cflags(arch, abi));

    if let Some(specific_target) = ios::specific_target(arch, abi) {
        builder.target(specific_target);
    }

    builder.clang_args(ios::additional_clang_args(&config.target.to_string(), abi))
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    let abi = cargo::target().abi;
    builder.link_libraries(ios::link_libraries(abi.as_deref(), features))
}
