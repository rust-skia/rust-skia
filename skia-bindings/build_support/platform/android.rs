use super::prelude::*;
use crate::build_support::android;

pub struct Android;

impl TargetDetails for Android {
    fn args(&self, config: &BuildConfiguration, builder: &mut ArgBuilder) {
        // TODO: this may belong into BuildConfiguration
        let use_system_libraries = skia::env::use_system_libraries();
        let (arch, _) = config.target.arch_abi();

        builder
            .arg("ndk", quote(&android::ndk()))
            .arg("ndk_api", android::API_LEVEL)
            .arg("target_cpu", quote(clang::target_arch(arch)))
            .arg("skia_enable_fontmgr_android", yes());

        if !config.features.embed_freetype {
            builder.arg("skia_use_system_freetype2", yes_if(use_system_libraries));
        }

        builder.cflags(android::extra_skia_cflags());
    }
}
