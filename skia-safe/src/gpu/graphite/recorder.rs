use std::cell::RefCell;
use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;

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
    ///
    /// Recorders created from these options share the provider and keep it alive. A later call
    /// replaces and drops it. `T` needs no `Send` because recorders and their options stay on one
    /// thread.
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
/// it calls [`find_or_create`](Self::find_or_create). The returned image must be Graphite-backed,
/// preserve the dimensions, alpha type and color channels of the original, and have a top-left
/// origin. If it is `None` or does not meet these requirements, Graphite drops the draw.
///
/// Graphite does not cache these images itself. A provider that caches them by
/// [`Image::unique_id`], as Chrome does, avoids uploading the same image on every draw. The example
/// below never evicts an entry, so every image it has seen keeps its texture alive. A real provider
/// bounds its cache, for example with a least-recently-used limit like Skia's own test provider.
///
/// ```no_run
/// use std::{cell::RefCell, collections::HashMap};
/// use skia_safe::{gpu::graphite, image::RequiredProperties, Image};
///
/// #[derive(Default)]
/// struct CachingProvider(RefCell<HashMap<(u32, bool), Image>>);
///
/// impl graphite::ImageProvider for CachingProvider {
///     fn find_or_create(
///         &self,
///         recorder: &graphite::BorrowedRecorder,
///         image: &Image,
///         required: RequiredProperties,
///     ) -> Option<Image> {
///         let key = (image.unique_id(), required.mipmapped);
///         if let Some(cached) = self.0.borrow().get(&key) {
///             return Some(cached.clone());
///         }
///         // Uploading a picture-backed image replays its picture, which calls this provider
///         // again, so the cache must not stay borrowed across the upload.
///         let uploaded = recorder.texture_from_image(image, required)?;
///         self.0.borrow_mut().insert(key, uploaded.clone());
///         Some(uploaded)
///     }
/// }
///
/// let mut options = graphite::RecorderOptions::new();
/// options.set_image_provider(CachingProvider::default());
/// ```
///
/// Graphite calls the provider again while it is still running whenever an upload draws, for
/// example when a picture-backed image is uploaded. That is why the provider receives only shared
/// references, and why a caching provider keeps its cache in a `RefCell` that it does not borrow
/// across an upload.
pub trait ImageProvider {
    /// Returns a Graphite-backed version of `image` that has the `required` properties, or `None`
    /// to drop the draw.
    fn find_or_create(
        &self,
        recorder: &BorrowedRecorder,
        image: &Image,
        required: RequiredProperties,
    ) -> Option<Image>;
}

/// Releases the `Box<T>` leaked by [`RecorderOptions::set_image_provider`]. The C++
/// `RustImageProvider` destructor calls it once the last reference to the provider is gone.
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
    let provider = unsafe { &*provider.cast::<T>() };
    let _drawing = DrawingScope::enter(recorder);
    let recorder = BorrowedRecorder {
        recorder: ManuallyDrop::new(
            Recorder::from_ptr(recorder).expect("Graphite passes the drawing recorder"),
        ),
        _owner: PhantomData,
    };
    let image_ptr = image as *mut sb::SkImage;
    let image = Image::from_unshared_ptr_ref(&image_ptr)
        .as_ref()
        .expect("Graphite passes the image it draws");
    let required = RequiredProperties::from_native_c(unsafe { *required });
    provider
        .find_or_create(&recorder, image, required)
        .map(|image| image.into_ptr())
        .unwrap_or(ptr::null_mut())
}

/// Lists a recorder in [`DRAWING_RECORDERS`] for as long as the scope lives, so that
/// [`Recorder::snap`] and dropping the recorder refuse to run on it during the provider call.
struct DrawingScope;

impl DrawingScope {
    fn enter(recorder: *const sb::skgpu_graphite_Recorder) -> Self {
        DRAWING_RECORDERS.with_borrow_mut(|drawing_recorders| drawing_recorders.push(recorder));
        Self
    }
}

impl Drop for DrawingScope {
    fn drop(&mut self) {
        DRAWING_RECORDERS.with_borrow_mut(|drawing_recorders| drawing_recorders.pop());
    }
}

thread_local! {
    /// The recorders on this thread that are in the middle of a draw because Graphite called an
    /// [`ImageProvider`] from it. [`Recorder::snap`] must not run on one of them, because it
    /// deletes the deferred canvas and flushes the device that is drawing, and neither must
    /// deleting the recorder, which the draw still uses. A recorder is used on one thread only, so
    /// the list on that thread holds all of its draws. A recorder appears once per nested provider
    /// call, so the list is a stack.
    static DRAWING_RECORDERS: RefCell<Vec<*const sb::skgpu_graphite_Recorder>> =
        const { RefCell::new(Vec::new()) };
}

/// Returns whether `recorder` is in the middle of a draw that called an [`ImageProvider`].
fn is_drawing(recorder: *const sb::skgpu_graphite_Recorder) -> bool {
    DRAWING_RECORDERS.with_borrow(|drawing_recorders| drawing_recorders.contains(&recorder))
}

// `skgpu::graphite::Recorder` is handed out as `std::unique_ptr<Recorder>`
// (Context::makeRecorder) and derives from `SkRecorder`, not `SkRefCnt`. It is
// NOT ref-counted, so it must be a `RefHandle` whose drop `delete`s it.
///
/// Records draws on Graphite surfaces and canvases into [`Recording`]s, which a
/// [`Context`](super::Context) then submits to the GPU.
///
/// # Panics
///
/// Dropping a recorder panics while a surface created from it is still alive, because Skia's
/// draws on that surface use the recorder. Drop every such surface first. In a struct that owns
/// both, declare the surface fields before the recorder field, because Rust drops fields in
/// declaration order. The check snaps the recorder, which prepares GPU resources for the work
/// that was not snapped yet and then discards that work.
///
/// Dropping a recorder also panics while an [`ImageProvider`] runs inside one of its draws.
pub type Recorder = RefHandle<sb::skgpu_graphite_Recorder>;

/// A non-owning, lifetime-bound view of a [`Recorder`] that is owned elsewhere
/// (for example by a Graphite-backed `Canvas`/surface, via `Canvas::recorder`).
///
/// Dropping a `BorrowedRecorder` does **not** `delete` the underlying recorder —
/// the owner keeps that responsibility — and the lifetime ties the borrow to the
/// object it was obtained from, so it cannot dangle.
///
/// Only shared access to the underlying [`Recorder`] is exposed (via `Deref`).
/// Handing out `&mut Recorder` would be unsound: safe code could `mem::replace` a
/// recorder created by `Context::make_recorder` into it and end up owning (and
/// later dropping, i.e. `delete`ing) a recorder the surface still owns.
///
/// [`Recorder::snap`] and [`Recorder::make_deferred_canvas`] stay on the owned
/// [`Recorder`]. A canvas can be the recorder's deferred canvas, which `snap`
/// deletes, so snapping through a recorder reached from a canvas could delete
/// that canvas while it is still in use.
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
}

// Deliberately NOT `Send`/`Sync`: Skia documents a Graphite `Recorder` as
// single-threaded (it and its child objects must be used on one thread), and its
// `&self`/`&mut self` methods drive C++ with no internal lock. Matches the
// Ganesh `gpu::RecordingContext`, which is neither `Send` nor `Sync`.

impl NativeDelete for sb::skgpu_graphite_Recorder {
    /// Panics instead of deleting a recorder that is still in use, in two cases.
    ///
    /// - A draw on the recorder called an [`ImageProvider`] that is still running. The draw
    ///   continues on the recorder after the provider returns.
    /// - A surface still uses the recorder. Skia's draws on such a surface dereference the
    ///   recorder, so the panic leaks the recorder. Later draws on the surface then don't crash,
    ///   but they are never submitted. This check snaps the recorder first, which prepares GPU
    ///   resources for the work that was not snapped yet and then discards that work, as deleting
    ///   the recorder would.
    ///
    /// The draw check runs first, because snapping a recorder in the middle of a draw deletes its
    /// deferred canvas and flushes the device that is drawing.
    unsafe fn delete(ptr: *mut Self) {
        assert!(
            !is_drawing(ptr),
            "a graphite::Recorder must not be dropped while an ImageProvider runs inside one of its draws"
        );
        assert!(
            !unsafe { sb::C_Recorder_snapAndHasLiveDevices(ptr) },
            "a graphite::Recorder was dropped while a surface created from it is still alive"
        );
        unsafe { sb::C_Recorder_delete(ptr) }
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
    ///
    /// # Panics
    /// Panics when an [`ImageProvider`] calls it on a recorder whose draw called that provider,
    /// because Graphite is then in the middle of a draw that `snap` would delete or flush. Snapping
    /// a different recorder from inside a provider is allowed.
    pub fn snap(&mut self) -> Option<Recording> {
        assert!(
            !is_drawing(self.native()),
            "Recorder::snap must not be called while an ImageProvider runs inside one of its draws"
        );
        Recording::from_ptr(unsafe { sb::C_Recorder_snap(self.native_ptr()) })
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
                self.native_ptr(),
                image_info.native(),
                texture_info.native(),
            )
        };
        if canvas_ptr.is_null() {
            None
        } else {
            Some(unsafe { Canvas::borrow_from_native(canvas_ptr) })
        }
    }

    /// Uploads `image` and returns a Graphite-backed version of it with the `required`
    /// properties, or `None` if the upload fails.
    ///
    /// Takes `&self` because an [`ImageProvider`] receives only a shared recorder, and it uploads
    /// while Graphite may already be inside another call to the same provider.
    pub fn texture_from_image(&self, image: &Image, required: RequiredProperties) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImages_TextureFromImageGraphite(
                self.native_mut_force(),
                image.native(),
                required.native(),
            )
        })
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

        struct Provider {
            _alive: Rc<()>,
        }
        impl ImageProvider for Provider {
            fn find_or_create(
                &self,
                _recorder: &BorrowedRecorder,
                _image: &Image,
                _required: RequiredProperties,
            ) -> Option<Image> {
                None
            }
        }

        let alive = Rc::new(());
        let mut options = RecorderOptions::new();
        options.set_image_provider(Provider {
            _alive: alive.clone(),
        });
        assert_eq!(Rc::strong_count(&alive), 2);
        options.set_image_provider(Provider {
            _alive: alive.clone(),
        });
        assert_eq!(
            Rc::strong_count(&alive),
            2,
            "replacing a provider drops the old one"
        );
        drop(options);
        assert_eq!(Rc::strong_count(&alive), 1);
    }
}
