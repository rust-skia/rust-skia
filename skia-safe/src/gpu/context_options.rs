use crate::gpu::DriverBugWorkarounds;
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::GrContextOptions;
use std::os::raw;

pub use skia_bindings::GrContextOptions_Enable as Enable;
pub use skia_bindings::GrContextOptions_ShaderCacheStrategy as ShaderCacheStrategy;

#[repr(C)]
pub struct ContextOptions {
    pub suppress_prints: bool,
    pub skip_gl_error_checks: Enable,
    pub max_texture_size_override: raw::c_int,
    pub buffer_map_threshold: raw::c_int,
    executor: *mut sb::SkExecutor,
    pub do_manual_mipmapping: bool,
    pub disable_coverage_counting_paths: bool,
    pub disable_distance_field_paths: bool,
    pub allow_path_mask_caching: bool,
    pub disable_gpu_yuv_conversion: bool,
    pub glyph_cache_texture_maximum_bytes: usize,
    pub min_distance_field_font_size: f32,
    pub glyphs_as_paths_font_size: f32,
    pub allow_multiple_glyph_cache_textures: Enable,
    pub avoid_stencil_buffers: bool,
    pub sharpen_mipmapped_textures: bool,
    pub use_draw_instead_of_clear: Enable,
    pub reduce_ops_task_splitting: Enable,
    pub prefer_external_images_over_es3: bool,
    pub disable_driver_correctness_workarounds: bool,
    pub runtime_program_cache_size: raw::c_int,
    persistent_cache: *mut sb::GrContextOptions_PersistentCache,
    pub shader_cache_strategy: ShaderCacheStrategy,
    shader_error_handler: *mut sb::GrContextOptions_ShaderErrorHandler,
    pub internal_multisample_count: raw::c_int,
    pub max_cached_vulkan_secondary_command_buffers: raw::c_int,
    pub suppress_mipmap_support: bool,
    pub driver_bug_workarounds: DriverBugWorkarounds,
}
unsafe impl Send for ContextOptions {}
unsafe impl Sync for ContextOptions {}

impl Default for ContextOptions {
    fn default() -> Self {
        Self::construct(|ptr| unsafe { sb::C_GrContextOptions_Construct(ptr) })
    }
}

impl ContextOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NativeTransmutable<GrContextOptions> for ContextOptions {}

#[cfg(test)]
mod tests {
    use crate::prelude::NativeTransmutable;
    #[test]
    fn test_enable_naming() {
        let _ = super::Enable::Yes;
    }

    #[test]
    fn test_shader_cache_strategy_naming() {
        let _ = super::ShaderCacheStrategy::BackendSource;
    }

    #[test]
    fn test_context_options_layout() {
        super::ContextOptions::test_layout()
    }
}
