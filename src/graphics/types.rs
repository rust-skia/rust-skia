use rust_skia::GrSurfaceOrigin;
use rust_skia::GrMipMapped;

#[derive(Copy, Clone, PartialEq)]
pub struct MipMapped(pub(crate) GrMipMapped);

#[allow(non_upper_case_globals)]
impl MipMapped {
    pub const No: MipMapped = MipMapped(GrMipMapped::kNo);
    pub const Yes: MipMapped = MipMapped(GrMipMapped::kYes);
}

#[derive(Copy, Clone, PartialEq)]
pub struct SurfaceOrigin(pub(crate) GrSurfaceOrigin);

#[allow(non_upper_case_globals)]
impl SurfaceOrigin {
    pub const TopLeft: SurfaceOrigin = SurfaceOrigin(GrSurfaceOrigin::kTopLeft_GrSurfaceOrigin);
    pub const BottomLeft: SurfaceOrigin = SurfaceOrigin(GrSurfaceOrigin::kBottomLeft_GrSurfaceOrigin);
}
