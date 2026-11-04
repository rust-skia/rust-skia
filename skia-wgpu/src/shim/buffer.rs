//! Buffers and their mapping.
//!
//! WebGPU hands out raw pointers into a mapped buffer (`wgpuBufferGetMappedRange`) that stay valid
//! until `wgpuBufferUnmap`. `wgpu` hands out views instead, and the mapping ends when the last
//! view is dropped. The shim therefore keeps one view over the whole mapped range for as long as
//! the buffer is mapped, and returns pointers into it.

use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use skia_bindings as sb;

use super::device::Device;
use super::future::Userdata;
use super::handle::{Handle, handles};
use super::instance::Instance;
use super::{convert, wgpu};

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    instance: Arc<Instance>,
    mapping: Mutex<Mapping>,
}

handles! {
    Buffer => WGPUBuffer: buffer_add_ref, buffer_release;
}

pub unsafe extern "C" fn device_create_buffer(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUBufferDescriptor,
) -> sb::WGPUBuffer {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUBufferDescriptor") };
    let mapped_at_creation = convert::bool(d.mappedAtCreation);
    let buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
        label: unsafe { convert::label(d.label) },
        size: d.size,
        usage: convert::buffer_usages(d.usage),
        mapped_at_creation,
    });
    let mapping = if mapped_at_creation {
        Mapping::Mapped {
            mode: wgpu::MapMode::Write,
            offset: 0,
            size: d.size,
            view: None,
        }
    } else {
        Mapping::Unmapped
    };
    Buffer {
        buffer,
        instance: device.instance.clone(),
        mapping: Mutex::new(mapping),
    }
    .into_handle()
}

pub unsafe extern "C" fn buffer_map_async(
    buffer: sb::WGPUBuffer,
    mode: sb::WGPUMapMode,
    offset: usize,
    size: usize,
    callback_info: sb::WGPUBufferMapCallbackInfo,
) -> sb::WGPUFuture {
    use sb::WGPUMapAsyncStatus as S;
    unsafe { convert::no_chain(callback_info.nextInChain, "WGPUBufferMapCallbackInfo") };
    let buffer = unsafe { Buffer::arc(buffer) };
    let mode = convert::map_mode(mode);
    let offset = offset as u64;
    let size = if size == usize::MAX {
        buffer.buffer.size() - offset
    } else {
        size as u64
    };
    let (future, completion) = buffer.instance.futures.register(callback_info.mode);
    let callback = callback_info.callback.expect("null WGPUBufferMapCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    *buffer.mapping.lock().unwrap() = Mapping::Pending;
    let mapped = buffer.clone();
    buffer
        .buffer
        .map_async(mode, offset..offset + size, move |result| {
            let status = match result {
                Ok(()) => {
                    *mapped.mapping.lock().unwrap() = Mapping::Mapped {
                        mode,
                        offset,
                        size,
                        view: None,
                    };
                    S::WGPUMapAsyncStatus_Success
                }
                Err(_) => {
                    *mapped.mapping.lock().unwrap() = Mapping::Unmapped;
                    S::WGPUMapAsyncStatus_Aborted
                }
            };
            completion.complete(move || {
                let userdata = userdata;
                unsafe { callback(status, convert::empty_string(), userdata.0, userdata.1) };
            });
        });
    future
}

pub unsafe extern "C" fn buffer_get_mapped_range(
    buffer: sb::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *mut c_void {
    let buffer = unsafe { Buffer::get(buffer) };
    let mut mapping = buffer.mapping.lock().unwrap();
    mapping.address(&buffer.buffer, wgpu::MapMode::Write, offset, size) as *mut c_void
}

pub unsafe extern "C" fn buffer_get_const_mapped_range(
    buffer: sb::WGPUBuffer,
    offset: usize,
    size: usize,
) -> *const c_void {
    let buffer = unsafe { Buffer::get(buffer) };
    let mut mapping = buffer.mapping.lock().unwrap();
    mapping.address(&buffer.buffer, wgpu::MapMode::Read, offset, size) as *const c_void
}

pub unsafe extern "C" fn buffer_unmap(buffer: sb::WGPUBuffer) {
    let buffer = unsafe { Buffer::get(buffer) };
    // The view must be dropped before `unmap`, which `wgpu` rejects while views exist.
    *buffer.mapping.lock().unwrap() = Mapping::Unmapped;
    buffer.buffer.unmap();
}

pub unsafe extern "C" fn buffer_get_map_state(buffer: sb::WGPUBuffer) -> sb::WGPUBufferMapState {
    use sb::WGPUBufferMapState as S;
    match *unsafe { Buffer::get(buffer) }.mapping.lock().unwrap() {
        Mapping::Unmapped => S::WGPUBufferMapState_Unmapped,
        Mapping::Pending => S::WGPUBufferMapState_Pending,
        Mapping::Mapped { .. } => S::WGPUBufferMapState_Mapped,
    }
}

pub unsafe extern "C" fn buffer_get_size(buffer: sb::WGPUBuffer) -> u64 {
    unsafe { Buffer::get(buffer) }.buffer.size()
}

pub unsafe extern "C" fn buffer_get_usage(buffer: sb::WGPUBuffer) -> sb::WGPUBufferUsage {
    convert::c_buffer_usage(unsafe { Buffer::get(buffer) }.buffer.usage())
}

pub unsafe extern "C" fn buffer_destroy(buffer: sb::WGPUBuffer) {
    unsafe { Buffer::get(buffer) }.buffer.destroy();
}

/// `wgpu` cannot rename an object after its creation. Labels only help debugging, so the shim
/// ignores the new label.
pub unsafe extern "C" fn buffer_set_label(_buffer: sb::WGPUBuffer, _label: sb::WGPUStringView) {}

enum Mapping {
    Unmapped,
    Pending,
    Mapped {
        mode: wgpu::MapMode,
        offset: u64,
        size: u64,
        /// Created by the first `GetMappedRange` call and kept until the buffer is unmapped.
        view: Option<View>,
    },
}

enum View {
    Read(wgpu::BufferView),
    Write(wgpu::BufferViewMut),
}

impl Mapping {
    /// Returns the address of `offset` in the mapped range, or null when the buffer is not mapped
    /// for `mode` or the range lies outside the mapping, as WebGPU specifies.
    fn address(
        &mut self,
        buffer: &wgpu::Buffer,
        requested: wgpu::MapMode,
        offset: usize,
        size: usize,
    ) -> *mut u8 {
        let Mapping::Mapped {
            mode,
            offset: mapped_offset,
            size: mapped_size,
            view,
        } = self
        else {
            return std::ptr::null_mut();
        };
        // A buffer mapped for writing may also be read. One mapped for reading may not be written.
        if requested == wgpu::MapMode::Write && *mode == wgpu::MapMode::Read {
            return std::ptr::null_mut();
        }
        let offset = offset as u64;
        let end = *mapped_offset + *mapped_size;
        let size = if size == usize::MAX {
            end.saturating_sub(offset)
        } else {
            size as u64
        };
        if offset < *mapped_offset || offset + size > end {
            return std::ptr::null_mut();
        }
        let range = *mapped_offset..end;
        let view = view.get_or_insert_with(|| match mode {
            wgpu::MapMode::Read => View::Read(
                buffer
                    .get_mapped_range(range)
                    .expect("mapping the buffer for reading failed"),
            ),
            wgpu::MapMode::Write => View::Write(
                buffer
                    .get_mapped_range_mut(range)
                    .expect("mapping the buffer for writing failed"),
            ),
        });
        let base = match view {
            View::Read(view) => view.as_ptr() as *mut u8,
            View::Write(view) => view.slice(..).as_raw_ptr().as_ptr() as *mut u8,
        };
        unsafe { base.add((offset - *mapped_offset) as usize) }
    }
}
