use std::fmt;

use skia_bindings as sb;

use crate::graphite::{BackendTexture, Context, ContextOptions};
use crate::prelude::NativeAccess;
use crate::prelude::{self, NativeDrop};

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
        BackendContext::construct(|bc| unsafe {
            sb::C_MtlBackendContext_Construct(bc, device, queue)
        })
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
pub fn make_context<'a>(
    backend_context: &BackendContext,
    options: impl Into<Option<&'a ContextOptions>>,
) -> Option<Context> {
    let default_options;
    let options_ptr = match options.into() {
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

pub mod contexts {
    use skia_bindings as sb;

    use super::BackendContext;
    use crate::{Context, ContextOptions, prelude::*};

    pub fn make_graphite(
        backend_context: &BackendContext,
        options: &ContextOptions,
    ) -> Option<Context> {
        Context::from_ptr(unsafe {
            sb::C_SkContexts_MakeGraphiteMetal(backend_context.native(), options.native())
        })
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
/// ```
pub unsafe fn make_backend_texture(
    dimensions: impl Into<crate::ISize>,
    mtl_texture: *mut std::ffi::c_void,
) -> BackendTexture {
    let dimensions = dimensions.into();
    BackendTexture::construct(|backend_texture| unsafe {
        sb::C_BackendTextures_MakeMetal(
            backend_texture,
            dimensions.width,
            dimensions.height,
            mtl_texture,
        )
    })
}
