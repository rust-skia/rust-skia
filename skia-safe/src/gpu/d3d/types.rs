use super::{cp, ID3D12Resource, D3D12_RESOURCE_STATES, DXGI_FORMAT};
use crate::gpu;
use crate::prelude::*;
use skia_bindings::GrD3DTextureResourceInfo;

#[repr(C)]
#[derive(Clone)]
pub struct TextureResourceInfo {
    pub resource: cp<ID3D12Resource>,
    pub resource_state: D3D12_RESOURCE_STATES,
    pub format: DXGI_FORMAT,
    pub level_count: u32,
    pub sample_quality_level: std::os::raw::c_uint,
    pub protected: gpu::Protected,
}

impl NativeTransmutable<GrD3DTextureResourceInfo> for TextureResourceInfo {}
#[test]
fn test_texture_resource_info_layout() {
    TextureResourceInfo::test_layout();
}
