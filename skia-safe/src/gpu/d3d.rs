use crate::prelude::*;
use skia_bindings as sb;
use winapi::{shared::dxgi, shared::dxgiformat, um::d3d12};

mod backend_context;
pub use backend_context::*;

mod types;
pub use types::*;

// re-export D3D types we use

pub use sb::GrD3DResourceStateEnum as ResourceStateEnum;

pub use d3d12::ID3D12CommandQueue;
pub use d3d12::ID3D12Device;
pub use d3d12::ID3D12Resource;
pub use d3d12::D3D12_RESOURCE_STATES;
pub use dxgi::IDXGIAdapter1;
pub use dxgiformat::DXGI_FORMAT;

impl NativeTransmutable<sb::DXGI_FORMAT> for dxgiformat::DXGI_FORMAT {}
#[test]
fn test_dxgi_format_layout() {
    dxgiformat::DXGI_FORMAT::test_layout();
}
