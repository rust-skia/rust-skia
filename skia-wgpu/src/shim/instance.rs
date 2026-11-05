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
    Instance => WGPUInstance:
        wgpuInstanceAddRef: WGPUProcInstanceAddRef,
        wgpuInstanceRelease: WGPUProcInstanceRelease;
    Adapter => WGPUAdapter:
        wgpuAdapterAddRef: WGPUProcAdapterAddRef,
        wgpuAdapterRelease: WGPUProcAdapterRelease;
}

impl Instance {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            futures: Arc::default(),
            device: device.clone(),
        }
    }

    /// Lets `wgpu` report completed work. With `wait`, blocks until the submitted work is done.
    ///
    /// Callers that do not wait need not look at the result, because a callback that the poll
    /// does not fire keeps waiting until its instance is released.
    ///
    /// On a lost device, `wgpu` panics in the poll, which aborts the process inside the WebGPU
    /// function that polls. `wgpu` treats the loss as fatal, so the shim does too (ADR 0003).
    /// The poll does not catch the panic, because `wgpu` fires every pending callback inside the
    /// poll: the shim's, Skia's, and the application's. A catch would also swallow a panic of
    /// those callbacks, mistake it for a lost device, and continue with half-updated state.
    pub fn poll(&self, wait: bool) -> Polled {
        loop {
            let poll_type = if wait {
                wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: None,
                }
            } else {
                wgpu::PollType::Poll
            };
            match self.device.poll(poll_type) {
                Ok(_) => return Polled::Complete,
                // `wgpu`'s GL backend gives up a wait after about 2 s, so the wait starts again.
                // On WebGL, a wait gives up at once, and the work cannot progress while it repeats.
                Err(wgpu::PollError::Timeout) if !cfg!(target_family = "wasm") => {}
                Err(wgpu::PollError::Timeout) => return Polled::Incomplete,
                Err(error @ wgpu::PollError::WrongSubmissionIndex(..)) => {
                    panic!("polling the wgpu device failed: {error}")
                }
            }
        }
    }
}

/// How far a poll of the device got.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polled {
    /// `wgpu` reported every completion that the poll covers: the work that is already done, or
    /// for a wait, all submitted work.
    Complete,
    /// A wait on WebGL gave up before the submitted work was done.
    Incomplete,
}

impl Adapter {
    pub fn new(info: wgpu::AdapterInfo) -> Self {
        Self { info }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuInstanceWaitAny(
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
    // Skia waits without a timeout or not at all, so the shim implements no other timeout.
    assert_eq!(
        timeout_ns,
        u64::MAX,
        "wgpuInstanceWaitAny with a finite timeout"
    );
    let polled = instance.poll(true);
    if instance.futures.fire_waited(infos) {
        return S::WGPUWaitStatus_Success;
    }
    // A complete wait returns once all submitted work is done, and `wgpu` has then reported every
    // completion it will report, so a future that is still pending would never complete.
    assert_eq!(
        polled,
        Polled::Incomplete,
        "wgpuInstanceWaitAny waits for futures that cannot complete"
    );
    S::WGPUWaitStatus_Error
}
const _: sb::WGPUProcInstanceWaitAny = Some(wgpuInstanceWaitAny);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuInstanceProcessEvents(instance: sb::WGPUInstance) {
    // Owns a reference, because a callback may release the last handle to the instance.
    let instance = unsafe { Instance::arc(instance) };
    instance.poll(false);
    instance.futures.fire_process_events();
}
const _: sb::WGPUProcInstanceProcessEvents = Some(wgpuInstanceProcessEvents);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuAdapterGetInfo(
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
const _: sb::WGPUProcAdapterGetInfo = Some(wgpuAdapterGetInfo);

/// Frees the strings that [`wgpuAdapterGetInfo`] allocated.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuAdapterInfoFreeMembers(info: sb::WGPUAdapterInfo) {
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
const _: sb::WGPUProcAdapterInfoFreeMembers = Some(wgpuAdapterInfoFreeMembers);

/// Creates the handle of a new adapter for the device's adapter information.
pub fn adapter_handle(device: &wgpu::Device) -> sb::WGPUAdapter {
    Adapter::new(device.adapter_info()).into_handle()
}

/// A string that the receiver frees with [`wgpuAdapterInfoFreeMembers`].
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
    use super::super::buffer::{wgpuBufferMapAsync, wgpuBufferRelease, wgpuDeviceCreateBuffer};
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
        let buffer = unsafe { wgpuDeviceCreateBuffer(handles.device(), &descriptor) };
        let mut statuses: Vec<sb::WGPUMapAsyncStatus> = Vec::new();
        let callback_info = sb::WGPUBufferMapCallbackInfo {
            nextInChain: ptr::null_mut(),
            mode: sb::WGPUCallbackMode::WGPUCallbackMode_WaitAnyOnly,
            callback: Some(record),
            userdata1: (&raw mut statuses).cast(),
            userdata2: ptr::null_mut(),
        };
        unsafe { wgpuBufferMapAsync(buffer, sb::WGPUMapMode_Read, 0, usize::MAX, callback_info) };
        // The map completes, but its callback may fire only inside `wgpuInstanceWaitAny`, which
        // nobody calls.
        device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .expect("polling the device failed");

        unsafe { wgpuBufferRelease(buffer) };
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
