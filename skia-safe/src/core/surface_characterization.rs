use crate::prelude::*;
use crate::{ISize, ImageInfo, ColorType, ColorSpace, SurfaceProps};
use crate::gpu::SurfaceOrigin;
use skia_bindings::SkSurfaceCharacterization;

pub type SurfaceCharacterization = Handle<SkSurfaceCharacterization>;

// TODO: implement clone.

impl NativeDrop for SkSurfaceCharacterization {
    fn drop(&mut self) {
        unimplemented!()
    }
}

// TODO: implement PartialEq

impl Default for SurfaceCharacterization {
    fn default() -> Self {
        unimplemented!()
    }
}

// TODO: complete the implementation
// TODO: there is an alterative for when SK_SUPPORT_GPU is not set, of which the
//       layout differs, should we support that?
impl SurfaceCharacterization {
    pub fn resized(&self, size: impl Into<ISize>) -> SurfaceCharacterization {
        unimplemented!()
    }

    // TODO: contextInfo() / refContextInfo()

    pub fn cache_max_resource_bytes(&self) -> usize {
        unimplemented!()
    }

    pub fn is_valid(&self) -> bool {
        unimplemented!()
    }

    pub fn image_info(&self) -> &ImageInfo {
        unimplemented!()
    }

    pub fn origin(&self) -> SurfaceOrigin {
        unimplemented!()
    }

    pub fn width(&self) -> i32 {
        unimplemented!()
    }

    pub fn height(&self) -> i32 {
        unimplemented!()
    }

    pub fn color_type(&self) -> ColorType {
        unimplemented!()
    }

    // TODO: fsaaType()

    pub fn stencil_count(&self) -> usize {
        unimplemented!()
    }

    pub fn is_textureable(&self) -> bool {
        unimplemented!()
    }

    pub fn is_mip_mapped(&self) -> bool {
        unimplemented!()
    }

    pub fn uses_glfbo0(&self) -> bool {
        unimplemented!()
    }

    pub fn vulkan_secondary_cb_compatible(&self) -> bool {
        unimplemented!()
    }

    pub fn color_space(&self) -> &ColorSpace {
        unimplemented!()
    }

    pub fn surface_props(&self) -> &SurfaceProps {
        unimplemented!()
    }
}
