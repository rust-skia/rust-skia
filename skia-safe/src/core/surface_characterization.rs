#[cfg(feature = "gpu")]
use crate::gpu;
use crate::{prelude::*, ColorSpace, SurfaceProps};
use skia_bindings::{self as sb, SkSurfaceCharacterization};
use std::fmt;

pub type SurfaceCharacterization = Handle<SkSurfaceCharacterization>;
unsafe_send_sync!(SurfaceCharacterization);

impl NativeDrop for SkSurfaceCharacterization {
    fn drop(&mut self) {
        unsafe { sb::C_SkSurfaceCharacterization_destruct(self) }
    }
}

impl NativeClone for SkSurfaceCharacterization {
    fn clone(&self) -> Self {
        construct(|sc| unsafe { sb::C_SkSurfaceCharacterization_CopyConstruct(sc, self) })
    }
}

impl NativePartialEq for SkSurfaceCharacterization {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkSurfaceCharacterization_equals(self, rhs) }
    }
}

impl Default for SurfaceCharacterization {
    fn default() -> Self {
        SurfaceCharacterization::construct(|sc| unsafe {
            sb::C_SkSurfaceCharacterization_Construct(sc)
        })
    }
}

impl fmt::Debug for SurfaceCharacterization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SurfaceCharacterization");
        #[cfg(feature = "gpu")]
        let d = d
            .field("cache_max_resource_bytes", &self.cache_max_resource_bytes())
            .field("image_info", &self.image_info())
            .field("backend_format", &self.backend_format())
            .field("origin", &self.origin())
            .field("sample_count", &self.sample_count())
            .field("is_textureable", &self.is_textureable())
            .field("is_mip_mapped", &self.is_mip_mapped())
            .field("uses_glfbo0", &self.uses_glfbo0())
            .field(
                "vk_rt_supports_input_attachment",
                &self.vk_rt_supports_input_attachment(),
            )
            .field(
                "vulkan_secondary_cb_compatible",
                &self.vulkan_secondary_cb_compatible(),
            )
            .field("is_protected", &self.is_protected())
            .field("color_space", &self.color_space());
        d.field("surface_props", &self.surface_props()).finish()
    }
}

// TODO: there is an alternative for when SK_GANESH is not set, of which the
//       layout differs, should we support that?
impl SurfaceCharacterization {
    #[cfg(feature = "gpu")]
    #[must_use]
    pub fn resized(&self, size: impl Into<crate::ISize>) -> Self {
        let size = size.into();
        Self::construct(|sc| unsafe {
            sb::C_SkSurfaceCharacterization_createResized(
                self.native(),
                size.width,
                size.height,
                sc,
            )
        })
    }

    #[must_use]
    pub fn with_color_space(&self, color_space: impl Into<Option<ColorSpace>>) -> Self {
        let mut characterization = Self::default();
        unsafe {
            sb::C_SkSurfaceCharacterization_createColorSpace(
                self.native(),
                color_space.into().into_ptr_or_null(),
                characterization.native_mut(),
            )
        };
        characterization
    }

    #[cfg(feature = "gpu")]
    #[must_use]
    pub fn with_backend_format(
        &self,
        color_type: crate::ColorType,
        backend_format: &gpu::BackendFormat,
    ) -> Self {
        Self::construct(|sc| unsafe {
            sb::C_SkSurfaceCharacterization_createBackendFormat(
                self.native(),
                color_type.into_native(),
                backend_format.native(),
                sc,
            )
        })
    }

    #[cfg(feature = "gl")]
    #[must_use]
    pub fn with_fbo0(&self, uses_glfbo0: bool) -> Self {
        Self::construct(|sc| unsafe {
            sb::C_SkSurfaceCharacterization_createFBO0(self.native(), uses_glfbo0, sc)
        })
    }
}

#[cfg(feature = "gpu")]
impl SurfaceCharacterization {
    // TODO: contextInfo() / refContextInfo()

    pub fn cache_max_resource_bytes(&self) -> usize {
        self.native().fCacheMaxResourceBytes
    }

    pub fn is_valid(&self) -> bool {
        self.image_info().color_type() != crate::ColorType::Unknown
    }

    pub fn image_info(&self) -> &crate::ImageInfo {
        crate::ImageInfo::from_native_ref(unsafe {
            &*sb::C_SkSurfaceCharacterization_imageInfo(self.native())
        })
    }

    pub fn backend_format(&self) -> &gpu::BackendFormat {
        gpu::BackendFormat::from_native_ref(&self.native().fBackendFormat)
    }

    pub fn origin(&self) -> gpu::SurfaceOrigin {
        self.native().fOrigin
    }

    pub fn dimensions(&self) -> crate::ISize {
        self.image_info().dimensions()
    }

    pub fn width(&self) -> i32 {
        self.image_info().width()
    }

    pub fn height(&self) -> i32 {
        self.image_info().height()
    }

    pub fn color_type(&self) -> crate::ColorType {
        self.image_info().color_type()
    }

    pub fn sample_count(&self) -> usize {
        self.native().fSampleCnt.try_into().unwrap()
    }

    pub fn is_textureable(&self) -> bool {
        self.native().fIsTextureable == sb::SkSurfaceCharacterization_Textureable::kYes
    }

    pub fn is_mip_mapped(&self) -> bool {
        self.native().fIsMipMapped == sb::SkSurfaceCharacterization_MipMapped::kYes
    }

    pub fn uses_glfbo0(&self) -> bool {
        self.native().fUsesGLFBO0 == sb::SkSurfaceCharacterization_UsesGLFBO0::kYes
    }

    pub fn vk_rt_supports_input_attachment(&self) -> bool {
        self.native().fVkRTSupportsInputAttachment
            == sb::SkSurfaceCharacterization_VkRTSupportsInputAttachment::kYes
    }

    pub fn vulkan_secondary_cb_compatible(&self) -> bool {
        self.native().fVulkanSecondaryCBCompatible
            == sb::SkSurfaceCharacterization_VulkanSecondaryCBCompatible::kYes
    }

    pub fn is_protected(&self) -> gpu::Protected {
        self.native().fIsProtected
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.image_info().color_space()
    }
}

impl Handle<SkSurfaceCharacterization> {
    pub fn surface_props(&self) -> &SurfaceProps {
        SurfaceProps::from_native_ref(&self.native().fSurfaceProps)
    }

    #[cfg(feature = "gpu")]
    pub fn is_compatible(&self, backend_texture: &gpu::BackendTexture) -> bool {
        unsafe { self.native().isCompatible(backend_texture.native()) }
    }
}
