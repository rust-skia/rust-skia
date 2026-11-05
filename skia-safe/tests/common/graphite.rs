//! A Graphite context on the platform's own GPU API for tests: Metal on macOS, Vulkan elsewhere.
//! The tests that use it check Graphite's backend-independent behavior, so the backend that runs
//! them does not matter. Tests skip when no GPU device is available.

// Each test target compiles its own copy of this module, and not every one uses it.
#![allow(dead_code)]

use skia_safe::gpu::graphite;

/// A Graphite context together with the GPU objects it was created from, which must outlive it.
pub struct Graphite {
    pub context: graphite::Context,
    // Declared after `context`, so that it is dropped after it.
    _device: Device,
}

/// The Graphite Metal context retains the device and the queue, so nothing else needs to live on.
#[cfg(all(target_os = "macos", feature = "metal"))]
type Device = ();

#[cfg(not(all(target_os = "macos", feature = "metal")))]
type Device = super::vulkan::Vulkan;

impl Graphite {
    #[cfg(all(target_os = "macos", feature = "metal"))]
    pub fn new() -> Option<Self> {
        use objc2::rc::Retained;
        use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

        let device = MTLCreateSystemDefaultDevice()?;
        let queue = device.newCommandQueue()?;
        let backend = unsafe {
            graphite::mtl::BackendContext::new(
                Retained::as_ptr(&device) as graphite::mtl::Handle,
                Retained::as_ptr(&queue) as graphite::mtl::Handle,
            )
        };
        let context = graphite::mtl::context_factory::make_metal(&backend, None)?;
        Some(Self {
            context,
            _device: (),
        })
    }

    #[cfg(not(all(target_os = "macos", feature = "metal")))]
    pub fn new() -> Option<Self> {
        let vulkan = super::vulkan::Vulkan::new()?;
        let context = vulkan.graphite_context()?;
        Some(Self {
            context,
            _device: vulkan,
        })
    }
}
