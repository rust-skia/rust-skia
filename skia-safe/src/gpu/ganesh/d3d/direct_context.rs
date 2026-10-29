pub mod direct_contexts {
    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext, d3d},
        prelude::*,
    };

    #[allow(clippy::missing_safety_doc)]
    /// Makes a [`DirectContext`] which uses Direct3D as the backend. The Direct3D context
    /// must be kept alive until the returned [`DirectContext`] is first destroyed or abandoned.
    pub unsafe fn make_d3d<'a>(
        backend_context: &d3d::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        unsafe {
            DirectContext::from_ptr(sb::C_GrDirectContexts_MakeD3D(
                backend_context.native(),
                options.into().native_ptr_or_null(),
            ))
        }
    }
}

pub mod contexts {
    use skia_bindings as sb;

    use crate::{Context, ContextOptions, gpu, gpu::d3d, prelude::*};

    #[allow(clippy::missing_safety_doc)]
    /// Creates a context wrapping a Ganesh GPU backend with D3D
    pub unsafe fn make_ganesh<'a>(
        backend_context: &d3d::BackendContext,
        context_options: &ContextOptions,
        gpu_options: impl Into<Option<&'a gpu::ContextOptions>>,
    ) -> Option<Context> {
        unsafe {
            Context::from_ptr(sb::C_SkContexts_MakeGaneshD3D(
                backend_context.native(),
                context_options.native(),
                gpu_options.into().native_ptr_or_null(),
            ))
        }
    }
}
