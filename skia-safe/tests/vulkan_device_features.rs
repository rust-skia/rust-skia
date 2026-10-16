//! Ganesh wraps a texture with a YCbCr conversion only when it receives the device features.

#![cfg(all(
    feature = "ganesh",
    feature = "vulkan",
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

mod common;

use ash::vk::{self, Handle};
use common::vulkan::Vulkan;
use skia_safe::gpu::{self, DirectContext, SurfaceOrigin};
use skia_safe::{AlphaType, ColorType, Image};

const SIZE: (i32, i32) = (16, 16);
const FORMAT: gpu::vk::Format = gpu::vk::Format::G8_B8R8_2PLANE_420_UNORM;

#[test]
fn wraps_a_ycbcr_texture_only_with_the_device_features() {
    let Some(vulkan) = Vulkan::with_sampler_ycbcr_conversion() else {
        eprintln!("skipped: no Vulkan device with YCbCr conversion available");
        return;
    };
    let Some(texture) = YcbcrTexture::new(&vulkan) else {
        eprintln!("skipped: {FORMAT:?} can not be sampled");
        return;
    };
    let (Some(mut with_features), Some(mut without_features)) = (
        vulkan.direct_context_with_device_features(),
        vulkan.direct_context(),
    ) else {
        eprintln!("skipped: no Vulkan context available");
        return;
    };

    assert!(texture.wrap(&mut with_features).is_some());
    assert!(texture.wrap(&mut without_features).is_none());
}

struct YcbcrTexture<'a> {
    device: &'a ash::Device,
    image: vk::Image,
    memory: vk::DeviceMemory,
    backend_texture: gpu::BackendTexture,
}

impl<'a> YcbcrTexture<'a> {
    fn new(vulkan: &'a Vulkan) -> Option<Self> {
        let format = vk::Format::from_raw(FORMAT as i32);
        let format_features = unsafe {
            vulkan
                .instance
                .get_physical_device_format_properties(vulkan.physical_device, format)
        }
        .optimal_tiling_features;
        let required_features = vk::FormatFeatureFlags::SAMPLED_IMAGE
            | vk::FormatFeatureFlags::TRANSFER_SRC
            | vk::FormatFeatureFlags::TRANSFER_DST;
        if !format_features.contains(required_features) {
            return None;
        }

        let device = &vulkan.device;
        let usage = vk::ImageUsageFlags::SAMPLED
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::TRANSFER_DST;
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(vk::Extent3D {
                width: SIZE.0 as u32,
                height: SIZE.1 as u32,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage);
        let image = unsafe { device.create_image(&image_info, None) }.unwrap();
        let requirements = unsafe { device.get_image_memory_requirements(image) };
        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(requirements.memory_type_bits.trailing_zeros());
        let memory = unsafe { device.allocate_memory(&allocate_info, None) }.unwrap();
        unsafe { device.bind_image_memory(image, memory, 0) }.unwrap();

        let identity = gpu::vk::ComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY;
        let ycbcr_conversion_info = gpu::vk::YcbcrConversionInfo::new_with_format(
            FORMAT,
            gpu::vk::SamplerYcbcrModelConversion::YCBCR_709,
            gpu::vk::SamplerYcbcrRange::ITU_NARROW,
            gpu::vk::ChromaLocation::COSITED_EVEN,
            gpu::vk::ChromaLocation::COSITED_EVEN,
            gpu::vk::Filter::NEAREST,
            vk::FALSE,
            gpu::vk::ComponentMapping {
                r: identity,
                g: identity,
                b: identity,
                a: identity,
            },
            format_features.as_raw(),
        );
        let mut image_info = unsafe {
            gpu::vk::ImageInfo::new(
                image.as_raw() as _,
                gpu::vk::Alloc::from_device_memory(
                    memory.as_raw() as _,
                    0,
                    requirements.size,
                    gpu::vk::AllocFlag::empty(),
                ),
                gpu::vk::ImageTiling::OPTIMAL,
                gpu::vk::ImageLayout::UNDEFINED,
                FORMAT,
                1,
                None,
                ycbcr_conversion_info,
                None,
                None,
            )
        };
        image_info.image_usage_flags = usage.as_raw();
        let backend_texture =
            unsafe { gpu::backend_textures::make_vk(SIZE, &image_info, "YCbCr texture") };

        Some(Self {
            device,
            image,
            memory,
            backend_texture,
        })
    }

    fn wrap(&self, context: &mut DirectContext) -> Option<Image> {
        gpu::images::borrow_texture_from(
            context,
            &self.backend_texture,
            SurfaceOrigin::TopLeft,
            ColorType::RGB888x,
            AlphaType::Opaque,
            None,
        )
    }
}

impl Drop for YcbcrTexture<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_image(self.image, None);
            self.device.free_memory(self.memory, None);
        }
    }
}
