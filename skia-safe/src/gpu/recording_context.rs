use crate::{
    gpu::{BackendAPI, BackendFormat, DirectContext, Renderable},
    prelude::*,
    ColorType, TextureCompressionType,
};
use skia_bindings::{self as sb, GrRecordingContext, SkRefCntBase};
use std::fmt;

pub type RecordingContext = RCHandle<GrRecordingContext>;

impl NativeRefCountedBase for GrRecordingContext {
    type Base = SkRefCntBase;
}

impl From<DirectContext> for RecordingContext {
    fn from(direct_context: DirectContext) -> Self {
        unsafe { std::mem::transmute(direct_context) }
    }
}

impl fmt::Debug for RecordingContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordingContext")
            .field("backend", &self.backend())
            .field("max_texture_size", &self.max_texture_size())
            .field("max_render_target_size", &self.max_render_target_size())
            .finish()
    }
}

impl RecordingContext {
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
        let mut format = BackendFormat::new_invalid();
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
        compression_type: TextureCompressionType,
    ) -> BackendFormat {
        let mut format = BackendFormat::new_invalid();
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

    pub fn max_texture_size(&self) -> i32 {
        unsafe { self.native().maxTextureSize() }
    }

    pub fn max_render_target_size(&self) -> i32 {
        unsafe { self.native().maxRenderTargetSize() }
    }

    pub fn color_type_supported_as_image(&self, color_type: ColorType) -> bool {
        unsafe {
            self.native()
                .colorTypeSupportedAsImage(color_type.into_native())
        }
    }

    pub fn supports_protected_content(&self) -> bool {
        unsafe { self.native().supportsProtectedContent() }
    }

    pub fn max_surface_sample_count_for_color_type(&self, color_type: ColorType) -> usize {
        unsafe {
            sb::C_GrRecordingContext_maxSurfaceSampleCountForColorType(
                self.native(),
                color_type.into_native(),
            )
        }
        .try_into()
        .unwrap()
    }

    // TODO: Wrap Arenas (if used).
}
