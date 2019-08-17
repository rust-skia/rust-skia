use super::{Bool32, Filter, Format, FormatFeatureFlags, ImageLayout};
use super::{
    ChromaLocation, DeviceMemory, DeviceSize, Image, ImageTiling, SamplerYcbcrModelConversion,
    SamplerYcbcrRange,
};
use crate::gpu::vk::{CommandBuffer, Device, Instance, Rect2D, RenderPass};
use crate::prelude::*;
use core::mem;
use skia_bindings::{
    C_GrVkAlloc_Construct, C_GrVkAlloc_Equals, C_GrVkImageInfo_Construct, C_GrVkImageInfo_Equals,
    C_GrVkImageInfo_updateImageLayout, C_GrVkYcbcrConversionInfo_Construct,
    C_GrVkYcbcrConversionInfo_Equals, GrVkAlloc_Flag_kMappable_Flag,
    GrVkAlloc_Flag_kNoncoherent_Flag, GrVkDrawableInfo,
};
use skia_bindings::{GrVkAlloc, GrVkBackendMemory};
use skia_bindings::{GrVkImageInfo, GrVkYcbcrConversionInfo};
use std::ffi::CStr;
use std::os::raw;

pub type GraphicsBackendMemory = GrVkBackendMemory;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Alloc {
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub flags: AllocFlag,
    pub backend_memory: GraphicsBackendMemory,
    uses_system_heap: bool,
}

impl NativeTransmutable<GrVkAlloc> for Alloc {}
#[test]
fn test_vk_alloc_layout() {
    Alloc::test_layout()
}

impl PartialEq for Alloc {
    fn eq(&self, other: &Self) -> bool {
        unsafe { C_GrVkAlloc_Equals(self.native(), other.native()) }
    }
}

bitflags! {
    pub struct AllocFlag : u32 {
        const NONCOHERENT = GrVkAlloc_Flag_kNoncoherent_Flag as _;
        const MAPPABLE = GrVkAlloc_Flag_kMappable_Flag as _;
    }
}

impl Alloc {
    pub unsafe fn from_device_memory(
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: AllocFlag,
    ) -> Alloc {
        Alloc::construct(|alloc| C_GrVkAlloc_Construct(alloc, memory, offset, size, flags.bits()))
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct YcbcrConversionInfo {
    pub ycrbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: Filter,
    pub force_explicit_reconsturction: Bool32,
    pub external_format: u64,
    pub external_format_features: FormatFeatureFlags,
}

impl NativeTransmutable<GrVkYcbcrConversionInfo> for YcbcrConversionInfo {}
#[test]
fn test_ycbcr_conversion_info_layout() {
    YcbcrConversionInfo::test_layout()
}

impl PartialEq for YcbcrConversionInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { C_GrVkYcbcrConversionInfo_Equals(self.native(), other.native()) }
    }
}

impl YcbcrConversionInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ycrbcr_model: SamplerYcbcrModelConversion,
        ycbcr_range: SamplerYcbcrRange,
        x_chroma_offset: ChromaLocation,
        y_chroma_offset: ChromaLocation,
        chroma_filter: Filter,
        force_explicit_reconstruction: Bool32,
        external_format: u64,
        external_format_features: FormatFeatureFlags,
    ) -> YcbcrConversionInfo {
        YcbcrConversionInfo::construct(|ci| unsafe {
            C_GrVkYcbcrConversionInfo_Construct(
                ci,
                ycrbcr_model,
                ycbcr_range,
                x_chroma_offset,
                y_chroma_offset,
                chroma_filter,
                force_explicit_reconstruction,
                external_format,
                external_format_features,
            )
        })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ImageInfo {
    pub image: Image,
    pub alloc: Alloc,
    pub tiling: ImageTiling,
    pub layout: ImageLayout,
    pub format: Format,
    pub level_count: u32,
    pub current_queue_familiy: u32,
    pub ycbcr_conversion_info: YcbcrConversionInfo,
}

impl NativeTransmutable<GrVkImageInfo> for ImageInfo {}
#[test]
fn test_image_info_layout() {
    ImageInfo::test_layout()
}

impl Default for ImageInfo {
    fn default() -> Self {
        ImageInfo::from_native(unsafe { GrVkImageInfo::new() })
    }
}

impl PartialEq for ImageInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { C_GrVkImageInfo_Equals(self.native(), other.native()) }
    }
}

impl ImageInfo {
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn from_image(
        image: Image,
        alloc: Alloc,
        tiling: ImageTiling,
        layout: ImageLayout,
        format: Format,
        level_count: u32,
        current_queue_family: Option<u32>,
        ycbcr_conversion_info: Option<YcbcrConversionInfo>,
    ) -> ImageInfo {
        // originally defined as a C macro in vulkan_core.h
        // and therefore not present in the bindings.
        const VK_QUEUE_FAMILY_IGNORED: u32 = 0;

        let current_queue_family = current_queue_family.unwrap_or(VK_QUEUE_FAMILY_IGNORED);
        let ycbcr_conversion_info = ycbcr_conversion_info.unwrap_or_default();
        ImageInfo::construct(|image_info| {
            C_GrVkImageInfo_Construct(
                image_info,
                image,
                alloc.native(),
                tiling,
                layout,
                format,
                level_count,
                current_queue_family,
                ycbcr_conversion_info.native(),
            );
        })
    }

    pub fn from_info(info: &ImageInfo, layout: ImageLayout) -> ImageInfo {
        ImageInfo::from_native(unsafe { GrVkImageInfo::new2(info.native(), layout) })
    }

    pub fn update_image_layout(&mut self, layout: ImageLayout) -> &mut Self {
        unsafe { C_GrVkImageInfo_updateImageLayout(self.native_mut(), layout) }
        self
    }
}

// TODO: Tried to use CStr here, but &CStr needs a lifetime parameter
//       which would make the whole GetProc trait generic.
#[derive(Copy, Clone, Debug)]
pub enum GetProcOf {
    Instance(Instance, *const raw::c_char),
    Device(Device, *const raw::c_char),
}

impl GetProcOf {
    pub unsafe fn name(&self) -> &CStr {
        match *self {
            GetProcOf::Instance(_, name) => CStr::from_ptr(name),
            GetProcOf::Device(_, name) => CStr::from_ptr(name),
        }
    }
}

// TODO: Really would like to see a fn() signature here, but I'm always running
//       into a conflict between extern "C" and extern "system".
pub type GetProcResult = *const raw::c_void;

// GetProc is a trait alias for Fn...
pub trait GetProc: Fn(GetProcOf) -> GetProcResult {}
impl<T> GetProc for T where T: Fn(GetProcOf) -> GetProcResult {}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DrawableInfo {
    secondary_command_buffer: CommandBuffer,
    color_attachment_index: u32,
    compatible_render_pass: RenderPass,
    format: Format,
    draw_bounds: *mut Rect2D,
    image: Image,
}

impl NativeTransmutable<GrVkDrawableInfo> for DrawableInfo {}
#[test]
fn test_drawable_info_layout() {
    DrawableInfo::test_layout()
}
