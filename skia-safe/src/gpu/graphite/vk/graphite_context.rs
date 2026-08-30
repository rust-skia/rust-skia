use skia_bindings as sb;

use super::super::{Context, ContextOptions};
use crate::gpu::vk as gpu_vk;
use crate::prelude::NativeAccess;

pub mod context_factory {
    use super::*;

    pub fn make_vulkan<'a>(
        backend_context: &gpu_vk::BackendContext,
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
            let end_resolving = backend_context.begin_resolving();
            let context = Context::from_ptr(sb::C_ContextFactory_MakeVulkan(
                backend_context.native.as_ptr() as *const _,
                options_ptr,
            ));
            drop(end_resolving);
            context
        }
    }
}

pub mod contexts {
    use super::*;

    pub fn make_graphite(
        backend_context: &gpu_vk::BackendContext,
        options: &crate::ContextOptions,
    ) -> Option<crate::Context> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = crate::Context::from_ptr(sb::C_SkContexts_MakeGraphiteVulkan(
                backend_context.native.as_ptr() as _,
                options.native(),
            ));
            drop(end_resolving);
            context
        }
    }
}
