use crate::prelude::*;
use skia_bindings::{GrSurfaceOrigin, GrMipMapped, GrBackendApi};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BackendAPI {
    Metal = GrBackendApi::kMetal as _,
    OpenGL = GrBackendApi::kOpenGL as _,
    Vulkan = GrBackendApi::kVulkan as _,
    Mock = GrBackendApi::kMock as _
}

impl NativeTransmutable<GrBackendApi> for BackendAPI {}
#[test] fn test_backend_api_layout() { BackendAPI::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum MipMapped {
    No = GrMipMapped::kNo as _,
    Yes = GrMipMapped::kYes as _
}

impl NativeTransmutable<GrMipMapped> for MipMapped {}
#[test] fn test_mip_mapped_layout() { MipMapped::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SurfaceOrigin {
    TopLeft = GrSurfaceOrigin::kTopLeft_GrSurfaceOrigin as _,
    BottomLeft = GrSurfaceOrigin::kBottomLeft_GrSurfaceOrigin as _
}

impl NativeTransmutable<GrSurfaceOrigin> for SurfaceOrigin {}
#[test] fn test_surface_origin_layout() { SurfaceOrigin::test_layout() }

// Note: BackendState is in gl/types.rs/
