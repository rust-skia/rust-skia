mod backend_drawable_info;
mod backend_surface;
pub mod context_options;
#[cfg(feature = "d3d")]
pub mod d3d;
mod direct_context;
mod driver_bug_workarounds;
mod ganesh;
#[cfg(feature = "gl")]
pub mod gl;
mod gpu_types;
#[cfg(feature = "metal")]
pub mod mtl;
mod mutable_texture_state;
mod recording_context;
mod types;
#[cfg(feature = "vulkan")]
pub mod vk;
mod yuva_backend_textures;

pub use backend_drawable_info::*;
pub use backend_surface::*;
pub use context_options::ContextOptions;
pub use direct_context::*;
pub use driver_bug_workarounds::DriverBugWorkarounds;
pub use ganesh::image_ganesh as images;
pub use gpu_types::*;
pub use mutable_texture_state::*;
pub use recording_context::*;
pub use types::*;
pub use yuva_backend_textures::*;

#[deprecated(since = "0.37.0", note = "Use RecordingContext or DirectContext")]
pub type Context = DirectContext;

pub mod surfaces {
    #[cfg(feature = "metal")]
    pub use super::ganesh::mtl::*;
    pub use super::ganesh::surface_ganesh::*;
}

pub mod backend_formats {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::backend_formats::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::backend_formats::*;
}

pub mod backend_textures {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::backend_textures::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::backend_textures::*;
}

pub mod backend_render_targets {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::backend_render_targets::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::backend_render_targets::*;
}

pub mod direct_contexts {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::direct_contexts::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::direct_contexts::*;
}

#[cfg(test)]
mod tests {
    use super::{DirectContext, RecordingContext};

    #[test]
    fn implicit_deref_conversion_from_direct_context_to_context_to_recording_context() {
        fn _recording_context(_context: &RecordingContext) {}
        fn _context(context: &DirectContext) {
            _recording_context(context)
        }
        fn _direct_context(context: &DirectContext) {
            _context(context)
        }

        fn _recording_context_mut(_context: &mut RecordingContext) {}
        fn _context_mut(context: &mut DirectContext) {
            _recording_context_mut(context)
        }
        fn _direct_context_mut(context: &mut DirectContext) {
            _context_mut(context)
        }
    }
}
