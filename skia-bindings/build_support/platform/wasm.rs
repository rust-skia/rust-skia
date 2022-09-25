use super::prelude::*;

pub struct Emscripten;

impl TargetDetails for Emscripten {
    fn args(&self, config: &BuildConfiguration, builder: &mut ArgBuilder) {
        let features = &config.features;

        builder
            .arg("cc", quote("emcc"))
            .arg("cxx", quote("em++"))
            .arg("skia_gl_standard", quote("webgl"))
            .arg("skia_use_freetype", yes())
            .arg("skia_use_system_freetype2", no())
            .arg("skia_use_webgl", yes_if(features.gpu()))
            .arg("target_cpu", quote("wasm"));

        // The custom embedded font manager is enabled by default on WASM, but depends
        // on the undefined symbol `SK_EMBEDDED_FONTS`. Enable the custom empty font
        // manager instead so typeface creation still works.
        // See https://github.com/rust-skia/rust-skia/issues/648
        builder
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes());
    }
}
