//! Skottie - Lottie Animation Support
//!
//! This module provides support for rendering Lottie animations via Skia's Skottie module.
//!
//! # Example
//!
//! ```no_run
//! use skia_safe::{skottie::Animation, surfaces};
//!
//! let json = r#"{"v":"5.5.7","fr":30,"ip":0,"op":60,"w":200,"h":200,"layers":[]}"#;
//! if let Some(animation) = Animation::from_str(json) {
//!     let mut surface = surfaces::raster_n32_premul((200, 200)).unwrap();
//!     animation.render(surface.canvas(), None);
//! }
//! ```

use std::{ffi::CString, fmt, path::Path};

use crate::{interop, prelude::*, Canvas, Rect, Size};
use skia_bindings::{self as sb, SkNVRefCnt};

/// A Lottie animation that can be rendered to a canvas.
///
/// Animations are reference-counted and can be cloned cheaply.
pub type Animation = RCHandle<sb::skottie_Animation>;
unsafe_send_sync!(Animation);
require_base_type!(sb::skottie_Animation, SkNVRefCnt);

impl NativeRefCounted for sb::skottie_Animation {
    fn _ref(&self) {
        unsafe { sb::C_skottie_Animation_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_skottie_Animation_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_skottie_Animation_unique(self) }
    }
}

impl fmt::Debug for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Animation")
            .field("version", &self.version())
            .field("duration", &self.duration())
            .field("fps", &self.fps())
            .field("size", &self.size())
            .finish()
    }
}

bitflags::bitflags! {
    /// Flags for rendering control.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RenderFlags: u32 {
        /// When rendering into a transparent canvas, disables the implicit
        /// top-level isolation layer.
        const SKIP_TOP_LEVEL_ISOLATION = 0x01;
        /// Disables the top-level clipping to the animation bounds.
        const DISABLE_TOP_LEVEL_CLIPPING = 0x02;
    }
}

impl Animation {
    /// Parse a Lottie animation from a JSON string.
    ///
    /// Returns `None` if the string cannot be parsed.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(json: impl AsRef<str>) -> Option<Self> {
        Self::from_bytes(json.as_ref().as_bytes())
    }

    /// Parse a Lottie animation from JSON bytes.
    ///
    /// Returns `None` if the data cannot be parsed.
    pub fn from_bytes(json: &[u8]) -> Option<Self> {
        Self::from_ptr(unsafe {
            sb::C_skottie_Animation_Make(json.as_ptr() as *const _, json.len())
        })
    }

    /// Load a Lottie animation from a file path.
    ///
    /// Returns `None` if the file cannot be loaded or parsed.
    /// Note: This will return `None` for non-UTF8 paths or paths containing null bytes.
    pub fn from_file(path: impl AsRef<Path>) -> Option<Self> {
        let path_str = path.as_ref().to_str()?;
        let c_path = CString::new(path_str).ok()?;
        Self::from_ptr(unsafe { sb::C_skottie_Animation_MakeFromFile(c_path.as_ptr()) })
    }

    /// Returns the Lottie format version string from the animation file.
    pub fn version(&self) -> interop::String {
        let mut version = interop::String::default();
        unsafe { sb::C_skottie_Animation_version(self.native(), version.native_mut()) };
        version
    }

    /// Returns the animation duration in seconds.
    pub fn duration(&self) -> f32 {
        unsafe { sb::C_skottie_Animation_duration(self.native()) }
    }

    /// Returns the animation frame rate (frames per second).
    pub fn fps(&self) -> f32 {
        unsafe { sb::C_skottie_Animation_fps(self.native()) }
    }

    /// Returns the first frame index (usually 0).
    pub fn in_point(&self) -> f32 {
        unsafe { sb::C_skottie_Animation_inPoint(self.native()) }
    }

    /// Returns the last frame index.
    pub fn out_point(&self) -> f32 {
        unsafe { sb::C_skottie_Animation_outPoint(self.native()) }
    }

    /// Returns the intrinsic animation size.
    pub fn size(&self) -> Size {
        let mut size = Size::default();
        unsafe { sb::C_skottie_Animation_size(self.native(), size.native_mut()) };
        size
    }

    /// Seek to a normalized position in the animation.
    ///
    /// `t` is in the range `[0, 1]` where 0 is the first frame and 1 is the last frame.
    ///
    /// Note: This method uses interior mutability as the underlying Skia animation
    /// state is updated internally.
    pub fn seek(&self, t: f32) {
        unsafe { sb::C_skottie_Animation_seek(self.native() as *const _ as *mut _, t) }
    }

    /// Seek to a specific frame number.
    ///
    /// Frame numbers are in the range `[in_point(), out_point()]`.
    /// Fractional frame values are supported for sub-frame accuracy.
    ///
    /// Note: This method uses interior mutability as the underlying Skia animation
    /// state is updated internally.
    pub fn seek_frame(&self, frame: f64) {
        unsafe { sb::C_skottie_Animation_seekFrame(self.native() as *const _ as *mut _, frame) }
    }

    /// Seek to a specific time in seconds.
    ///
    /// Time is in the range `[0, duration()]`.
    ///
    /// Note: This method uses interior mutability as the underlying Skia animation
    /// state is updated internally.
    pub fn seek_frame_time(&self, time: f64) {
        unsafe { sb::C_skottie_Animation_seekFrameTime(self.native() as *const _ as *mut _, time) }
    }

    /// Render the current frame to the canvas.
    ///
    /// If `dst` is provided, the animation is scaled/positioned to fit the rectangle.
    /// If `dst` is `None`, the animation is rendered at its intrinsic size at the origin.
    pub fn render(&self, canvas: &Canvas, dst: impl Into<Option<Rect>>) {
        let dst = dst.into();
        unsafe {
            sb::C_skottie_Animation_render(
                self.native(),
                canvas.native_mut(),
                dst.as_ref()
                    .map(|r| r.native() as *const _)
                    .unwrap_or(std::ptr::null()),
            )
        }
    }

    /// Render the current frame to the canvas with additional flags.
    ///
    /// If `dst` is provided, the animation is scaled/positioned to fit the rectangle.
    /// If `dst` is `None`, the animation is rendered at its intrinsic size at the origin.
    pub fn render_with_flags(
        &self,
        canvas: &Canvas,
        dst: impl Into<Option<Rect>>,
        flags: RenderFlags,
    ) {
        let dst = dst.into();
        unsafe {
            sb::C_skottie_Animation_render_with_flags(
                self.native(),
                canvas.native_mut(),
                dst.as_ref()
                    .map(|r| r.native() as *const _)
                    .unwrap_or(std::ptr::null()),
                flags.bits(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surfaces;

    #[test]
    fn parse_minimal_animation() {
        let json = r#"{"v":"5.5.7","fr":30,"ip":0,"op":60,"w":200,"h":200,"layers":[]}"#;
        let anim = Animation::from_str(json).expect("Failed to parse animation");

        assert_eq!(anim.version().as_str(), "5.5.7");
        assert_eq!(anim.fps(), 30.0);
        assert_eq!(anim.in_point(), 0.0);
        assert_eq!(anim.out_point(), 60.0);
        // duration = (out_point - in_point) / fps = 60 / 30 = 2.0
        assert!((anim.duration() - 2.0).abs() < 0.001);
        assert_eq!(anim.size(), Size::new(200.0, 200.0));
    }

    #[test]
    fn render_animation() {
        let json = r#"{"v":"5.5.7","fr":30,"ip":0,"op":60,"w":200,"h":200,"layers":[]}"#;
        let anim = Animation::from_str(json).expect("Failed to parse animation");

        let mut surface = surfaces::raster_n32_premul((200, 200)).unwrap();
        anim.seek(0.0);
        anim.render(surface.canvas(), None);
    }

    #[test]
    fn invalid_json_returns_none() {
        assert!(Animation::from_str("not valid json").is_none());
        assert!(Animation::from_str("{}").is_none());
    }
}
