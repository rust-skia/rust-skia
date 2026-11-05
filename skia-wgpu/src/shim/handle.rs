//! Conversion between the shim's object types and the opaque C handles that Skia holds.
//!
//! A C handle is the pointer of an `Arc<T>`, created with [`Arc::into_raw`]. Each handle type
//! belongs to exactly one shim type, which the [`handles!`] macro records, so a handle can only be
//! read back as the type it was created from.

use std::sync::Arc;

/// A shim type that Skia holds through the C handle type [`Handle::C`].
pub trait Handle: Sized {
    type C;

    /// Moves `self` into a new `Arc` and returns its handle, which owns one reference.
    fn into_handle(self) -> Self::C;

    /// Returns a new handle that owns one more reference to `arc`.
    fn handle_of(arc: &Arc<Self>) -> Self::C;

    /// Borrows the object behind `handle` for as long as Skia's reference keeps it alive.
    ///
    /// # Safety
    ///
    /// `handle` must be a live handle of this type.
    unsafe fn get<'a>(handle: Self::C) -> &'a Self;

    /// Returns a new `Arc` that owns one more reference to the object behind `handle`.
    ///
    /// # Safety
    ///
    /// `handle` must be a live handle of this type.
    unsafe fn arc(handle: Self::C) -> Arc<Self>;
}

/// Implements [`Handle`] for each shim type and exports its WebGPU `AddRef` and `Release`
/// functions, each checked against its function pointer type in `webgpu.h` (`WGPUProc*`).
///
/// Skia shares its handles between threads, and the compiler cannot see that through the raw
/// pointers. The macro therefore asserts that each shim type is `Send` and `Sync`.
///
/// The macro is invoked in the modules next to this one, so it names this module as
/// `super::handle`. The shim source exists once per `wgpu` version, so `$crate` cannot name it.
macro_rules! handles {
    ($(
        $shim:ty => $c:ident:
            $add_ref:ident: $add_ref_proc:ident,
            $release:ident: $release_proc:ident;
    )*) => {
        $(
            const _: () = {
                const fn shared_between_threads<T: Send + Sync>() {}
                shared_between_threads::<$shim>()
            };

            impl super::handle::Handle for $shim {
                type C = skia_bindings::$c;

                fn into_handle(self) -> Self::C {
                    std::sync::Arc::into_raw(std::sync::Arc::new(self)) as Self::C
                }

                fn handle_of(arc: &std::sync::Arc<Self>) -> Self::C {
                    std::sync::Arc::into_raw(arc.clone()) as Self::C
                }

                unsafe fn get<'a>(handle: Self::C) -> &'a Self {
                    assert!(!handle.is_null(), concat!("null ", stringify!($c)));
                    unsafe { &*(handle as *const Self) }
                }

                unsafe fn arc(handle: Self::C) -> std::sync::Arc<Self> {
                    assert!(!handle.is_null(), concat!("null ", stringify!($c)));
                    let ptr = handle as *const Self;
                    unsafe {
                        std::sync::Arc::increment_strong_count(ptr);
                        std::sync::Arc::from_raw(ptr)
                    }
                }
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $add_ref(handle: skia_bindings::$c) {
                unsafe { std::sync::Arc::increment_strong_count(handle as *const $shim) }
            }
            const _: skia_bindings::$add_ref_proc = Some($add_ref);

            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $release(handle: skia_bindings::$c) {
                unsafe { std::sync::Arc::decrement_strong_count(handle as *const $shim) }
            }
            const _: skia_bindings::$release_proc = Some($release);
        )*
    };
}

pub(crate) use handles;

/// Borrows the object behind a handle that may be null.
///
/// # Safety
///
/// `handle` must be null or a live handle of this type.
pub unsafe fn get_optional<'a, T: Handle<C = *mut C>, C>(handle: *mut C) -> Option<&'a T> {
    (!handle.is_null()).then(|| unsafe { T::get(handle) })
}
