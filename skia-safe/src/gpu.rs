mod backend_drawable_info;
pub use self::backend_drawable_info::*;

mod backend_surface;
pub use self::backend_surface::*;

mod backend_surface_mutable_state;
pub use self::backend_surface_mutable_state::*;

pub mod context_options;
pub use self::context_options::ContextOptions;

#[cfg(feature = "d3d")]
pub mod d3d;

mod direct_context;
pub use self::direct_context::*;

#[deprecated(since = "0.37.0", note = "Use RecordingContext or DirectContext")]
pub type Context = DirectContext;

mod driver_bug_workarounds;
pub use self::driver_bug_workarounds::DriverBugWorkarounds;

#[cfg(feature = "gl")]
pub mod gl;

#[cfg(feature = "metal")]
pub mod mtl;

mod recording_context;
pub use self::recording_context::*;

mod types;
pub use self::types::*;

#[cfg(feature = "vulkan")]
pub mod vk;

mod yuva_backend_textures;
pub use yuva_backend_textures::*;

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
