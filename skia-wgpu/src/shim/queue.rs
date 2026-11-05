//! The queue: submission, buffer writes, and the work-done future.

use std::ffi;
use std::sync::Arc;

use skia_bindings as sb;

use super::buffer::Buffer;
use super::encoder::CommandBuffer;
use super::future::Userdata;
use super::handle::{Handle, handles};
use super::instance::Instance;
use super::{convert, wgpu};

/// Behind a `WGPUQueue`: the application's `wgpu` queue, which Skia and the application share.
#[derive(Debug)]
pub struct Queue {
    pub queue: wgpu::Queue,
    pub instance: Arc<Instance>,
}

handles! {
    Queue => WGPUQueue:
        wgpuQueueAddRef: WGPUProcQueueAddRef,
        wgpuQueueRelease: WGPUProcQueueRelease;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuQueueSubmit(
    queue: sb::WGPUQueue,
    command_count: usize,
    commands: *const sb::WGPUCommandBuffer,
) {
    let queue = unsafe { Queue::get(queue) };
    let commands = unsafe { convert::slice(commands, command_count) };
    let buffers: Vec<wgpu::CommandBuffer> = commands
        .iter()
        .map(|&command| unsafe { CommandBuffer::get(command) }.take())
        .collect();
    queue.queue.submit(buffers);
}
const _: sb::WGPUProcQueueSubmit = Some(wgpuQueueSubmit);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuQueueWriteBuffer(
    queue: sb::WGPUQueue,
    buffer: sb::WGPUBuffer,
    buffer_offset: u64,
    data: *const ffi::c_void,
    size: usize,
) {
    let queue = unsafe { Queue::get(queue) };
    let buffer = unsafe { Buffer::get(buffer) };
    let data = unsafe { convert::slice(data as *const u8, size) };
    queue
        .queue
        .write_buffer(&buffer.buffer, buffer_offset, data);
}
const _: sb::WGPUProcQueueWriteBuffer = Some(wgpuQueueWriteBuffer);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuQueueOnSubmittedWorkDone(
    queue: sb::WGPUQueue,
    callback_info: sb::WGPUQueueWorkDoneCallbackInfo,
) -> sb::WGPUFuture {
    unsafe { convert::no_chain(callback_info.nextInChain, "WGPUQueueWorkDoneCallbackInfo") };
    let queue = unsafe { Queue::get(queue) };
    use sb::WGPUQueueWorkDoneStatus as S;
    let callback = callback_info
        .callback
        .expect("null WGPUQueueWorkDoneCallback");
    let userdata = Userdata(callback_info.userdata1, callback_info.userdata2);
    let (future, completion) =
        queue
            .instance
            .futures
            .register(callback_info.mode, move |done: Option<()>| {
                let userdata = userdata;
                let status = match done {
                    Some(()) => S::WGPUQueueWorkDoneStatus_Success,
                    None => S::WGPUQueueWorkDoneStatus_CallbackCancelled,
                };
                unsafe { callback(status, convert::empty_string(), userdata.0, userdata.1) };
            });
    queue
        .queue
        .on_submitted_work_done(move || completion.complete(()));
    future
}
const _: sb::WGPUProcQueueOnSubmittedWorkDone = Some(wgpuQueueOnSubmittedWorkDone);
