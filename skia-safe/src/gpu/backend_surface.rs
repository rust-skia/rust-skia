#[cfg(feature = "d3d")]
use super::d3d;
#[cfg(feature = "gl")]
use super::gl;
#[cfg(feature = "metal")]
use super::mtl;
#[cfg(feature = "vulkan")]
use super::vk;
use super::{BackendAPI, BackendSurfaceMutableState};
use crate::prelude::*;
use crate::ISize;
use skia_bindings as sb;
use skia_bindings::{GrBackendFormat, GrBackendRenderTarget, GrBackendTexture, GrMipmapped};

pub type BackendFormat = Handle<GrBackendFormat>;
unsafe impl Send for BackendFormat {}
unsafe impl Sync for BackendFormat {}

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

    #[cfg(feature = "gl")]
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

    #[cfg(feature = "metal")]
    pub fn new_metal(format: mtl::PixelFormat) -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormat_ConstructMtl(bf, format) })
    }

    #[cfg(feature = "d3d")]
    pub fn new_dxgi(format: d3d::DXGI_FORMAT) -> Self {
        Self::construct(|bf| unsafe {
            sb::C_GrBackendFormat_ConstructDxgi(bf, format.into_native())
        })
    }

    #[deprecated(since = "0.19.0", note = "use backend()")]
    pub fn backend_api(&self) -> BackendAPI {
        self.backend()
    }

    pub fn backend(&self) -> BackendAPI {
        self.native().fBackend
    }

    // texture_type() would return a private type.

    pub fn channel_mask(&self) -> u32 {
        unsafe { self.native().channelMask() }
    }

    #[deprecated(since = "0.19.0", note = "use as_gl_format()")]
    #[cfg(feature = "gl")]
    pub fn gl_format(&self) -> Option<gl::Enum> {
        Some(self.as_gl_format() as _)
    }

    #[cfg(feature = "gl")]
    pub fn as_gl_format(&self) -> gl::Format {
        unsafe {
            #[allow(clippy::map_clone)]
            self.native().asGLFormat()
        }
    }

    #[deprecated(since = "0.19.0", note = "use as_vk_format()")]
    #[cfg(feature = "vulkan")]
    pub fn vulkan_format(&self) -> Option<vk::Format> {
        self.as_vk_format()
    }

    #[cfg(feature = "vulkan")]
    pub fn as_vk_format(&self) -> Option<vk::Format> {
        let mut r = vk::Format::UNDEFINED;
        unsafe { self.native().asVkFormat(&mut r) }.if_true_some(r)
    }

    #[cfg(feature = "metal")]
    pub fn as_mtl_format(&self) -> mtl::PixelFormat {
        unsafe { self.native().asMtlFormat() }
    }

    #[cfg(feature = "d3d")]
    pub fn as_dxgi_format(&self) -> Option<d3d::DXGI_FORMAT> {
        let mut f = sb::DXGI_FORMAT::DXGI_FORMAT_UNKNOWN;
        unsafe { self.native().asDxgiFormat(&mut f) }
            .if_true_some(d3d::DXGI_FORMAT::from_native_c(f))
    }

    pub fn to_texture_2d(&self) -> Option<Self> {
        let mut new = Self::new();
        unsafe { sb::C_GrBackendFormat_makeTexture2D(self.native(), new.native_mut()) };
        new.is_valid().if_true_some(new)
    }

    pub fn is_valid(&self) -> bool {
        self.native().fValid
    }
}

pub type BackendTexture = Handle<GrBackendTexture>;
unsafe impl Send for BackendTexture {}
unsafe impl Sync for BackendTexture {}

impl NativeDrop for GrBackendTexture {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendTexture_destruct(self) }
    }
}

impl NativeClone for GrBackendTexture {
    fn clone(&self) -> Self {
        construct(|texture| unsafe { sb::C_GrBackendTexture_CopyConstruct(texture, self) })
    }
}

impl Handle<GrBackendTexture> {
    #[cfg(feature = "gl")]
    pub unsafe fn new_gl(
        (width, height): (i32, i32),
        mipmapped: super::Mipmapped,
        gl_info: gl::TextureInfo,
    ) -> Self {
        Self::from_native_if_valid(construct(|texture| {
            sb::C_GrBackendTexture_ConstructGL(texture, width, height, mipmapped, gl_info.native())
        }))
        .unwrap()
    }

    #[cfg(feature = "vulkan")]
    pub unsafe fn new_vulkan((width, height): (i32, i32), vk_info: &vk::ImageInfo) -> Self {
        Self::from_native_if_valid(construct(|texture| {
            sb::C_GrBackendTexture_ConstructVk(texture, width, height, vk_info.native())
        }))
        .unwrap()
    }

    #[cfg(feature = "metal")]
    pub unsafe fn new_metal(
        (width, height): (i32, i32),
        mipmapped: super::Mipmapped,
        mtl_info: &mtl::TextureInfo,
    ) -> Self {
        Self::from_native_if_valid(construct(|texture| {
            sb::C_GrBackendTexture_ConstructMtl(
                texture,
                width,
                height,
                mipmapped,
                mtl_info.native(),
            )
        }))
        .unwrap()
    }

    #[cfg(feature = "d3d")]
    pub fn new_d3d((width, height): (i32, i32), d3d_info: &d3d::TextureResourceInfo) -> Self {
        unsafe {
            Self::from_native_if_valid(construct(|texture| {
                sb::C_GrBackendTexture_ConstructD3D(texture, width, height, d3d_info.native())
            }))
        }
        .unwrap()
    }

    pub(crate) unsafe fn from_native_if_valid(
        backend_texture: GrBackendTexture,
    ) -> Option<BackendTexture> {
        backend_texture
            .fIsValid
            .if_true_then_some(|| BackendTexture::from_native_c(backend_texture))
    }

    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    #[deprecated(since = "0.35.0", note = "Use has_mipmaps()")]
    pub fn has_mip_maps(&self) -> bool {
        self.has_mipmaps()
    }

    pub fn has_mipmaps(&self) -> bool {
        self.native().fMipmapped == GrMipmapped::Yes
    }

    pub fn backend(&self) -> BackendAPI {
        self.native().fBackend
    }

    #[cfg(feature = "gl")]
    pub fn gl_texture_info(&self) -> Option<gl::TextureInfo> {
        unsafe {
            let mut texture_info = gl::TextureInfo::default();
            self.native()
                .getGLTextureInfo(texture_info.native_mut())
                .if_true_some(texture_info)
        }
    }

    #[cfg(feature = "gl")]
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

    #[cfg(feature = "metal")]
    pub fn metal_texture_info(&self) -> Option<mtl::TextureInfo> {
        unsafe {
            let mut texture_info = mtl::TextureInfo::default();
            self.native()
                .getMtlTextureInfo(texture_info.native_mut())
                .if_true_some(texture_info)
        }
    }

    #[cfg(feature = "d3d")]
    pub fn d3d_texture_resource_info(&self) -> Option<d3d::TextureResourceInfo> {
        unsafe {
            let mut info = sb::GrD3DTextureResourceInfo::default();
            self.native()
                .getD3DTextureResourceInfo(&mut info)
                .if_true_then_some(|| {
                    assert!(!info.fResource.fObject.is_null());
                    d3d::TextureResourceInfo::from_native_c(info)
                })
        }
    }

    #[cfg(feature = "d3d")]
    pub fn set_d3d_resource_state(&mut self, resource_state: d3d::ResourceStateEnum) -> &mut Self {
        unsafe { self.native_mut().setD3DResourceState(resource_state) }
        self
    }

    pub fn backend_format(&self) -> Option<BackendFormat> {
        let mut format = BackendFormat::new();
        unsafe { sb::C_GrBackendTexture_getBackendFormat(self.native(), format.native_mut()) };
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
unsafe impl Send for BackendRenderTarget {}
unsafe impl Sync for BackendRenderTarget {}

impl NativeDrop for GrBackendRenderTarget {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendRenderTarget_destruct(self) }
    }
}

impl NativeClone for GrBackendRenderTarget {
    fn clone(&self) -> Self {
        construct(|render_target| unsafe {
            sb::C_GrBackendRenderTarget_CopyConstruct(render_target, self)
        })
    }
}

impl Handle<GrBackendRenderTarget> {
    #[cfg(feature = "gl")]
    pub fn new_gl(
        (width, height): (i32, i32),
        sample_count: impl Into<Option<usize>>,
        stencil_bits: usize,
        info: gl::FramebufferInfo,
    ) -> Self {
        Self::construct(|target| unsafe {
            sb::C_GrBackendRenderTarget_ConstructGL(
                target,
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
    ) -> Self {
        Self::construct(|target| unsafe {
            sb::C_GrBackendRenderTarget_ConstructVk(
                target,
                width,
                height,
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                info.native(),
            )
        })
    }

    #[cfg(feature = "metal")]
    pub fn new_metal(
        (width, height): (i32, i32),
        sample_cnt: i32,
        mtl_info: &mtl::TextureInfo,
    ) -> Self {
        Self::construct(|target| unsafe {
            sb::C_GrBackendRenderTarget_ConstructMtl(
                target,
                width,
                height,
                sample_cnt,
                mtl_info.native(),
            )
        })
    }

    #[cfg(feature = "d3d")]
    pub fn new_d3d((width, height): (i32, i32), d3d_info: &d3d::TextureResourceInfo) -> Self {
        Self::construct(|brt| unsafe {
            sb::C_GrBackendRenderTarget_ConstructD3D(brt, width, height, d3d_info.native())
        })
    }

    pub(crate) fn from_native_c_if_valid(
        native: GrBackendRenderTarget,
    ) -> Option<BackendRenderTarget> {
        let backend_render_target = BackendRenderTarget::from_native_c(native);
        backend_render_target
            .is_valid()
            .if_true_some(backend_render_target)
    }

    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
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
        self.native().fBackend
    }

    pub fn is_framebuffer_only(&self) -> bool {
        self.native().fFramebufferOnly
    }

    #[cfg(feature = "gl")]
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

    #[cfg(feature = "metal")]
    pub fn metal_texture_info(&self) -> Option<mtl::TextureInfo> {
        let mut info = mtl::TextureInfo::default();
        unsafe { self.native().getMtlTextureInfo(info.native_mut()) }.if_true_some(info)
    }

    #[cfg(feature = "d3d")]
    pub fn d3d_texture_resource_info(&self) -> Option<d3d::TextureResourceInfo> {
        let mut info = sb::GrD3DTextureResourceInfo::default();
        unsafe { self.native().getD3DTextureResourceInfo(&mut info) }.if_true_then_some(|| {
            assert!(!info.fResource.fObject.is_null());
            d3d::TextureResourceInfo::from_native_c(info)
        })
    }

    #[cfg(feature = "d3d")]
    pub fn set_d3d_resource_state(&mut self, resource_state: d3d::ResourceStateEnum) -> &mut Self {
        unsafe { self.native_mut().setD3DResourceState(resource_state) }
        self
    }

    pub fn backend_format(&self) -> BackendFormat {
        BackendFormat::construct(|format| unsafe {
            sb::C_GrBackendRenderTarget_getBackendFormat(self.native(), format)
        })
    }

    pub fn set_mutable_stat(&mut self, state: &BackendSurfaceMutableState) {
        unsafe { self.native_mut().setMutableState(state.native()) }
    }

    pub fn is_protected(&self) -> bool {
        unsafe { self.native().isProtected() }
    }

    pub fn is_valid(&self) -> bool {
        self.native().fIsValid
    }
}
