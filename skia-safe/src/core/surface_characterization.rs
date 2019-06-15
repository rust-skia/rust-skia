use crate::gpu::SurfaceOrigin;
use crate::prelude::*;
use crate::{ColorSpace, ColorType, ISize, ImageInfo, SurfaceProps};
use skia_bindings::{
    C_SkSurfaceCharacterization_destruct, C_SkSurfaceCharacterization_equals,
    SkSurfaceCharacterization,C_SkSurfaceCharacterization_imageInfo
};

pub type SurfaceCharacterization = Handle<SkSurfaceCharacterization>;

impl NativeDrop for SkSurfaceCharacterization {
    fn drop(&mut self) {
        unsafe { C_SkSurfaceCharacterization_destruct(self) }
    }
}

impl NativeClone for SkSurfaceCharacterization {
    fn clone(&self) -> Self {
        unsafe { SkSurfaceCharacterization::new2(self) }
    }
}

impl NativePartialEq for SkSurfaceCharacterization {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkSurfaceCharacterization_equals(self, rhs) }
    }
}

impl Default for Handle<SkSurfaceCharacterization> {
    fn default() -> Self {
        SurfaceCharacterization::from_native(unsafe { SkSurfaceCharacterization::new() })
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
        unsafe { self.native().cacheMaxResourceBytes() }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }

    pub fn image_info(&self) -> &ImageInfo {
        // no dice to link that under windows:
        // ImageInfo::from_native(unsafe { (*self.native().imageInfo()).clone() })
        ImageInfo::from_native_ref(unsafe { &*C_SkSurfaceCharacterization_imageInfo(self.native()) })
    }

    pub fn origin(&self) -> SurfaceOrigin {
        SurfaceOrigin::from_native(unsafe { self.native().origin() })
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn color_type(&self) -> ColorType {
        ColorType::from_native(unsafe { self.native().colorType() })
    }

    // TODO: fsaaType() (GrFSAAType is defined in GrTypesPriv.h)

    pub fn stencil_count(&self) -> usize {
        unsafe { self.native().stencilCount() }.try_into().unwrap()
    }

    pub fn is_textureable(&self) -> bool {
        unsafe { self.native().isTextureable() }
    }

    pub fn is_mip_mapped(&self) -> bool {
        unsafe { self.native().isMipMapped() }
    }

    pub fn uses_glfbo0(&self) -> bool {
        unsafe { self.native().usesGLFBO0() }
    }

    pub fn vulkan_secondary_cb_compatible(&self) -> bool {
        unsafe { self.native().vulkanSecondaryCBCompatible() }
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(unsafe { self.native().colorSpace() })
    }

    pub fn surface_props(&self) -> &SurfaceProps {
        SurfaceProps::from_native_ref(unsafe { &*self.native().surfaceProps() })
    }
}
