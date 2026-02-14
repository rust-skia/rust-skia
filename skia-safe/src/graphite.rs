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
//! // Context creation would be platform-specific
//! # let context = None::<graphite::Context>;
//! # if let Some(context) = context {
//! // Create a recorder for recording draw operations
//! let recorder = context.make_recorder(None)?;
//!
//! // Get a canvas to draw on
//! let canvas = recorder.canvas();
//! // ... perform drawing operations ...
//!
//! // Finish recording and submit
//! let recording = recorder.snap()?;
//! let info = graphite::InsertRecordingInfo::new(&recording);
//! context.insert_recording(&info);
//! context.submit(None);
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
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
    pub use super::recorder::Recorder;
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

#[cfg(feature = "metal")]
pub mod mtl;

#[cfg(feature = "metal")]
pub use mtl::{make_backend_texture, make_context, BackendContext, Handle};

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
        // Test that graphite modules can be referenced
        // This ensures the module structure is correct
        use super::{images, surfaces};
        let _ = (&images, &surfaces);
    }
}
