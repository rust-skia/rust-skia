pub mod direct_contexts {
    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext, gl},
        prelude::*,
    };

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
    use skia_bindings as sb;

    use crate::{Context, ContextOptions, gpu::gl, prelude::*};

    pub fn make_ganesh(
        interface: impl Into<gl::Interface>,
        options: &ContextOptions,
    ) -> Option<Context> {
        Context::from_ptr(unsafe {
            sb::C_SkContexts_MakeGaneshGL(interface.into().into_ptr(), options.native())
        })
    }
}
