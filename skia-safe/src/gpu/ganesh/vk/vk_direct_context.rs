pub mod direct_contexts {
    //! Creates a Vulkan-backed [`crate::gpu::DirectContext`].
    use skia_bindings as sb;

    use crate::{
        gpu::{ContextOptions, DirectContext, vk},
        prelude::*,
    };

    /// The Vulkan context ([`vk::Queue`], [`vk::Device`], [`vk::Instance`]) must be kept alive
    /// until the returned [`DirectContext`] is destroyed. This also means that any objects
    /// created with this [`DirectContext`] (e.g. [`crate::Surface`]s, [`crate::Image`]s, etc.)
    /// must also be released as they may hold refs on the [`DirectContext`]. Once all these
    /// objects and the [`DirectContext`] are released, then it is safe to delete the vulkan
    /// objects.
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
    //! Creates a [`crate::Context`] wrapping a Ganesh GPU backend with Vulkan.
    use skia_bindings as sb;

    use crate::{Context, ContextOptions, gpu, gpu::vk, prelude::*};

    /// Creates a context wrapping a Ganesh GPU backend with Vulkan
    pub fn make_ganesh<'a>(
        backend_context: &vk::BackendContext,
        context_options: &ContextOptions,
        gpu_options: impl Into<Option<&'a gpu::ContextOptions>>,
    ) -> Option<Context> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = Context::from_ptr(sb::C_SkContexts_MakeGaneshVulkan(
                backend_context.native.as_ptr() as _,
                context_options.native(),
                gpu_options.into().native_ptr_or_null(),
            ));
            drop(end_resolving);
            context
        }
    }
}
