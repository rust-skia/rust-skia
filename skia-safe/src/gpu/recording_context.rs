use crate::gpu::{BackendFormat, Renderable};
use crate::prelude::*;
use crate::ColorType;
use skia_bindings as sb;
use skia_bindings::{GrContext, GrDirectContext, GrRecordingContext, SkRefCntBase};

pub type RecordingContext = RCHandle<GrRecordingContext>;

impl NativeRefCountedBase for GrRecordingContext {
    type Base = SkRefCntBase;
}

impl From<RCHandle<GrContext>> for RCHandle<GrRecordingContext> {
    fn from(direct_context: RCHandle<GrContext>) -> Self {
        unsafe { std::mem::transmute(direct_context) }
    }
}
impl From<RCHandle<GrDirectContext>> for RCHandle<GrRecordingContext> {
    fn from(direct_context: RCHandle<GrDirectContext>) -> Self {
        unsafe { std::mem::transmute(direct_context) }
    }
}

impl RCHandle<GrRecordingContext> {
    pub fn default_backend_format(&self, ct: ColorType, renderable: Renderable) -> BackendFormat {
        let mut format = BackendFormat::default();
        unsafe {
            sb::C_GrRecordingContext_defaultBackendFormat(
                self.native(),
                ct.into_native(),
                renderable,
                format.native_mut(),
            )
        };
        format
    }

    pub fn abandoned(&mut self) -> bool {
        unsafe { sb::C_GrRecordingContext_abandoned(self.native_mut()) }
    }

    pub fn color_type_supported_as_surface(&self, color_type: ColorType) -> bool {
        unsafe {
            sb::C_GrRecordingContext_colorTypeSupportedAsSurface(
                self.native(),
                color_type.into_native(),
            )
        }
    }

    pub fn max_surface_sample_count_for_color_type(&self, color_type: ColorType) -> usize {
        unsafe {
            self.native()
                .maxSurfaceSampleCountForColorType(color_type.into_native())
        }
        .try_into()
        .unwrap()
    }
}
