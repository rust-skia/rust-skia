use super::prelude::*;
use crate::build_support::android;

pub struct Android;

impl PlatformDetails for Android {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        // TODO: this may belong into BuildConfiguration
        let (arch, _) = config.target.arch_abi();

        builder
            .skia("ndk", quote(&android::ndk()))
            .skia("ndk_api", android::API_LEVEL)
            .skia("target_cpu", quote(clang::target_arch(arch)))
            .skia("skia_enable_fontmgr_android", yes());

        if !config.features.embed_freetype {
            builder.skia(
                "skia_use_system_freetype2",
                yes_if(builder.use_system_libraries()),
            );
        }

        builder.skia_cflags(android::extra_skia_cflags());
    }

    fn bindgen_args(&self, target: &Platform, builder: &mut BindgenArgsBuilder) {
        builder.clang_args(android::additional_clang_args(
            &target.to_string(),
            &target.architecture,
        ));
    }

    fn link_libraries(&self, features: &Features, builder: &mut LinkLibrariesBuilder) {
        builder.link_libraries(android::link_libraries(features));
    }
}
