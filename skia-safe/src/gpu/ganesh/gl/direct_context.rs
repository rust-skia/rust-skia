pub mod direct_contexts {
    //! Creates a GL-backed [`crate::gpu::DirectContext`].
    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext, gl},
        prelude::*,
    };

    /// Creates a [`DirectContext`] for a backend context. The [`gl::Interface`] must be non-null.
    pub fn make_gl<'a>(
        interface: impl Into<gl::Interface>,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(unsafe {
            sb::C_GrDirectContext_MakeGL(
                interface.into().into_ptr(),
                options.into().native_ptr_or_null(),
            )
        })
    }
}

pub mod contexts {
    //! Creates a [`crate::Context`] wrapping a Ganesh GPU backend with OpenGL.
    use skia_bindings as sb;

    use crate::{Context, ContextOptions, gpu, gpu::gl, prelude::*};

    /// Creates a context wrapping a Ganesh GPU backend with OpenGL
    pub fn make_ganesh<'a>(
        interface: impl Into<gl::Interface>,
        context_options: &ContextOptions,
        gpu_options: impl Into<Option<&'a gpu::ContextOptions>>,
    ) -> Option<Context> {
        Context::from_ptr(unsafe {
            sb::C_SkContexts_MakeGaneshGL(
                interface.into().into_ptr(),
                context_options.native(),
                gpu_options.into().native_ptr_or_null(),
            )
        })
    }
}
