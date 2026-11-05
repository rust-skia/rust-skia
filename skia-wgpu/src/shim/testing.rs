//! What the shim's tests share.

use super::future::block_on;
use super::wgpu;

/// Creates a device and its queue on the default adapter.
///
/// Returns `None` and prints a note on a machine without an adapter, such as a CI runner without a
/// GPU. The test then passes without checking anything.
pub fn device() -> Option<(wgpu::Device, wgpu::Queue)> {
    device_with(wgpu::Features::empty())
}

/// Like [`device`], with the `features` enabled. Returns `None` and prints a note also when the
/// adapter lacks one of them.
pub fn device_with(features: wgpu::Features) -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::default();
    let Ok(adapter) = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
    else {
        eprintln!("skipped: no wgpu adapter available");
        return None;
    };
    if !adapter.features().contains(features) {
        eprintln!("skipped: the wgpu adapter lacks {features:?}");
        return None;
    }
    let device = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: features,
        ..Default::default()
    }))
    .expect("no wgpu device");
    Some(device)
}
