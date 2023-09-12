pub mod backend_formats {
    use skia_bindings as sb;

    use crate::{
        gpu::{
            vk::{Format, YcbcrConversionInfo},
            BackendFormat,
        },
        prelude::*,
    };

    pub fn make_vk(
        format: Format,
        will_use_drm_format_modifiers: impl Into<Option<bool>>,
    ) -> BackendFormat {
        let will_use_drm_format_modifiers = will_use_drm_format_modifiers.into().unwrap_or(false);
        BackendFormat::construct(|bf| unsafe {
            sb::C_GrBackendFormat_ConstructVk(bf, format, will_use_drm_format_modifiers)
        })
        .assert_valid()
    }

    pub fn make_vk_ycbcr(
        conversion_info: &YcbcrConversionInfo,
        will_use_drm_format_modifiers: impl Into<Option<bool>>,
    ) -> BackendFormat {
        let will_use_drm_format_modifiers = will_use_drm_format_modifiers.into().unwrap_or(false);
        BackendFormat::construct(|bf| unsafe {
            sb::C_GrBackendFormat_ConstructVk2(
                bf,
                conversion_info.native(),
                will_use_drm_format_modifiers,
            )
        })
        .assert_valid()
    }

    pub fn as_vk_format(backend_format: &BackendFormat) -> Option<Format> {
        let mut r = Format::UNDEFINED;
        unsafe { sb::C_GrBackendFormats_AsVkFormat(backend_format.native(), &mut r) }
            .if_true_some(r)
    }

    pub fn get_vk_ycbcr_conversion_info(
        backend_format: &BackendFormat,
    ) -> Option<&YcbcrConversionInfo> {
        unsafe {
            YcbcrConversionInfo::from_native_ptr(sb::C_GrBackendFormats_GetVkYcbcrConversionInfo(
                backend_format.native(),
            ))
            .into_option()
            .map(|r| &*r)
        }
    }
}

pub mod backend_textures {
    use skia_bindings as sb;

    use crate::{
        gpu::{
            vk::{ImageInfo, ImageLayout},
            BackendTexture,
        },
        prelude::*,
    };

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn make_vk(
        (width, height): (i32, i32),
        vk_info: &ImageInfo,
        label: impl AsRef<str>,
    ) -> BackendTexture {
        let label = label.as_ref().as_bytes();
        BackendTexture::from_native_if_valid(sb::C_GrBackendTexture_newVk(
            width,
            height,
            vk_info.native(),
            label.as_ptr() as _,
            label.len(),
        ))
        .unwrap()
    }

    pub fn get_vk_image_info(texture: &BackendTexture) -> Option<ImageInfo> {
        unsafe {
            // constructor not available.
            let mut image_info = ImageInfo::default();
            texture
                .native()
                .getVkImageInfo(image_info.native_mut())
                .if_true_some(image_info)
        }
    }

    pub fn set_vk_image_layout(
        texture: &mut BackendTexture,
        layout: ImageLayout,
    ) -> &mut BackendTexture {
        unsafe { texture.native_mut().setVkImageLayout(layout) }
        texture
    }
}

pub mod backend_render_targets {
    use skia_bindings as sb;

    use crate::{
        gpu::{
            vk::{ImageInfo, ImageLayout},
            BackendRenderTarget,
        },
        prelude::*,
    };

    pub fn make_vk((width, height): (i32, i32), info: &ImageInfo) -> BackendRenderTarget {
        BackendRenderTarget::construct(|target| unsafe {
            sb::C_GrBackendRenderTargets_ConstructVk(target, width, height, info.native())
        })
    }

    pub fn get_vk_image_info(target: &BackendRenderTarget) -> Option<ImageInfo> {
        let mut info = ImageInfo::default();
        unsafe { target.native().getVkImageInfo(info.native_mut()) }.if_true_some(info)
    }

    pub fn set_vk_image_layout(
        target: &mut BackendRenderTarget,
        layout: ImageLayout,
    ) -> &mut BackendRenderTarget {
        unsafe { target.native_mut().setVkImageLayout(layout) }
        target
    }
}
