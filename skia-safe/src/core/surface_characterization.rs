use crate::gpu::SurfaceOrigin;
use crate::prelude::*;
use crate::{ColorSpace, ColorType, ISize, ImageInfo, SurfaceProps};
use skia_bindings::{
    C_SkSurfaceCharacterization_Construct, C_SkSurfaceCharacterization_CopyConstruct,
    C_SkSurfaceCharacterization_destruct, C_SkSurfaceCharacterization_equals,
    C_SkSurfaceCharacterization_imageInfo, SkSurfaceCharacterization,
    SkSurfaceCharacterization_MipMapped, SkSurfaceCharacterization_Textureable,
    SkSurfaceCharacterization_UsesGLFBO0, SkSurfaceCharacterization_VulkanSecondaryCBCompatible,
};

pub type SurfaceCharacterization = Handle<SkSurfaceCharacterization>;

impl NativeDrop for SkSurfaceCharacterization {
    fn drop(&mut self) {
        unsafe { C_SkSurfaceCharacterization_destruct(self) }
    }
}

impl NativeClone for SkSurfaceCharacterization {
    fn clone(&self) -> Self {
        construct(|sc| unsafe { C_SkSurfaceCharacterization_CopyConstruct(sc, self) })
    }
}

impl NativePartialEq for SkSurfaceCharacterization {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkSurfaceCharacterization_equals(self, rhs) }
    }
}

impl Default for Handle<SkSurfaceCharacterization> {
    fn default() -> Self {
        SurfaceCharacterization::from_native(construct(|sc| unsafe {
            C_SkSurfaceCharacterization_Construct(sc)
        }))
    }
}

// TODO: there is an alterative for when SK_SUPPORT_GPU is not set, of which the
//       layout differs, should we support that?
impl Handle<SkSurfaceCharacterization> {
    pub fn resized(&self, size: impl Into<ISize>) -> SurfaceCharacterization {
        let size = size.into();
        Self::from_native(unsafe { self.native().createResized(size.width, size.height) })
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
            &*C_SkSurfaceCharacterization_imageInfo(self.native())
        })
    }

    pub fn origin(&self) -> SurfaceOrigin {
        SurfaceOrigin::from_native(self.native().fOrigin)
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

    // TODO: fsaaType() (GrFSAAType is defined in GrTypesPriv.h)

    pub fn stencil_count(&self) -> usize {
        self.native().fStencilCnt.try_into().unwrap()
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

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.image_info().color_space()
    }

    pub fn surface_props(&self) -> &SurfaceProps {
        SurfaceProps::from_native_ref(&self.native().fSurfaceProps)
    }
}
