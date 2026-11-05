//! The instance, which owns the futures and polls the devices while Skia waits, and the adapter.

use std::ffi::CString;
use std::sync::Arc;

use skia_bindings as sb;

use super::future::Futures;
use super::handle::{Handle, handles};
use super::{convert, wgpu};

/// Behind a `WGPUInstance`: the futures of a device, which `wgpuInstanceWaitAny` completes.
#[derive(Debug)]
pub struct Instance {
    pub futures: Arc<Futures>,
    /// The device whose work completes the futures. `wgpu` reports completions only while the
    /// device is polled, and it runs the callbacks inside the poll, so no lock may be held then.
    device: wgpu::Device,
}

/// The adapter of a device. `wgpu` 30 reports the adapter information through the device, so the
/// shim needs no `wgpu::Adapter`.
#[derive(Debug)]
pub struct Adapter {
    info: wgpu::AdapterInfo,
}

handles! {
    Instance => WGPUInstance: instance_add_ref, instance_release;
    Adapter => WGPUAdapter: adapter_add_ref, adapter_release;
}

impl Instance {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            futures: Arc::default(),
            device: device.clone(),
        }
    }

    /// Lets `wgpu` report completed work. With `wait`, blocks until the submitted work is done.
    pub fn poll(&self, wait: bool) {
        let poll_type = if wait {
            wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            }
        } else {
            wgpu::PollType::Poll
        };
        self.device
            .poll(poll_type)
            .expect("polling the wgpu device failed");
    }
}

impl Adapter {
    pub fn new(info: wgpu::AdapterInfo) -> Self {
        Self { info }
    }
}

pub unsafe extern "C" fn instance_wait_any(
    instance: sb::WGPUInstance,
    future_count: usize,
    futures: *mut sb::WGPUFutureWaitInfo,
    timeout_ns: u64,
) -> sb::WGPUWaitStatus {
    use sb::WGPUWaitStatus as S;
    // The wait owns a reference, because a callback may release the last handle to the instance.
    let instance = unsafe { Instance::arc(instance) };
    assert!(future_count > 0, "wgpuInstanceWaitAny without futures");
    let infos = unsafe { std::slice::from_raw_parts_mut(futures, future_count) };
    if instance.futures.fire_waited(infos) {
        return S::WGPUWaitStatus_Success;
    }
    if timeout_ns == 0 {
        instance.poll(false);
        return if instance.futures.fire_waited(infos) {
            S::WGPUWaitStatus_Success
        } else {
            S::WGPUWaitStatus_TimedOut
        };
    }
    // Skia waits without a timeout or not at all, so any other timeout waits without one. The
    // wait returns once all submitted work is done, and `wgpu` has then reported every completion
    // it will report, so a future that is still pending would never complete.
    instance.poll(true);
    assert!(
        instance.futures.fire_waited(infos),
        "wgpuInstanceWaitAny waits for futures that cannot complete"
    );
    S::WGPUWaitStatus_Success
}

pub unsafe extern "C" fn instance_process_events(instance: sb::WGPUInstance) {
    // Owns a reference, because a callback may release the last handle to the instance.
    let instance = unsafe { Instance::arc(instance) };
    instance.poll(false);
    instance.futures.fire_process_events();
}

pub unsafe extern "C" fn adapter_get_info(
    adapter: sb::WGPUAdapter,
    info: *mut sb::WGPUAdapterInfo,
) -> sb::WGPUStatus {
    let adapter = unsafe { Adapter::get(adapter) };
    let info = unsafe { &mut *info };
    unsafe { convert::no_chain(info.nextInChain, "WGPUAdapterInfo") };
    let w = &adapter.info;
    info.vendor = owned_string("");
    info.architecture = owned_string("");
    info.device = owned_string(&w.name);
    info.description = owned_string(&w.driver_info);
    info.backendType = backend_type(w.backend);
    info.adapterType = adapter_type(w.device_type);
    info.vendorID = w.vendor;
    info.deviceID = w.device;
    info.subgroupMinSize = 0;
    info.subgroupMaxSize = 0;
    sb::WGPUStatus::WGPUStatus_Success
}

/// Frees the strings that [`adapter_get_info`] allocated.
pub unsafe extern "C" fn adapter_info_free_members(info: sb::WGPUAdapterInfo) {
    for view in [
        info.vendor,
        info.architecture,
        info.device,
        info.description,
    ] {
        if !view.data.is_null() {
            drop(unsafe { CString::from_raw(view.data as *mut _) });
        }
    }
}

/// Creates the handle of a new adapter for the device's adapter information.
pub fn adapter_handle(device: &wgpu::Device) -> sb::WGPUAdapter {
    Adapter::new(device.adapter_info()).into_handle()
}

/// A string that the receiver frees with [`adapter_info_free_members`].
fn owned_string(s: &str) -> sb::WGPUStringView {
    let length = s.len();
    sb::WGPUStringView {
        data: CString::new(s.replace('\0', " ")).unwrap().into_raw(),
        length,
    }
}

fn backend_type(backend: wgpu::Backend) -> sb::WGPUBackendType {
    use sb::WGPUBackendType as B;
    match backend {
        wgpu::Backend::Noop => B::WGPUBackendType_Null,
        wgpu::Backend::Vulkan => B::WGPUBackendType_Vulkan,
        wgpu::Backend::Metal => B::WGPUBackendType_Metal,
        wgpu::Backend::Dx12 => B::WGPUBackendType_D3D12,
        wgpu::Backend::Gl => B::WGPUBackendType_OpenGLES,
        wgpu::Backend::BrowserWebGpu => B::WGPUBackendType_WebGPU,
    }
}

fn adapter_type(device_type: wgpu::DeviceType) -> sb::WGPUAdapterType {
    use sb::WGPUAdapterType as A;
    match device_type {
        wgpu::DeviceType::DiscreteGpu => A::WGPUAdapterType_DiscreteGPU,
        wgpu::DeviceType::IntegratedGpu => A::WGPUAdapterType_IntegratedGPU,
        wgpu::DeviceType::Cpu => A::WGPUAdapterType_CPU,
        wgpu::DeviceType::Other | wgpu::DeviceType::VirtualGpu => A::WGPUAdapterType_Unknown,
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::ptr;

    use skia_bindings as sb;

    use super::super::api::BackendHandles;
    use super::super::buffer::{buffer_map_async, buffer_release, device_create_buffer};
    use super::super::{convert, testing, wgpu};

    /// WebGPU calls every callback exactly once. A callback that has not fired when its instance
    /// is released fires with `CallbackCancelled`, so that the caller can free what the callback
    /// owns, such as the lambda that Dawn's C++ wrapper allocates for each callback.
    #[test]
    fn releasing_the_instance_cancels_a_callback_that_has_not_fired() {
        let Some((device, queue)) = testing::device() else {
            return;
        };
        let handles = BackendHandles::new(&device, &queue);
        let descriptor = sb::WGPUBufferDescriptor {
            nextInChain: ptr::null_mut(),
            label: convert::empty_string(),
            usage: sb::WGPUBufferUsage_MapRead,
            size: 256,
            mappedAtCreation: 0,
        };
        let buffer = unsafe { device_create_buffer(handles.device(), &descriptor) };
        let mut statuses: Vec<sb::WGPUMapAsyncStatus> = Vec::new();
        let callback_info = sb::WGPUBufferMapCallbackInfo {
            nextInChain: ptr::null_mut(),
            mode: sb::WGPUCallbackMode::WGPUCallbackMode_WaitAnyOnly,
            callback: Some(record),
            userdata1: (&raw mut statuses).cast(),
            userdata2: ptr::null_mut(),
        };
        unsafe { buffer_map_async(buffer, sb::WGPUMapMode_Read, 0, usize::MAX, callback_info) };
        // The map completes, but its callback may fire only inside `wgpuInstanceWaitAny`, which
        // nobody calls.
        device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .expect("polling the device failed");

        unsafe { buffer_release(buffer) };
        drop(handles);

        assert_eq!(
            statuses,
            [sb::WGPUMapAsyncStatus::WGPUMapAsyncStatus_CallbackCancelled]
        );
    }

    unsafe extern "C" fn record(
        status: sb::WGPUMapAsyncStatus,
        _: sb::WGPUStringView,
        statuses: *mut c_void,
        _: *mut c_void,
    ) {
        unsafe { (*statuses.cast::<Vec<sb::WGPUMapAsyncStatus>>()).push(status) };
    }
}
