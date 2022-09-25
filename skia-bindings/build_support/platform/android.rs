use super::prelude::*;
use crate::build_support::android;

pub fn args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    // TODO: this may belong into BuildConfiguration
    let use_system_libraries = skia::env::use_system_libraries();
    let (arch, _) = config.target.arch_abi();

    builder
        .skia("ndk", quote(&android::ndk()))
        .skia("ndk_api", android::API_LEVEL)
        .skia("target_cpu", quote(clang::target_arch(arch)))
        .skia("skia_enable_fontmgr_android", yes());

    if !config.features.embed_freetype {
        builder.skia("skia_use_system_freetype2", yes_if(use_system_libraries));
    }

    builder.skia_cflags(android::extra_skia_cflags());
    builder.clang_args(android::additional_clang_args(
        &config.target.to_string(),
        arch,
    ));
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    builder.link_libraries(android::link_libraries(features));
}
