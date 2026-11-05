//! The handles that let Skia's Dawn backend use an application's `wgpu` device and textures.

use std::fmt;
use std::sync::Arc;

use skia_bindings as sb;

use super::device::Device;
use super::handle::Handle;
use super::instance::{self, Instance};
use super::queue::Queue;
use super::texture::Texture;
use super::{convert, wgpu};

/// The WebGPU handles of an application's `wgpu` device and queue, which Skia's
/// `DawnBackendContext` takes. Each handle owns one reference, which `Drop` releases.
pub struct BackendHandles {
    instance: sb::WGPUInstance,
    device: sb::WGPUDevice,
    queue: sb::WGPUQueue,
}

impl BackendHandles {
    /// Wraps the device and the queue, so that Skia's WebGPU calls reach them.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
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

    /// The instance handle, borrowed from `self`. Skia's backend context takes its own reference.
    pub fn instance(&self) -> sb::WGPUInstance {
        self.instance
    }

    /// The device handle, borrowed from `self`. Skia's backend context takes its own reference.
    pub fn device(&self) -> sb::WGPUDevice {
        self.device
    }

    /// The queue handle, borrowed from `self`. Skia's backend context takes its own reference.
    pub fn queue(&self) -> sb::WGPUQueue {
        self.queue
    }
}

impl Drop for BackendHandles {
    fn drop(&mut self) {
        unsafe {
            super::queue::wgpuQueueRelease(self.queue);
            super::device::wgpuDeviceRelease(self.device);
            instance::wgpuInstanceRelease(self.instance);
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
    /// Wraps the texture. Fails for a format that has no WebGPU equivalent, which Skia could not
    /// query.
    pub fn new(texture: &wgpu::Texture) -> Result<Self, UnsupportedTextureFormat> {
        let format = convert::c_texture_format(texture.format())
            .ok_or(UnsupportedTextureFormat(texture.format()))?;
        Ok(Self(
            Texture {
                texture: texture.clone(),
                format,
            }
            .into_handle(),
        ))
    }

    /// The texture handle, borrowed from `self`. Skia's backend texture takes no reference, so
    /// `self` must outlive it. Surfaces and images that wrap the backend texture take their own.
    pub fn handle(&self) -> sb::WGPUTexture {
        self.0
    }
}

impl Drop for TextureHandle {
    fn drop(&mut self) {
        unsafe { super::texture::wgpuTextureRelease(self.0) }
    }
}

impl fmt::Debug for TextureHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TextureHandle").finish_non_exhaustive()
    }
}

/// The error of [`TextureHandle::new`] for a texture whose format has no WebGPU equivalent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedTextureFormat(pub wgpu::TextureFormat);

impl fmt::Display for UnsupportedTextureFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the texture format {:?} has no WebGPU equivalent",
            self.0
        )
    }
}

impl std::error::Error for UnsupportedTextureFormat {}
