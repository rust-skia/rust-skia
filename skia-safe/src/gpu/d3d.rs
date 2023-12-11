use skia_bindings as sb;

mod backend_context;
pub use backend_context::*;

mod types;
pub use types::*;

// re-export D3D types we use

pub use sb::GrD3DResourceStateEnum as ResourceStateEnum;

pub use windows::Win32::Graphics::Direct3D12::ID3D12CommandQueue;
pub use windows::Win32::Graphics::Direct3D12::ID3D12Device;
pub use windows::Win32::Graphics::Direct3D12::ID3D12Resource;
pub use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_STATES;
pub use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;
pub use windows::Win32::Graphics::Dxgi::IDXGIAdapter1;

native_transmutable!(sb::DXGI_FORMAT, DXGI_FORMAT, dxgi_format_layout);
