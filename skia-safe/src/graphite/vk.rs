//! Vulkan backend support for Graphite.

use skia_bindings as sb;

use crate::gpu::vk;
use crate::graphite::{Context, ContextOptions};

/// Create a new Graphite [`Context`] for Vulkan rendering.
///
/// Reuses the [`gpu::vk::BackendContext`] binding — the
/// `skgpu::VulkanBackendContext` is shared between Ganesh and Graphite — and
/// hands it to `skgpu::graphite::ContextFactory::MakeVulkan`.
///
/// # Arguments
///
/// - `backend_context` - A Vulkan backend context describing the instance,
///   physical device, device and queue.
/// - `options` - Optional context configuration, defaults to
///   [`ContextOptions::default()`] if `None`.
///
/// # Returns
///
/// A new [`Context`] instance, or `None` if creation failed.
pub fn make_context<'a>(
    backend_context: &vk::BackendContext,
    options: impl Into<Option<&'a ContextOptions>>,
) -> Option<Context> {
    let default_options;
    let options_ptr = match options.into() {
        Some(opts) => opts.native() as *const _,
        None => {
            default_options = ContextOptions::default();
            default_options.native() as *const _
        }
    };

    // SAFETY: `MakeVulkan` queries Vulkan entry points through the backend
    // context's `GetProc`, which skia-safe routes via a thread-local resolver
    // that is only active inside the `begin_resolving()` guard (mirrors
    // `direct_contexts::make_vulkan`). The guard covers the whole FFI call;
    // `options_ptr` is non-null (a default is materialized above); the returned
    // raw pointer is owned and null-checked by `Context::from_ptr`.
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
