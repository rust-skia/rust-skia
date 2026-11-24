use std::fmt;

use crate::{
    ColorType, TextureCompressionType,
    gpu::{BackendFormat, DirectContext, Renderable},
    prelude::*,
    recorder::RecorderRef,
};

use super::types::BackendApi;
use skia_bindings::{self as sb, GrRecordingContext, SkRefCntBase};

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
    /// Returns this context as a [`DirectContext`], or `None` if this context is not a
    /// [`DirectContext`].
    pub fn as_direct_context(&mut self) -> Option<DirectContext> {
        DirectContext::from_unshared_ptr(unsafe {
            sb::C_GrRecordingContext_asDirectContext(self.native_mut())
        })
    }

    // From GrContext_Base
    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_GrRecordingContext_backend(self.native()) }
    }

    /// Retrieve the default [`BackendFormat`] for a given [`ColorType`] and renderability.
    /// The caller should check that the returned format is valid.
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
    /// Retrieve the [`BackendFormat`] for a given [`TextureCompressionType`].
    ///
    /// The caller should check that the returned format is valid.
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

    /// Reports whether the [`DirectContext`] associated with this [`RecordingContext`] is
    /// abandoned. When called on a [`DirectContext`] it may actively check whether the
    /// underlying 3D API device/context has been disconnected before reporting the status.
    /// If so, calling this method will transition the [`DirectContext`] to the abandoned state.
    pub fn abandoned(&mut self) -> bool {
        unsafe { sb::C_GrRecordingContext_abandoned(self.native_mut()) }
    }

    /// Can a [`crate::Surface`] be created with the given color type. To check whether MSAA is
    /// supported use [`RecordingContext::max_surface_sample_count_for_color_type()`].
    pub fn color_type_supported_as_surface(&self, color_type: ColorType) -> bool {
        unsafe {
            sb::C_GrRecordingContext_colorTypeSupportedAsSurface(
                self.native(),
                color_type.into_native(),
            )
        }
    }

    /// Gets the maximum supported texture size.
    pub fn max_texture_size(&self) -> i32 {
        unsafe { self.native().maxTextureSize() }
    }

    /// Gets the maximum supported render target size.
    pub fn max_render_target_size(&self) -> i32 {
        unsafe { self.native().maxRenderTargetSize() }
    }

    /// Can a [`crate::Image`] be created with the given color type.
    pub fn color_type_supported_as_image(&self, color_type: ColorType) -> bool {
        unsafe {
            self.native()
                .colorTypeSupportedAsImage(color_type.into_native())
        }
    }

    /// Does this context support protected content?
    pub fn supports_protected_content(&self) -> bool {
        unsafe { self.native().supportsProtectedContent() }
    }

    /// Gets the maximum supported sample count for a color type. 1 is returned if only non-MSAA
    /// rendering is supported for the color type. 0 is returned if rendering to this color type
    /// is not supported at all.
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

    /// Returns this context as a [`RecorderRef`].
    pub fn as_recorder(&mut self) -> &mut RecorderRef {
        RecorderRef::from_ref_mut(unsafe { &mut *self.native_mut().asRecorder() })
    }

    // TODO: Wrap Arenas (if used).
}
