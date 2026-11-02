//! Base-class for objects that draw into a [`crate::Canvas`]. The object has a generation ID,
//! which is guaranteed to be unique across all drawables.

use std::fmt;

use skia_bindings::{self as sb, SkDrawable, SkFlattenable, SkRefCntBase};

use crate::{Canvas, Matrix, NativeFlattenable, Picture, Point, Rect, prelude::*};

/// Base-class for objects that draw into [`Canvas`].
///
/// The object has a generation ID, which is guaranteed to be unique across all drawables. To allow
/// for clients of the drawable that may want to cache the results, the drawable must change its
/// generation ID whenever its internal state changes such that it will draw differently.
pub type Drawable = RCHandle<SkDrawable>;

impl NativeRefCountedBase for SkDrawable {
    type Base = SkRefCntBase;
}

impl NativeFlattenable for SkDrawable {
    fn native_flattenable(&self) -> &SkFlattenable {
        unsafe { &*(self as *const SkDrawable as *const SkFlattenable) }
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkDrawable_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl fmt::Debug for Drawable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            // TODO: clarify why &self has to be mut here.
            // .field("generation_id", &self.generation_id())
            // .field("bounds", &self.bounds())
            .finish()
    }
}

impl Drawable {
    /// Draws into the specified content. The drawing sequence will be balanced upon return (i.e.
    /// the save level on the canvas will match what it was when `draw` was called, and the current
    /// matrix and clip settings will not be changed).
    ///
    /// - `canvas` canvas to draw into
    /// - `matrix` optional matrix to apply
    pub fn draw(&mut self, canvas: &Canvas, matrix: Option<&Matrix>) {
        unsafe {
            self.native_mut()
                .draw(canvas.native_ptr(), matrix.native_ptr_or_null())
        }
    }

    /// Draws into the specified content at the given point.
    ///
    /// - `canvas` canvas to draw into
    /// - `point` point to draw at
    pub fn draw_at(&mut self, canvas: &Canvas, point: impl Into<Point>) {
        let point = point.into();
        unsafe {
            self.native_mut()
                .draw1(canvas.native_ptr(), point.x, point.y)
        }
    }

    /// Snaps off a [`gpu_draw_handler::GPUDrawHandler`] to represent the state of the drawable at
    /// the time the snap is called. This is used for executing GPU backend specific draws
    /// intermixed with normal Skia GPU draws. The GPU API, which will be used for the draw, as well
    /// as the full matrix, device clip bounds and image info of the target buffer are passed in as
    /// inputs.
    ///
    /// - `api` GPU backend API
    /// - `matrix` full matrix
    /// - `clip_bounds` device clip bounds
    /// - `buffer_info` image info of the target buffer
    #[cfg(feature = "ganesh")]
    pub fn snap_gpu_draw_handler(
        &mut self,
        api: crate::gpu::ganesh::BackendApi,
        matrix: &Matrix,
        clip_bounds: impl Into<crate::IRect>,
        buffer_info: &crate::ImageInfo,
    ) -> Option<gpu_draw_handler::GPUDrawHandler> {
        gpu_draw_handler::GPUDrawHandler::from_ptr(unsafe {
            sb::C_SkDrawable_snapGpuDrawHandler(
                self.native_mut(),
                api,
                matrix.native(),
                clip_bounds.into().native(),
                buffer_info.native(),
            )
        })
    }

    /// Returns a [`Picture`] with the contents of this drawable.
    pub fn make_picture_snapshot(&mut self) -> Picture {
        Picture::from_ptr(unsafe { sb::C_SkDrawable_makePictureSnapshot(self.native_mut()) })
            .expect("Internal error: SkDrawable::makePictureSnapshot returned null")
    }

    /// Returns a unique value for this instance. If two calls to this return the same value, it is
    /// presumed that calling the [`Self::draw()`] method will render the same thing as well.
    ///
    /// Subclasses that change their state should call [`Self::notify_drawing_changed()`] to ensure
    /// that a new value will be returned the next time it is called.
    pub fn generation_id(&mut self) -> u32 {
        unsafe { self.native_mut().getGenerationID() }
    }

    /// Returns the (conservative) bounds of what the drawable will draw. If the drawable can change
    /// what it draws (e.g. animation or in response to some external change), then this must return
    /// a bounds that is always valid for all possible states.
    pub fn bounds(&mut self) -> Rect {
        Rect::construct(|r| unsafe { sb::C_SkDrawable_getBounds(self.native_mut(), r) })
    }

    /// Returns approximately how many bytes would be freed if this drawable is destroyed. The base
    /// implementation returns 0 to indicate that this is unknown.
    pub fn approximate_bytes_used(&mut self) -> usize {
        unsafe { self.native_mut().approximateBytesUsed() }
    }

    /// Calling this invalidates the previous generation ID, and causes a new one to be computed the
    /// next time [`Self::generation_id()`] is called. Typically this is called by the object itself,
    /// in response to its internal state changing.
    pub fn notify_drawing_changed(&mut self) {
        unsafe { self.native_mut().notifyDrawingChanged() }
    }
}

#[cfg(feature = "ganesh")]
pub use gpu_draw_handler::*;

#[cfg(feature = "ganesh")]
pub mod gpu_draw_handler {
    //! GPU backend support for drawables: a [`GPUDrawHandler`] lets a drawable execute using the
    //! underlying 3D API rather than the [`crate::Canvas`] API.
    use std::fmt;

    use skia_bindings::{self as sb, SkDrawable_GpuDrawHandler};

    use crate::prelude::*;

    /// When using the GPU backend it is possible for a drawable to execute using the underlying 3D
    /// API rather than the [`crate::Canvas`] API. It does so by creating a GPU draw handler. The GPU
    /// backend is deferred so the handler will be given access to the 3D API at the correct point
    /// in the drawing stream as the GPU backend flushes. Since the drawable may mutate, each time it
    /// is drawn to a GPU-backed canvas a new handler is snapped, representing the drawable's state
    /// at the time of the snap.
    ///
    /// When the GPU backend flushes to the 3D API it will call the draw method on the GPU draw
    /// handler. At this time the drawable may add commands to the stream of GPU commands for the
    /// underlying 3D API. The draw function takes a backend drawable info which contains
    /// information about the current state of the 3D API which the caller must respect. See the
    /// backend drawable info for more specific details on what information is sent and the
    /// requirements for different 3D APIs.
    ///
    /// Additionally there may be a slight delay from when the drawable adds its commands to when
    /// those commands are actually submitted to the GPU. Thus the drawable or GPU draw handler is
    /// required to keep any resources that are used by its added commands alive and valid until
    /// those commands are submitted to the GPU. The GPU draw handler will be kept alive and then
    /// deleted once the commands are submitted to the GPU. The destructor of the GPU draw handler
    /// is the signal to the drawable that the commands have all been submitted. Different 3D APIs
    /// may have additional requirements for certain resources which require waiting for the GPU to
    /// finish all work on those resources before reusing or deleting them. In this case, the
    /// drawable can use the destructor call of the GPU draw handler to add a fence to the GPU to
    /// track when the GPU work has completed.
    ///
    /// Currently this is only supported for the GPU Vulkan backend.
    pub type GPUDrawHandler = RefHandle<SkDrawable_GpuDrawHandler>;

    impl NativeDelete for SkDrawable_GpuDrawHandler {
        unsafe fn delete(ptr: *mut Self) {
            unsafe { sb::C_SkDrawable_GpuDrawHandler_delete(ptr) }
        }
    }

    impl fmt::Debug for GPUDrawHandler {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GPUDrawHandler").finish()
        }
    }

    #[cfg(feature = "vulkan")]
    impl GPUDrawHandler {
        pub fn draw(&mut self, info: &crate::gpu::vk::BackendDrawableInfo) {
            unsafe {
                sb::C_SkDrawable_GpuDrawHandler_draw(self.native_ptr(), info.native());
            }
        }
    }
}
