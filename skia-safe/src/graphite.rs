//! Graphite GPU backend support
//!
//! Graphite is Skia's next-generation GPU backend, designed to provide
//! better performance and more predictable behavior than the legacy Ganesh backend.
//!
//! # Overview
//!
//! Graphite offers:
//! - More explicit resource management
//! - Better multi-threading support
//! - Reduced driver overhead
//! - More predictable performance characteristics
//!
//! # Basic Usage
//!
//! ```no_run
//! use skia_safe::graphite;
//!
//! // Context creation is platform-specific (see `graphite::mtl` / `graphite::vk`).
//! # let context = None::<graphite::Context>;
//! # if let Some(mut context) = context {
//! // Create a recorder for recording draw operations
//! let mut recorder = context.make_recorder(None).expect("make_recorder");
//!
//! // Create a Graphite-backed surface from the recorder (see `graphite::surfaces`)
//! // and draw into its canvas, then finish recording and submit:
//! let mut recording = recorder.snap().expect("snap");
//! let info = graphite::InsertRecordingInfo::new(&mut recording);
//! context.insert_recording(&info);
//! context.submit(None);
//! # }
//! ```

mod backend_texture;
mod context;
mod context_options;
mod image_graphite;
mod recorder;
mod recording;
mod surface_graphite;
mod texture_info;
mod types;

mod implementation {
    // Core types
    pub use super::context::Context;
    pub use super::recorder::{BorrowedRecorder, Recorder};
    pub use super::recording::Recording;

    // Configuration and options
    pub use super::context_options::ContextOptions;
    pub use super::types::*;

    // Texture and backend types
    pub use super::backend_texture::BackendTexture;
    pub use super::texture_info::TextureInfo;
}

// Surface and image creation functions - re-export as modules
pub mod surfaces {
    //! Surface creation functions for Graphite
    pub use super::surface_graphite::*;
}

pub mod images {
    //! Image utilities for Graphite
    pub use super::image_graphite::*;
}

// Backend entry points are kept module-qualified for a symmetric API and to
// avoid the two backends' `make_context` clashing at the `graphite` root:
// `graphite::mtl::make_context` / `graphite::vk::make_context` (mirrors the
// `gpu::mtl` / `gpu::vk` namespacing). `vk` reuses `gpu::vk::BackendContext`.
#[cfg(feature = "metal")]
pub mod mtl;

#[cfg(feature = "vulkan")]
pub mod vk;

pub use implementation::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphite_types_accessible() {
        // Test that graphite types can be referenced without compilation errors
        let _context: Option<Context> = None;
        let _recorder: Option<Recorder> = None;
        let _recording: Option<Recording> = None;
        let _backend_texture: Option<BackendTexture> = None;
        let _texture_info: Option<TextureInfo> = None;
        let _context_options: Option<ContextOptions> = None;
    }

    #[test]
    fn test_graphite_modules_accessible() {
        // Ensure the module structure is correct by naming an item from each
        // module (a module path itself is not a value, so reference functions).
        use super::{images, surfaces};
        let _ = surfaces::render_target;
        // `wrap_texture` is generic over the color space argument; reference a
        // non-generic item so the module path can be named without turbofish.
        let _ = images::texture_from_image;
    }
}
