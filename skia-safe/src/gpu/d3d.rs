use skia_bindings as sb;

mod backend_context;
pub use backend_context::*;

mod types;
pub use types::*;

// re-export D3D types we use

pub use sb::AsIUnknown;
pub use sb::GrD3DResourceStateEnum as ResourceStateEnum;
pub use sb::ID3D12CommandQueue;
pub use sb::ID3D12Device;
pub use sb::ID3D12Resource;
pub use sb::IDXGIAdapter1;
pub use sb::D3D12_RESOURCE_STATES;
pub use sb::DXGI_FORMAT;
