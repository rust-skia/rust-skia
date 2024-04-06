pub mod direct_contexts {

    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext},
        prelude::*,
    };

    pub fn make_metal<'a>(
        backend: &crate::gpu::mtl::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(unsafe {
            sb::C_GrContext_MakeMetal(backend.native(), options.into().native_ptr_or_null())
        })
    }
}
