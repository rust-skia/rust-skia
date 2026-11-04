//! The handles that let Skia's Dawn backend use an application's `wgpu` device and textures.

use std::fmt;
use std::sync::Arc;

use skia_bindings as sb;

use super::device::Device;
use super::handle::Handle;
use super::instance::{self, Instance};
use super::queue::Queue;
use super::texture::Texture;
use super::{table, wgpu};

/// The WebGPU handles of an application's `wgpu` device and queue, which Skia's
/// `DawnBackendContext` takes. Each handle owns one reference, which `Drop` releases.
pub struct BackendHandles {
    instance: sb::WGPUInstance,
    device: sb::WGPUDevice,
    queue: sb::WGPUQueue,
}

impl BackendHandles {
    /// Wraps the device and the queue and installs the shim's function table, so that Skia's
    /// WebGPU calls reach them.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        table::install();
        let instance = Arc::new(Instance::new(device));
        Self {
            device: Device::new(device.clone(), instance.clone()).into_handle(),
            queue: Queue {
                queue: queue.clone(),
                instance: instance.clone(),
            }
            .into_handle(),
            instance: Instance::handle_of(&instance),
        }
    }

    pub fn instance(&self) -> sb::WGPUInstance {
        self.instance
    }

    pub fn device(&self) -> sb::WGPUDevice {
        self.device
    }

    pub fn queue(&self) -> sb::WGPUQueue {
        self.queue
    }
}

impl Drop for BackendHandles {
    fn drop(&mut self) {
        unsafe {
            super::queue::queue_release(self.queue);
            super::device::device_release(self.device);
            instance::instance_release(self.instance);
        }
    }
}

impl fmt::Debug for BackendHandles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackendHandles").finish_non_exhaustive()
    }
}

/// The WebGPU handle of an application's `wgpu` texture, which Skia wraps as a backend texture.
/// It owns one reference, which `Drop` releases.
pub struct TextureHandle(sb::WGPUTexture);

impl TextureHandle {
    pub fn new(texture: &wgpu::Texture) -> Self {
        Self(
            Texture {
                texture: texture.clone(),
            }
            .into_handle(),
        )
    }

    pub fn handle(&self) -> sb::WGPUTexture {
        self.0
    }
}

impl Drop for TextureHandle {
    fn drop(&mut self) {
        unsafe { super::texture::texture_release(self.0) }
    }
}

impl fmt::Debug for TextureHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TextureHandle").finish_non_exhaustive()
    }
}
