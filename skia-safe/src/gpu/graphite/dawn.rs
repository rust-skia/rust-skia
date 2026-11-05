//! Dawn backend support for Graphite.
//!
//! Skia's Dawn backend renders through the WebGPU C API. rust-skia links no WebGPU implementation
//! for it: the `skia-wgpu` crate runs it on the `wgpu` crate (ADR 0003). The functions in here
//! take the WebGPU C handles that such an implementation provides.

use std::fmt;

use skia_bindings as sb;

use super::{BackendTexture, Context, ContextOptions};
use crate::prelude::{self, NativeDrop};

pub use sb::{WGPUDevice, WGPUInstance, WGPUQueue, WGPUTexture};

/// The WebGPU instance, device, and queue that a Graphite context renders with.
pub type BackendContext = prelude::Handle<sb::skgpu_graphite_DawnBackendContext>;

impl NativeDrop for sb::skgpu_graphite_DawnBackendContext {
    fn drop(&mut self) {
        unsafe { sb::C_DawnBackendContext_destruct(self) }
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
    /// The handles must be valid WebGPU handles of the WebGPU implementation that the build links,
    /// which is `skia-wgpu`'s shim. The backend context takes one reference to each and releases
    /// it when it is dropped.
    pub unsafe fn new(instance: WGPUInstance, device: WGPUDevice, queue: WGPUQueue) -> Self {
        BackendContext::construct(|backend_context| unsafe {
            sb::C_DawnBackendContext_Construct(backend_context, instance, device, queue)
        })
    }
}

pub mod context_factory {
    //! Factory functions to create a Graphite [`Context`] on the Dawn backend.

    use skia_bindings as sb;

    use super::{BackendContext, Context, ContextOptions};
    use crate::prelude::*;

    /// Creates a Graphite context that renders with the backend context's device and queue.
    /// Returns `None` if Skia cannot use the device.
    pub fn make_dawn<'a>(
        backend_context: &BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<Context> {
        let default_options;
        let options = match options.into() {
            Some(options) => options,
            None => {
                default_options = ContextOptions::default();
                &default_options
            }
        };
        Context::from_ptr(unsafe {
            sb::C_ContextFactory_MakeDawn(backend_context.native(), options.native())
        })
    }
}

pub mod backend_textures {
    //! Functions to create Graphite [`BackendTexture`]s on the Dawn backend.

    use skia_bindings as sb;

    use super::{BackendTexture, WGPUTexture};

    /// Wraps a WebGPU texture. Its dimensions and format come from the texture.
    ///
    /// # Safety
    ///
    /// `texture` must be a valid WebGPU texture handle, and stay valid for as long as the backend
    /// texture is in use. The backend texture takes no reference to it, but a surface or image that
    /// wraps the backend texture does.
    pub unsafe fn make_dawn(texture: WGPUTexture) -> BackendTexture {
        BackendTexture::construct(|backend_texture| unsafe {
            sb::C_BackendTextures_MakeDawn(backend_texture, texture)
        })
    }
}
