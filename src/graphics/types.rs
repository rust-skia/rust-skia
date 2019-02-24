use crate::prelude::*;
use rust_skia::GrSurfaceOrigin;
use rust_skia::GrMipMapped;

pub type MipMapped = EnumHandle<GrMipMapped>;

#[allow(non_upper_case_globals)]
impl EnumHandle<GrMipMapped> {
    pub const No: MipMapped = EnumHandle(GrMipMapped::kNo);
    pub const Yes: MipMapped = EnumHandle(GrMipMapped::kYes);
}

pub type SurfaceOrigin = EnumHandle<GrSurfaceOrigin>;

#[allow(non_upper_case_globals)]
impl EnumHandle<GrSurfaceOrigin> {
    pub const TopLeft: SurfaceOrigin = EnumHandle(GrSurfaceOrigin::kTopLeft_GrSurfaceOrigin);
    pub const BottomLeft: SurfaceOrigin = EnumHandle(GrSurfaceOrigin::kBottomLeft_GrSurfaceOrigin);
}
