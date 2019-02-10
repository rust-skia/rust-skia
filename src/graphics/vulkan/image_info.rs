use rust_skia::{GrVkImageInfo, VkImage, VkImageTiling, VkImageLayout, VkFormat, GrVkYcbcrConversionInfo};
use std::ffi::c_void;
use super::alloc::Alloc;

#[derive(Debug)]
pub struct ImageInfo {
    pub(crate) native: GrVkImageInfo
}

impl ImageInfo {

    pub unsafe fn new(
        image: *mut c_void,
        alloc: &Alloc,
        image_tiling: VkImageTiling,
        image_layout: VkImageLayout,
        format: VkFormat,
        level_count: u32) -> ImageInfo
    {
        // originally defined as a C macro in vulkan_core.h
        // and therefore not present in the bindings.
        const VK_QUEUE_FAMILY_IGNORED : u32 = 0;

        ImageInfo { native: GrVkImageInfo {
            fImage: image as VkImage,
            fAlloc: alloc.native,
            fImageTiling: image_tiling,
            fImageLayout: image_layout,
            fFormat: format,
            fLevelCount: level_count,
            fCurrentQueueFamily: VK_QUEUE_FAMILY_IGNORED,
            fYcbcrConversionInfo: GrVkYcbcrConversionInfo {
                fYcbcrModel: 0,
                fYcbcrRange: 0,
                fXChromaOffset: 0,
                fYChromaOffset: 0,
                fChromaFilter: 0,
                fForceExplicitReconstruction: 0,
                fExternalFormat: 0,
                fExternalFormatFeatures: 0
            }
        }}
    }
}
