use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;

use skia_bindings as sb;

use super::{Recording, TextureInfo};
use crate::gpu::BackendApi;
use crate::prelude::*;
use crate::{Canvas, Image, ImageInfo, image::RequiredProperties};

/// Configuration for recorder creation
pub type RecorderOptions = Handle<sb::skgpu_graphite_RecorderOptions>;

impl NativeDrop for sb::skgpu_graphite_RecorderOptions {
    fn drop(&mut self) {
        unsafe { sb::C_RecorderOptions_destruct(self) }
    }
}

impl fmt::Debug for RecorderOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecorderOptions").finish()
    }
}

impl Default for RecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl RecorderOptions {
    /// Create new recorder options with the C++ defaults (e.g. a 256 MiB GPU
    /// budget). Placement-constructed rather than zero-initialized, because
    /// `RecorderOptions` has a non-trivial constructor and members (an `sk_sp`,
    /// a `std::optional`, and a non-zero default budget).
    pub fn new() -> Self {
        Self::construct(|options| unsafe { sb::C_RecorderOptions_Construct(options) })
    }

    /// Sets the [`ImageProvider`] that recorders created with these options ask for a
    /// Graphite-backed version of every image that is not Graphite-backed.
    ///
    /// Without a provider, Graphite drops draws of raster and other non-Graphite images.
    pub fn set_image_provider<T: ImageProvider + 'static>(
        &mut self,
        image_provider: T,
    ) -> &mut Self {
        let param = sb::RustImageProvider_Param {
            data: Box::into_raw(Box::new(image_provider)).cast(),
            drop: Some(drop_provider::<T>),
            findOrCreate: Some(find_or_create::<T>),
        };
        unsafe {
            let provider = sb::C_RustImageProvider_New(&param);
            sb::C_RecorderOptions_setImageProvider(self.native_mut(), provider);
        }
        self
    }
}

/// Lets clients cache the Graphite-backed versions of images that are not Graphite-backed.
///
/// Whenever Graphite draws an [`Image`] that is not Graphite-backed (a raster image, for example),
/// it calls [`find_or_create`](Self::find_or_create). The returned image must be Graphite-backed
/// and preserve the dimensions and alpha type of the original. If it is `None` or does not meet the
/// requirements, Graphite drops the draw.
///
/// Graphite does not cache these images itself. A provider that caches them by
/// [`Image::unique_id`], as Chrome does, avoids uploading the same image on every draw:
///
/// ```no_run
/// use std::collections::HashMap;
/// use skia_safe::{gpu::graphite, image::RequiredProperties, Image};
///
/// #[derive(Default)]
/// struct CachingProvider(HashMap<(u32, bool), Image>);
///
/// impl graphite::ImageProvider for CachingProvider {
///     fn find_or_create(
///         &mut self,
///         recorder: &mut graphite::BorrowedRecorder,
///         image: &Image,
///         required: RequiredProperties,
///     ) -> Option<Image> {
///         let key = (image.unique_id(), required.mipmapped);
///         if let Some(cached) = self.0.get(&key) {
///             return Some(cached.clone());
///         }
///         let uploaded = recorder.texture_from_image(image, required)?;
///         self.0.insert(key, uploaded.clone());
///         Some(uploaded)
///     }
/// }
///
/// let mut options = graphite::RecorderOptions::new();
/// options.set_image_provider(CachingProvider::default());
/// ```
///
/// If the same provider is given to multiple recorders, the client is responsible for any
/// synchronization; see the Skia documentation of `skgpu::graphite::ImageProvider`.
pub trait ImageProvider {
    /// Returns a Graphite-backed version of `image` that has the `required` properties, or `None`
    /// to drop the draw.
    fn find_or_create(
        &mut self,
        recorder: &mut BorrowedRecorder,
        image: &Image,
        required: RequiredProperties,
    ) -> Option<Image>;
}

unsafe extern "C" fn drop_provider<T>(provider: *mut c_void) {
    mem::drop(unsafe { Box::from_raw(provider.cast::<T>()) });
}

/// `provider` is the `Box<T>` leaked in [`RecorderOptions::set_image_provider`], the only place
/// that pairs it with this callback. It stays valid until `drop_provider` reclaims it.
unsafe extern "C" fn find_or_create<T: ImageProvider>(
    provider: *mut c_void,
    recorder: *mut sb::skgpu_graphite_Recorder,
    image: *const sb::SkImage,
    required: *const sb::SkImage_RequiredProperties,
) -> *mut sb::SkImage {
    let provider = unsafe { &mut *provider.cast::<T>() };
    let Some(recorder) = Recorder::from_ptr(recorder) else {
        return std::ptr::null_mut();
    };
    let mut recorder = BorrowedRecorder {
        recorder: ManuallyDrop::new(recorder),
        _owner: PhantomData,
    };
    let image_ptr = image as *mut sb::SkImage;
    let Some(image) = Image::from_unshared_ptr_ref(&image_ptr) else {
        return std::ptr::null_mut();
    };
    let required = RequiredProperties {
        mipmapped: unsafe { (*required).fMipmapped },
    };
    provider
        .find_or_create(&mut recorder, image, required)
        .map(|image| image.into_ptr())
        .unwrap_or(std::ptr::null_mut())
}

// `skgpu::graphite::Recorder` is handed out as `std::unique_ptr<Recorder>`
// (Context::makeRecorder) and derives from `SkRecorder`, not `SkRefCnt`. It is
// NOT ref-counted, so it must be a `RefHandle` whose drop `delete`s it.
pub type Recorder = RefHandle<sb::skgpu_graphite_Recorder>;

/// A non-owning, lifetime-bound view of a [`Recorder`] that is owned elsewhere
/// (for example by a Graphite-backed `Canvas`/surface, via `Canvas::recorder`).
///
/// Dropping a `BorrowedRecorder` does **not** `delete` the underlying recorder —
/// the owner keeps that responsibility — and the lifetime ties the borrow to the
/// object it was obtained from, so it cannot dangle.
///
/// Only shared access to the underlying [`Recorder`] is exposed (via `Deref`);
/// the mutating operations are forwarded as inherent methods. Handing out
/// `&mut Recorder` would be unsound: safe code could `mem::replace` a recorder
/// created by `Context::make_recorder` into it and end up owning (and later
/// dropping, i.e. `delete`ing) a recorder the surface still owns.
#[derive(Debug)]
pub struct BorrowedRecorder<'a> {
    recorder: ManuallyDrop<Recorder>,
    _owner: PhantomData<&'a Canvas>,
}

impl Deref for BorrowedRecorder<'_> {
    type Target = Recorder;

    fn deref(&self) -> &Recorder {
        &self.recorder
    }
}

impl<'a> BorrowedRecorder<'a> {
    pub(crate) fn from_canvas(recorder: Recorder, _canvas: &'a Canvas) -> Self {
        Self {
            recorder: ManuallyDrop::new(recorder),
            _owner: PhantomData,
        }
    }

    /// See [`Recorder::snap`].
    pub fn snap(&mut self) -> Option<Recording> {
        self.recorder.snap()
    }

    /// Uploads `image` and returns a Graphite-backed version of it with the `required`
    /// properties. See [`super::images::texture_from_image`].
    pub fn texture_from_image(
        &mut self,
        image: &Image,
        required: RequiredProperties,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImages_TextureFromImageGraphite(
                self.recorder.native_mut(),
                image.native(),
                required.native(),
            )
        })
    }

    /// See [`Recorder::make_deferred_canvas`].
    pub fn make_deferred_canvas(
        &mut self,
        image_info: &ImageInfo,
        texture_info: &TextureInfo,
    ) -> Option<&Canvas> {
        self.recorder.make_deferred_canvas(image_info, texture_info)
    }
}

// Deliberately NOT `Send`/`Sync`: Skia documents a Graphite `Recorder` as
// single-threaded (it and its child objects must be used on one thread), and its
// `&self`/`&mut self` methods drive C++ with no internal lock. Matches the
// Ganesh `gpu::RecordingContext`, which is neither `Send` nor `Sync`.

impl NativeDrop for sb::skgpu_graphite_Recorder {
    fn drop(&mut self) {
        unsafe { sb::C_Recorder_delete(self) }
    }
}

impl fmt::Debug for Recorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recorder")
            .field("backend", &self.backend())
            .finish()
    }
}

impl Recorder {
    /// Finish recording and create a Recording object
    ///
    /// This method finalizes all the draw operations that have been recorded
    /// and returns a Recording that can be submitted to a Context.
    ///
    /// # Returns
    /// A `Recording` containing the recorded operations, or `None` if recording failed
    pub fn snap(&mut self) -> Option<Recording> {
        Recording::from_ptr(unsafe { sb::C_Recorder_snap(self.native_mut()) })
    }

    // Note: Canvas creation in Graphite is typically done through Surface creation
    // Surface::canvas() is the recommended way to get a canvas for drawing
    // See the `surfaces` module for surface creation functions.

    /// Returns a canvas that records into a proxy surface (instantiated on
    /// replay), targeting a texture with the given `image_info` / `texture_info`.
    ///
    /// The returned canvas is owned by the recorder and borrows it: it is only
    /// valid until the next [`snap`](Self::snap) — which the borrow checker
    /// enforces, since `snap` needs `&mut self`. Returns `None` if a deferred
    /// canvas is already outstanding for the current recording (only one may
    /// exist per recording, until the next `snap`).
    pub fn make_deferred_canvas(
        &mut self,
        image_info: &ImageInfo,
        texture_info: &TextureInfo,
    ) -> Option<&Canvas> {
        let canvas_ptr = unsafe {
            sb::C_Recorder_makeDeferredCanvas(
                self.native_mut(),
                image_info.native(),
                texture_info.native(),
            )
        };
        if canvas_ptr.is_null() {
            None
        } else {
            Some(Canvas::borrow_from_native(unsafe { &*canvas_ptr }))
        }
    }

    /// Get the backend API used by this recorder
    ///
    /// # Returns
    /// The backend API (Vulkan, Metal, etc.)
    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_Recorder_backend(self.native()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_options_creation() {
        let options = RecorderOptions::new();
        let _default_options = RecorderOptions::default();
        let _ = format!("{:?}", options);
    }

    #[test]
    fn image_provider_is_dropped_with_the_options() {
        use std::rc::Rc;

        struct Provider(Rc<()>);
        impl ImageProvider for Provider {
            fn find_or_create(
                &mut self,
                _recorder: &mut BorrowedRecorder,
                _image: &Image,
                _required: RequiredProperties,
            ) -> Option<Image> {
                None
            }
        }

        let alive = Rc::new(());
        let mut options = RecorderOptions::new();
        options.set_image_provider(Provider(alive.clone()));
        assert_eq!(Rc::strong_count(&alive), 2);
        options.set_image_provider(Provider(alive.clone()));
        assert_eq!(
            Rc::strong_count(&alive),
            2,
            "replacing a provider drops the old one"
        );
        drop(options);
        assert_eq!(Rc::strong_count(&alive), 1);
    }
}
