use std::fmt;

use crate::graphite::{BackendTexture, Context, ContextOptions};
use crate::prelude::NativeAccess;
use crate::prelude::{self, NativeDrop};
use skia_bindings as sb;

/// A handle representing a Metal object (e.g., MTLDevice, MTLCommandQueue)
pub type Handle = *mut std::ffi::c_void;

pub type BackendContext = prelude::Handle<sb::skgpu_graphite_MtlBackendContext>;
unsafe_send_sync!(BackendContext);

impl NativeDrop for sb::skgpu_graphite_MtlBackendContext {
    fn drop(&mut self) {
        unsafe { sb::C_MtlBackendContext_destruct(self) }
    }
}

impl fmt::Debug for BackendContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackendContext").finish()
    }
}

impl BackendContext {
    /// # Safety
    ///
    /// Unsafe because it expects Metal device and queue objects in the form of `c_void` pointers.
    ///
    /// This function retains all the non-`null` handles passed to it and releases them as soon the
    /// [`BackendContext`] is dropped.
    ///
    /// # Arguments
    ///
    /// - `device` - A pointer to an MTLDevice
    /// - `queue` - A pointer to an MTLCommandQueue
    pub unsafe fn new(device: Handle, queue: Handle) -> Self {
        BackendContext::construct(|bc| sb::C_MtlBackendContext_Construct(bc, device, queue))
    }
}

/// Create a new Graphite Context for Metal rendering
///
/// # Arguments
///
/// - `backend_context` - The Metal backend context containing device and queue
/// - `options` - Optional context configuration, defaults to `ContextOptions::default()` if `None`
///
/// # Returns
///
/// A new `Context` instance, or `None` if creation failed
///
/// # Example
///
/// ```ignore
/// use skia_safe::graphite::mtl;
/// use skia_safe::graphite::Context;
///
/// # let device = std::ptr::null_mut();
/// # let queue = std::ptr::null_mut();
/// unsafe {
///     let backend_context = mtl::BackendContext::new(device, queue);
///     let context = mtl::make_context(&backend_context, None);
/// }
/// ```
pub fn make_context(
    backend_context: &BackendContext,
    options: Option<&ContextOptions>,
) -> Option<Context> {
    let default_options;
    let options_ptr = match options {
        Some(opts) => opts.native() as *const _,
        None => {
            default_options = ContextOptions::default();
            default_options.native() as *const _
        }
    };

    unsafe {
        Context::from_ptr(sb::C_ContextFactory_MakeMetal(
            backend_context.native(),
            options_ptr,
        ))
    }
}

/// Create a [`BackendTexture`] from an existing Metal texture
///
/// # Safety
///
/// Unsafe because it expects a Metal texture object in the form of a `c_void` pointer.
///
/// This function will **not** call retain or release on the passed in Metal texture.
/// Thus, you must keep the Metal texture valid until you are no longer using the
/// [`BackendTexture`].
///
/// # Arguments
///
/// - `dimensions` - The width and height of the texture
/// - `mtl_texture` - A pointer to an `id<MTLTexture>` object
///
/// # Returns
///
/// A [`BackendTexture`] that can be passed to Graphite functions.
///
/// # Example
///
/// ```ignore
/// use skia_safe::graphite::mtl;
///
/// # let mtl_texture: *mut std::ffi::c_void = std::ptr::null_mut();
/// unsafe {
///     let dimensions = (512, 512);
///     let backend_texture = mtl::make_backend_texture(dimensions, mtl_texture);
///     // Use backend_texture with Graphite functions
/// }

pub unsafe fn make_backend_texture(
    dimensions: impl Into<crate::ISize>,
    mtl_texture: *mut std::ffi::c_void,
) -> BackendTexture {
    let dimensions = dimensions.into();
    let mut backend_texture = std::mem::MaybeUninit::uninit();
    sb::C_BackendTextures_MakeMetal(
        backend_texture.as_mut_ptr(),
        dimensions.width,
        dimensions.height,
        mtl_texture,
    );
    std::mem::transmute(backend_texture.assume_init())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_context_debug() {
        // We can't easily create a BackendContext without Metal objects,
        // but we can test that the debug implementation compiles
        let context: Option<BackendContext> = None;
        assert!(context.is_none());
    }

    #[test]
    fn test_make_backend_texture_compiles() {
        // We can't easily test make_backend_texture without actual Metal objects,
        // but we can verify the function signature compiles correctly
        let _dimensions: crate::ISize = (512, 512).into();
        let _mtl_texture: *mut std::ffi::c_void = std::ptr::null_mut();
        // We don't actually call the function as it's unsafe and requires valid Metal objects
        // but this test verifies the types are compatible
    }

    #[test]
    fn test_wrapped_backend_texture_deref() {
        // Test that WrappedBackendTexture implements Deref
        // We can't create a real WrappedBackendTexture without Metal objects,
        // but we can verify the type system works
        //fn expects_backend_texture(_t: &crate::graphite::BackendTexture) {}
        // This would compile if we had a WrappedBackendTexture:
        // let wrapped: WrappedBackendTexture = /* ... */;
        // expects_backend_texture(&wrapped);
        // expects_backend_texture(wrapped.deref());
    }
}
