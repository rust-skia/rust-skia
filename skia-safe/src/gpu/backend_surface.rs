use super::{gl, BackendAPI, MipMapped};
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{GrBackendFormat, GrBackendRenderTarget, GrBackendTexture, GrMipMapped};

#[cfg(feature = "vulkan")]
use super::vk;

pub type BackendFormat = Handle<GrBackendFormat>;

impl NativeDrop for GrBackendFormat {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendFormat_destruct(self) }
    }
}

impl NativeClone for GrBackendFormat {
    fn clone(&self) -> Self {
        unsafe { GrBackendFormat::new(self) }
    }
}

impl Default for BackendFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<GrBackendFormat> {
    pub fn new() -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormat_Construct(bf) })
    }

    pub fn new_gl(format: gl::Enum, target: gl::Enum) -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormat_ConstructGL(bf, format, target) })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(format: vk::Format) -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormat_ConstructVk(bf, format) })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan_ycbcr(conversion_info: &vk::YcbcrConversionInfo) -> Self {
        Self::construct(|bf| unsafe {
            sb::C_GrBackendFormat_ConstructVk2(bf, conversion_info.native())
        })
    }

    #[deprecated(since = "0.0.0", note = "use backend()")]
    pub fn backend_api(&self) -> BackendAPI {
        self.backend()
    }

    pub fn backend(&self) -> BackendAPI {
        BackendAPI::from_native(self.native().fBackend)
    }

    // texture_type() would return a private type.

    #[deprecated(since = "0.0.0", note = "use as_gl_format()")]
    pub fn gl_format(&self) -> Option<gl::Enum> {
        Some(self.as_gl_format() as _)
    }

    pub fn as_gl_format(&self) -> gl::Format {
        gl::Format::from_native(unsafe {
            #[allow(clippy::map_clone)]
            self.native().asGLFormat()
        })
    }

    #[deprecated(since = "0.0.0", note = "use as_vk_format()")]
    #[cfg(feature = "vulkan")]
    pub fn vulkan_format(&self) -> Option<vk::Format> {
        self.as_vk_format()
    }

    #[cfg(feature = "vulkan")]
    pub fn as_vk_format(&self) -> Option<vk::Format> {
        let mut r = vk::Format::UNDEFINED;
        unsafe { self.native().asVkFormat(&mut r) }.if_true_some(r)
    }

    pub fn to_texture_2d(&self) -> Option<Self> {
        let new = Self::from_native(unsafe { self.native().makeTexture2D() });

        new.is_valid().if_true_some(new)
    }

    pub fn is_valid(&self) -> bool {
        self.native().fValid
    }
}

pub type BackendTexture = Handle<GrBackendTexture>;

impl NativeDrop for GrBackendTexture {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendTexture_destruct(self) }
    }
}

impl NativeClone for GrBackendTexture {
    fn clone(&self) -> Self {
        unsafe { GrBackendTexture::new3(self) }
    }
}

impl Handle<GrBackendTexture> {
    pub unsafe fn new_gl(
        (width, height): (i32, i32),
        mip_mapped: MipMapped,
        gl_info: gl::TextureInfo,
    ) -> BackendTexture {
        Self::from_native_if_valid(GrBackendTexture::new(
            width,
            height,
            mip_mapped.into_native(),
            gl_info.native(),
        ))
        .unwrap()
    }

    #[cfg(feature = "vulkan")]
    pub unsafe fn new_vulkan(
        (width, height): (i32, i32),
        vk_info: &vk::ImageInfo,
    ) -> BackendTexture {
        Self::from_native_if_valid(GrBackendTexture::new1(width, height, vk_info.native())).unwrap()
    }

    pub(crate) unsafe fn from_native_if_valid(
        backend_texture: GrBackendTexture,
    ) -> Option<BackendTexture> {
        backend_texture
            .fIsValid
            .if_true_then_some(|| BackendTexture::from_native(backend_texture))
    }

    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    pub fn has_mip_maps(&self) -> bool {
        self.native().fMipMapped == GrMipMapped::kYes
    }

    pub fn backend(&self) -> BackendAPI {
        BackendAPI::from_native(self.native().fBackend)
    }

    pub fn gl_texture_info(&self) -> Option<gl::TextureInfo> {
        unsafe {
            let mut texture_info = gl::TextureInfo::default();
            self.native()
                .getGLTextureInfo(texture_info.native_mut())
                .if_true_some(texture_info)
        }
    }

    pub fn gl_texture_parameters_modified(&mut self) {
        unsafe { self.native_mut().glTextureParametersModified() }
    }

    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        unsafe {
            // constructor not available.
            let mut image_info = vk::ImageInfo::default();
            self.native()
                .getVkImageInfo(image_info.native_mut())
                .if_true_some(image_info)
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        unsafe { self.native_mut().setVkImageLayout(layout) }
        self
    }

    pub fn backend_format(&self) -> Option<BackendFormat> {
        let format = BackendFormat::from_native(unsafe { self.native().getBackendFormat() });

        format.is_valid().if_true_some(format)
    }

    pub fn is_protected(&self) -> bool {
        unsafe { self.native().isProtected() }
    }

    pub fn is_valid(&self) -> bool {
        self.native().fIsValid
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_same_texture(&mut self, texture: &BackendTexture) -> bool {
        unsafe { self.native_mut().isSameTexture(texture.native()) }
    }
}

pub type BackendRenderTarget = Handle<GrBackendRenderTarget>;

impl NativeDrop for GrBackendRenderTarget {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendRenderTarget_destruct(self) }
    }
}

impl NativeClone for GrBackendRenderTarget {
    fn clone(&self) -> Self {
        unsafe { GrBackendRenderTarget::new4(self) }
    }
}

impl Handle<GrBackendRenderTarget> {
    pub fn new_gl(
        (width, height): (i32, i32),
        sample_count: impl Into<Option<usize>>,
        stencil_bits: usize,
        info: gl::FramebufferInfo,
    ) -> BackendRenderTarget {
        Self::from_native(unsafe {
            GrBackendRenderTarget::new(
                width,
                height,
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                stencil_bits.try_into().unwrap(),
                info.native(),
            )
        })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(
        (width, height): (i32, i32),
        sample_count: impl Into<Option<usize>>,
        info: &vk::ImageInfo,
    ) -> BackendRenderTarget {
        BackendRenderTarget::from_native(unsafe {
            GrBackendRenderTarget::new2(
                width,
                height,
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                info.native(),
            )
        })
    }

    pub(crate) fn from_native_if_valid(
        native: GrBackendRenderTarget,
    ) -> Option<BackendRenderTarget> {
        let backend_render_target = BackendRenderTarget::from_native(native);
        backend_render_target
            .is_valid()
            .if_true_some(backend_render_target)
    }

    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    pub fn sample_count(&self) -> usize {
        self.native().fSampleCnt.try_into().unwrap()
    }

    pub fn stencil_bits(&self) -> usize {
        self.native().fStencilBits.try_into().unwrap()
    }

    pub fn backend(&self) -> BackendAPI {
        BackendAPI::from_native(self.native().fBackend)
    }

    pub fn gl_framebuffer_info(&self) -> Option<gl::FramebufferInfo> {
        let mut info = gl::FramebufferInfo::default();
        unsafe { self.native().getGLFramebufferInfo(info.native_mut()) }.if_true_some(info)
    }

    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        let mut info = vk::ImageInfo::default();
        unsafe { self.native().getVkImageInfo(info.native_mut()) }.if_true_some(info)
    }

    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        unsafe { self.native_mut().setVkImageLayout(layout) }
        self
    }

    pub fn backend_format(&self) -> BackendFormat {
        BackendFormat::from_native(unsafe { self.native().getBackendFormat() })
    }

    pub fn is_protected(&self) -> bool {
        unsafe { self.native().isProtected() }
    }

    pub fn is_valid(&self) -> bool {
        self.native().fIsValid
    }
}
