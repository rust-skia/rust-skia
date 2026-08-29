pub mod direct_contexts {
    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext, vk},
        prelude::*,
    };

    pub fn make_vulkan<'a>(
        backend_context: &vk::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = DirectContext::from_ptr(sb::C_GrDirectContexts_MakeVulkan(
                backend_context.native.as_ptr() as _,
                options.into().native_ptr_or_null(),
            ));
            drop(end_resolving);
            context
        }
    }
}

pub mod contexts {
    use skia_bindings as sb;

    use crate::{Context, ContextOptions, gpu::vk, prelude::*};

    pub fn make_ganesh(
        backend_context: &vk::BackendContext,
        options: &ContextOptions,
    ) -> Option<Context> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = Context::from_ptr(sb::C_SkContexts_MakeGaneshVulkan(
                backend_context.native.as_ptr() as _,
                options.native(),
            ));
            drop(end_resolving);
            context
        }
    }
}
