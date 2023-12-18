use super::{ID3D12CommandQueue, ID3D12Device, IDXGIAdapter1, MemoryAllocator};
use crate::gpu;
use skia_bindings::GrD3DBackendContext;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BackendContext {
    pub adapter: IDXGIAdapter1,
    pub device: ID3D12Device,
    pub queue: ID3D12CommandQueue,
    pub memory_allocator: Option<MemoryAllocator>,
    pub protected_context: gpu::Protected,
}

native_transmutable!(GrD3DBackendContext, BackendContext, backend_context_layout);
