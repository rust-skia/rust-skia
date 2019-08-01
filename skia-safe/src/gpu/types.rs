use crate::prelude::*;
use skia_bindings::{GrBackendApi, GrMipMapped, GrProtected, GrRenderable, GrSurfaceOrigin};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BackendAPI {
    Metal = GrBackendApi::kMetal as _,
    Dawn = GrBackendApi::kDawn as _,
    OpenGL = GrBackendApi::kOpenGL as _,
    Vulkan = GrBackendApi::kVulkan as _,
    Mock = GrBackendApi::kMock as _,
}

impl NativeTransmutable<GrBackendApi> for BackendAPI {}
#[test]
fn test_backend_api_layout() {
    BackendAPI::test_layout()
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum MipMapped {
    No = GrMipMapped::kNo as _,
    Yes = GrMipMapped::kYes as _,
}

impl NativeTransmutable<GrMipMapped> for MipMapped {}
#[test]
fn test_mip_mapped_layout() {
    MipMapped::test_layout()
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Renderable {
    No = GrRenderable::kNo as _,
    Yes = GrRenderable::kYes as _,
}

impl NativeTransmutable<GrRenderable> for Renderable {}
#[test]
fn test_renderable_layout() {
    Renderable::test_layout()
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Protected {
    No = GrProtected::kNo as _,
    Yes = GrProtected::kYes as _,
}

impl NativeTransmutable<GrProtected> for Protected {}
#[test]
fn test_protected_layout() {
    Protected::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SurfaceOrigin {
    TopLeft = GrSurfaceOrigin::kTopLeft_GrSurfaceOrigin as _,
    BottomLeft = GrSurfaceOrigin::kBottomLeft_GrSurfaceOrigin as _,
}

impl NativeTransmutable<GrSurfaceOrigin> for SurfaceOrigin {}
#[test]
fn test_surface_origin_layout() {
    SurfaceOrigin::test_layout()
}

// Note: BackendState is in gl/types.rs/
