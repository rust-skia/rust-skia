use std::fmt;

use skia_bindings as sb;

use super::super::{Context, ContextOptions};
use crate::prelude::{self, NativeDrop};

/// A handle representing a Metal object (e.g., MTLDevice, MTLCommandQueue).
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
    /// `device` and `queue` must point to valid Metal objects. This function
    /// retains non-null handles and releases them when the context is dropped.
    pub unsafe fn new(device: Handle, queue: Handle) -> Self {
        BackendContext::construct(|backend_context| unsafe {
            sb::C_MtlBackendContext_Construct(backend_context, device, queue)
        })
    }
}

pub mod context_factory {
    use skia_bindings as sb;

    use super::{BackendContext, Context, ContextOptions};
    use crate::prelude::*;

    pub fn make_metal<'a>(
        backend_context: &BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<Context> {
        let default_options;
        let options_ptr = match options.into() {
            Some(options) => options.native() as *const _,
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
