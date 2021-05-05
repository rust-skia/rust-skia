use super::{cp, MemoryAllocator};
use super::{ID3D12CommandQueue, ID3D12Device, IDXGIAdapter1};
use crate::gpu;
use crate::prelude::*;
use skia_bindings::GrD3DBackendContext;

#[repr(C)]
#[derive(Clone)]
pub struct BackendContext {
    pub adapter: cp<IDXGIAdapter1>,
    pub device: cp<ID3D12Device>,
    pub queue: cp<ID3D12CommandQueue>,
    pub memory_allocator: Option<MemoryAllocator>,
    pub protected_context: gpu::Protected,
}
unsafe impl Send for BackendContext {}

impl NativeTransmutable<GrD3DBackendContext> for BackendContext {}
#[test]
fn test_backend_context_layout() {
    BackendContext::test_layout();
}
