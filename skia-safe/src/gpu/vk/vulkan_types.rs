use skia_bindings::{
    self as sb, skgpu_VulkanAlloc, skgpu_VulkanBackendMemory, skgpu_VulkanYcbcrConversionInfo,
};

use crate::{gpu::vk, prelude::*};

#[deprecated(since = "0.76.0", note = "Use BackendMemory")]
pub type GraphicsBackendMemory = skgpu_VulkanBackendMemory;
pub type BackendMemory = skgpu_VulkanBackendMemory;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Alloc {
    pub memory: vk::DeviceMemory,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub flags: AllocFlag,
    pub backend_memory: BackendMemory,
    uses_system_heap: bool,
}
unsafe_send_sync!(Alloc);

native_transmutable!(skgpu_VulkanAlloc, Alloc, alloc_layout);

impl Default for Alloc {
    fn default() -> Self {
        Self {
            memory: vk::NULL_HANDLE.into(),
            offset: 0,
            size: 0,
            flags: AllocFlag::empty(),
            backend_memory: 0,
            uses_system_heap: false,
        }
    }
}

impl PartialEq for Alloc {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_VulkanAlloc_Equals(self.native(), other.native()) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AllocFlag : u32 {
        const NONCOHERENT = sb::skgpu_VulkanAlloc_Flag::kNoncoherent_Flag as _;
        const MAPPABLE = sb::skgpu_VulkanAlloc_Flag::kMappable_Flag as _;
        const LAZILY_ALLOCATED = sb::skgpu_VulkanAlloc_Flag::kLazilyAllocated_Flag as _;
    }
}

impl Alloc {
    /// # Safety
    /// The memory's lifetime is expected to outlive the lifetime of the returned object.
    pub unsafe fn from_device_memory(
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: AllocFlag,
    ) -> Alloc {
        Alloc {
            memory,
            offset,
            size,
            flags,
            backend_memory: 0,
            uses_system_heap: false,
        }
    }
}

#[derive(Copy, Clone, Eq, Debug)]
#[repr(C)]
pub struct YcbcrConversionInfo {
    pub format: vk::Format,
    pub external_format: u64,
    pub ycbcr_model: vk::SamplerYcbcrModelConversion,
    pub ycbcr_range: vk::SamplerYcbcrRange,
    pub x_chroma_offset: vk::ChromaLocation,
    pub y_chroma_offset: vk::ChromaLocation,
    pub chroma_filter: vk::Filter,
    pub force_explicit_reconstruction: vk::Bool32,
    pub format_features: vk::FormatFeatureFlags,
    pub components: vk::ComponentMapping,
}

native_transmutable!(
    skgpu_VulkanYcbcrConversionInfo,
    YcbcrConversionInfo,
    ycbcr_conversion_info_layout
);

impl PartialEq for YcbcrConversionInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_VulkanYcbcrConversionInfo_Equals(self.native(), other.native()) }
    }
}

impl Default for YcbcrConversionInfo {
    fn default() -> Self {
        YcbcrConversionInfo {
            format: vk::Format::UNDEFINED,
            external_format: 0,
            ycbcr_model: vk::SamplerYcbcrModelConversion::RGB_IDENTITY,
            ycbcr_range: vk::SamplerYcbcrRange::ITU_FULL,
            x_chroma_offset: vk::ChromaLocation::COSITED_EVEN,
            y_chroma_offset: vk::ChromaLocation::COSITED_EVEN,
            chroma_filter: vk::Filter::NEAREST,
            force_explicit_reconstruction: 0,
            format_features: 0,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                g: vk::ComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                b: vk::ComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
                a: vk::ComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
            },
        }
    }
}

impl YcbcrConversionInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_format(
        format: vk::Format,
        external_format: u64,
        ycbcr_model: vk::SamplerYcbcrModelConversion,
        ycbcr_range: vk::SamplerYcbcrRange,
        x_chroma_offset: vk::ChromaLocation,
        y_chroma_offset: vk::ChromaLocation,
        chroma_filter: vk::Filter,
        force_explicit_reconstruction: vk::Bool32,
        format_features: vk::FormatFeatureFlags,
    ) -> YcbcrConversionInfo {
        debug_assert!(ycbcr_model != vk::SamplerYcbcrModelConversion::RGB_IDENTITY);
        debug_assert!((format != vk::Format::UNDEFINED) ^ (external_format != 0));
        YcbcrConversionInfo {
            format,
            external_format,
            ycbcr_model,
            ycbcr_range,
            x_chroma_offset,
            y_chroma_offset,
            chroma_filter,
            force_explicit_reconstruction,
            format_features,
            ..YcbcrConversionInfo::default()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ycbcr_model: vk::SamplerYcbcrModelConversion,
        ycbcr_range: vk::SamplerYcbcrRange,
        x_chroma_offset: vk::ChromaLocation,
        y_chroma_offset: vk::ChromaLocation,
        chroma_filter: vk::Filter,
        force_explicit_reconstruction: vk::Bool32,
        external_format: u64,
        external_format_features: vk::FormatFeatureFlags,
    ) -> YcbcrConversionInfo {
        Self::new_with_format(
            vk::Format::UNDEFINED,
            external_format,
            ycbcr_model,
            ycbcr_range,
            x_chroma_offset,
            y_chroma_offset,
            chroma_filter,
            force_explicit_reconstruction,
            external_format_features,
        )
    }

    pub fn is_valid(&self) -> bool {
        self.ycbcr_model != vk::SamplerYcbcrModelConversion::RGB_IDENTITY
            || self.external_format != 0
    }
}
