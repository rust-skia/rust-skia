pub mod direct_contexts {
    //! Creates a Metal-backed [`crate::gpu::DirectContext`].

    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext, mtl},
        prelude::*,
    };

    /// Makes a [`DirectContext`] which uses Metal as the backend. The [`mtl::BackendContext`] contains a
    /// MTLDevice and MTLCommandQueue which should be used by the backend. These objects must
    /// have their own ref which will be released when the [`mtl::BackendContext`] is destroyed.
    /// Ganesh will take its own ref on the objects which will be released when the [`DirectContext`]
    /// is destroyed.
    pub fn make_metal<'a>(
        backend: &mtl::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(unsafe {
            sb::C_GrContext_MakeMetal(backend.native(), options.into().native_ptr_or_null())
        })
    }
}

pub mod contexts {
    //! Creates a [`crate::Context`] wrapping a Ganesh GPU backend with Metal.
    use skia_bindings as sb;

    use crate::{Context, ContextOptions, gpu, gpu::mtl, prelude::*};

    /// Creates a context wrapping a Ganesh GPU backend with Metal
    pub fn make_ganesh<'a>(
        backend: &mtl::BackendContext,
        context_options: &ContextOptions,
        gpu_options: impl Into<Option<&'a gpu::ContextOptions>>,
    ) -> Option<Context> {
        Context::from_ptr(unsafe {
            sb::C_SkContexts_MakeGaneshMetal(
                backend.native(),
                context_options.native(),
                gpu_options.into().native_ptr_or_null(),
            )
        })
    }
}
