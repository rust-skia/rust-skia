use crate::gpu::{BackendAPI, BackendFormat, DirectContext, Renderable};
use crate::prelude::*;
use crate::{image, ColorType};
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
    // From GrContext_Base
    pub fn as_direct_context(&mut self) -> Option<DirectContext> {
        DirectContext::from_unshared_ptr(unsafe {
            sb::C_GrRecordingContext_asDirectContext(self.native_mut())
        })
    }

    // From GrContext_Base
    pub fn backend(&self) -> BackendAPI {
        unsafe { sb::C_GrRecordingContext_backend(self.native()) }
    }

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

    // From GrContext_Base
    pub fn compressed_backend_format(
        &self,
        compression_type: image::CompressionType,
    ) -> BackendFormat {
        let mut format = BackendFormat::default();
        unsafe {
            sb::C_GrRecordingContext_compressedBackendFormat(
                self.native(),
                compression_type,
                format.native_mut(),
            )
        }
        format
    }

    // TODO: GrContext_Base::threadSafeProxy

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
