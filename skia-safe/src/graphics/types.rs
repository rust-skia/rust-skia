use crate::prelude::*;
use skia_bindings::{
    GrSurfaceOrigin,
    GrMipMapped
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum MipMapped {
    No = GrMipMapped::kNo as _,
    Yes = GrMipMapped::kYes as _
}

impl NativeTransmutable<GrMipMapped> for MipMapped {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SurfaceOrigin {
    TopLeft = GrSurfaceOrigin::kTopLeft_GrSurfaceOrigin as _,
    BottomLeft = GrSurfaceOrigin::kBottomLeft_GrSurfaceOrigin as _
}

impl NativeTransmutable<GrSurfaceOrigin> for SurfaceOrigin {}
