#[cfg(feature = "d3d")]
pub mod d3d;
mod ganesh;
#[cfg(feature = "gl")]
pub mod gl;
mod mutable_texture_state;
#[cfg(feature = "vulkan")]
pub mod vk;

pub use context_options::ContextOptions;
pub use ganesh::backend_surface::*;
pub use ganesh::context_options;
pub use ganesh::direct_context::*;
pub use ganesh::driver_bug_workarounds::*;
pub use ganesh::image_ganesh as images;
pub use ganesh::recording_context::*;
pub use ganesh::types::*;
pub use ganesh::yuva_backend_textures::*;
pub use mutable_texture_state::*;

#[deprecated(since = "0.37.0", note = "Use RecordingContext or DirectContext")]
pub type Context = DirectContext;

#[cfg(feature = "metal")]
pub mod mtl {
    pub use super::ganesh::mtl::{types::*, BackendContext};
}

pub mod surfaces {
    #[cfg(feature = "metal")]
    pub use super::ganesh::mtl::surface_metal::*;
    pub use super::ganesh::surface_ganesh::*;
}

pub mod backend_formats {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::backend_formats::*;
    #[cfg(feature = "metal")]
    pub use super::ganesh::mtl::backend_formats::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::backend_formats::*;
}

pub mod backend_textures {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::backend_textures::*;
    #[cfg(feature = "metal")]
    pub use super::ganesh::mtl::backend_textures::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::backend_textures::*;
}

pub mod backend_render_targets {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::backend_render_targets::*;
    #[cfg(feature = "metal")]
    pub use super::ganesh::mtl::backend_render_targets::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::backend_render_targets::*;
}

pub mod direct_contexts {
    #[cfg(feature = "gl")]
    pub use super::ganesh::gl::direct_contexts::*;
    #[cfg(feature = "metal")]
    pub use super::ganesh::mtl::direct_contexts::*;
    #[cfg(feature = "vulkan")]
    pub use super::ganesh::vk::direct_contexts::*;
}

#[cfg(feature = "gl")]
pub mod interfaces {
    #[cfg(feature = "egl")]
    pub use super::ganesh::gl::make_egl_interface::interfaces::*;
    #[cfg(target_os = "ios")]
    pub use super::ganesh::gl::make_ios_interface::interfaces::*;
    #[cfg(target_os = "macos")]
    pub use super::ganesh::gl::make_mac_interface::interfaces::*;
    #[cfg(target_arch = "wasm32")]
    pub use super::ganesh::gl::make_web_gl_interface::interfaces::*;
    #[cfg(target_os = "windows")]
    pub use super::ganesh::gl::make_win_interface::interfaces::*;
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
