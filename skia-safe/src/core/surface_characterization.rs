use crate::gpu::SurfaceOrigin;
use crate::prelude::*;
use crate::{gpu, ColorSpace, ColorType, ISize, ImageInfo, SurfaceProps};
use skia_bindings as sb;
use skia_bindings::{
    SkSurfaceCharacterization, SkSurfaceCharacterization_MipMapped,
    SkSurfaceCharacterization_Textureable, SkSurfaceCharacterization_UsesGLFBO0,
    SkSurfaceCharacterization_VulkanSecondaryCBCompatible,
};

pub type SurfaceCharacterization = Handle<SkSurfaceCharacterization>;

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

impl Default for Handle<SkSurfaceCharacterization> {
    fn default() -> Self {
        SurfaceCharacterization::from_native(construct(|sc| unsafe {
            sb::C_SkSurfaceCharacterization_Construct(sc)
        }))
    }
}

// TODO: there is an alterative for when SK_SUPPORT_GPU is not set, of which the
//       layout differs, should we support that?
impl Handle<SkSurfaceCharacterization> {
    pub fn resized(&self, size: impl Into<ISize>) -> Self {
        let size = size.into();
        Self::from_native(unsafe { self.native().createResized(size.width, size.height) })
    }

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

    // TODO: contextInfo() / refContextInfo()

    pub fn cache_max_resource_bytes(&self) -> usize {
        self.native().fCacheMaxResourceBytes
    }

    pub fn is_valid(&self) -> bool {
        self.image_info().color_type() != ColorType::Unknown
    }

    pub fn image_info(&self) -> &ImageInfo {
        ImageInfo::from_native_ref(unsafe {
            &*sb::C_SkSurfaceCharacterization_imageInfo(self.native())
        })
    }

    pub fn backend_format(&self) -> &gpu::BackendFormat {
        gpu::BackendFormat::from_native_ref(&self.native().fBackendFormat)
    }

    pub fn origin(&self) -> SurfaceOrigin {
        self.native().fOrigin
    }

    pub fn width(&self) -> i32 {
        self.image_info().width()
    }

    pub fn height(&self) -> i32 {
        self.image_info().height()
    }

    pub fn color_type(&self) -> ColorType {
        self.image_info().color_type()
    }

    #[deprecated(since = "0.17.0", note = "use sample_count()")]
    pub fn stencil_count(&self) -> usize {
        self.sample_count()
    }

    pub fn sample_count(&self) -> usize {
        self.native().fSampleCnt.try_into().unwrap()
    }

    pub fn is_textureable(&self) -> bool {
        self.native().fIsTextureable == SkSurfaceCharacterization_Textureable::kYes
    }

    pub fn is_mip_mapped(&self) -> bool {
        self.native().fIsMipMapped == SkSurfaceCharacterization_MipMapped::kYes
    }

    pub fn uses_glfbo0(&self) -> bool {
        self.native().fUsesGLFBO0 == SkSurfaceCharacterization_UsesGLFBO0::kYes
    }

    pub fn vulkan_secondary_cb_compatible(&self) -> bool {
        self.native().fVulkanSecondaryCBCompatible
            == SkSurfaceCharacterization_VulkanSecondaryCBCompatible::kYes
    }

    pub fn is_protected(&self) -> gpu::Protected {
        self.native().fIsProtected
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.image_info().color_space()
    }

    pub fn surface_props(&self) -> &SurfaceProps {
        SurfaceProps::from_native_ref(&self.native().fSurfaceProps)
    }

    pub fn is_compatible(&self, backend_texture: &gpu::BackendTexture) -> bool {
        unsafe { self.native().isCompatible(backend_texture.native()) }
    }
}
