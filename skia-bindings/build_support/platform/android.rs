use super::prelude::*;
use crate::build_support::android;

pub struct Android;

impl PlatformDetails for Android {
    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        // TODO: this may belong into BuildConfiguration
        let (arch, _) = config.target.arch_abi();

        builder
            .arg("ndk", quote(&android::ndk()))
            .arg("ndk_api", android::API_LEVEL)
            .arg("target_cpu", quote(clang::target_arch(arch)))
            .arg("skia_enable_fontmgr_android", yes());

        if !config.features.embed_freetype {
            builder.arg(
                "skia_use_system_freetype2",
                yes_if(builder.use_system_libraries()),
            );
        }

        builder.cflags(android::extra_skia_cflags());
    }

    fn bindgen_args(&self, target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(android::additional_clang_args(
            &target.to_string(),
            &target.architecture,
        ));
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        android::link_libraries(features)
            .iter()
            .map(|l| l.to_string())
            .collect()
    }
}



