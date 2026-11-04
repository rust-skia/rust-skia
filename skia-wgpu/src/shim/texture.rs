//! Textures, texture views, and samplers.

use skia_bindings as sb;

use super::device::Device;
use super::handle::{Handle, handles};
use super::{convert, wgpu};

pub struct Texture {
    pub texture: wgpu::Texture,
}

pub struct TextureView {
    pub view: wgpu::TextureView,
}

pub struct Sampler {
    pub sampler: wgpu::Sampler,
}

handles! {
    Texture => WGPUTexture: texture_add_ref, texture_release;
    TextureView => WGPUTextureView: texture_view_add_ref, texture_view_release;
    Sampler => WGPUSampler: sampler_add_ref, sampler_release;
}

pub unsafe extern "C" fn device_create_texture(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUTextureDescriptor,
) -> sb::WGPUTexture {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUTextureDescriptor") };
    let view_formats: Vec<wgpu::TextureFormat> =
        unsafe { convert::slice(d.viewFormats, d.viewFormatCount) }
            .iter()
            .map(|&format| convert::required_texture_format(format))
            .collect();
    let texture = device.device.create_texture(&wgpu::TextureDescriptor {
        label: unsafe { convert::label(d.label) },
        size: convert::extent(&d.size),
        mip_level_count: d.mipLevelCount,
        sample_count: d.sampleCount,
        dimension: convert::texture_dimension(d.dimension),
        format: convert::required_texture_format(d.format),
        usage: convert::texture_usages(d.usage),
        view_formats: &view_formats,
    });
    Texture { texture }.into_handle()
}

pub unsafe extern "C" fn texture_create_view(
    texture: sb::WGPUTexture,
    descriptor: *const sb::WGPUTextureViewDescriptor,
) -> sb::WGPUTextureView {
    let texture = unsafe { Texture::get(texture) };
    let descriptor = match unsafe { descriptor.as_ref() } {
        None => wgpu::TextureViewDescriptor::default(),
        Some(d) => {
            unsafe { convert::no_chain(d.nextInChain, "WGPUTextureViewDescriptor") };
            wgpu::TextureViewDescriptor {
                label: unsafe { convert::label(d.label) },
                format: convert::texture_format(d.format),
                dimension: convert::texture_view_dimension(d.dimension),
                usage: (d.usage != sb::WGPUTextureUsage_None)
                    .then(|| convert::texture_usages(d.usage)),
                aspect: convert::texture_aspect(d.aspect),
                base_mip_level: d.baseMipLevel,
                mip_level_count: convert::count(d.mipLevelCount),
                base_array_layer: d.baseArrayLayer,
                array_layer_count: convert::count(d.arrayLayerCount),
            }
        }
    };
    let view = texture.texture.create_view(&descriptor);
    TextureView { view }.into_handle()
}

pub unsafe extern "C" fn texture_get_format(texture: sb::WGPUTexture) -> sb::WGPUTextureFormat {
    convert::c_texture_format(unsafe { Texture::get(texture) }.texture.format())
}

pub unsafe extern "C" fn texture_get_width(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.width()
}

pub unsafe extern "C" fn texture_get_height(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.height()
}

pub unsafe extern "C" fn texture_get_mip_level_count(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.mip_level_count()
}

pub unsafe extern "C" fn texture_get_sample_count(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.sample_count()
}

pub unsafe extern "C" fn texture_get_usage(texture: sb::WGPUTexture) -> sb::WGPUTextureUsage {
    convert::c_texture_usage(unsafe { Texture::get(texture) }.texture.usage())
}

pub unsafe extern "C" fn texture_destroy(texture: sb::WGPUTexture) {
    unsafe { Texture::get(texture) }.texture.destroy();
}

/// See `buffer_set_label`.
pub unsafe extern "C" fn texture_set_label(_texture: sb::WGPUTexture, _label: sb::WGPUStringView) {}

/// See `buffer_set_label`.
pub unsafe extern "C" fn texture_view_set_label(
    _view: sb::WGPUTextureView,
    _label: sb::WGPUStringView,
) {
}

pub unsafe extern "C" fn device_create_sampler(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUSamplerDescriptor,
) -> sb::WGPUSampler {
    let device = unsafe { Device::get(device) };
    let descriptor = match unsafe { descriptor.as_ref() } {
        None => wgpu::SamplerDescriptor::default(),
        Some(d) => {
            unsafe { convert::no_chain(d.nextInChain, "WGPUSamplerDescriptor") };
            wgpu::SamplerDescriptor {
                label: unsafe { convert::label(d.label) },
                address_mode_u: convert::address_mode(d.addressModeU),
                address_mode_v: convert::address_mode(d.addressModeV),
                address_mode_w: convert::address_mode(d.addressModeW),
                mag_filter: convert::filter_mode(d.magFilter),
                min_filter: convert::filter_mode(d.minFilter),
                mipmap_filter: convert::mipmap_filter_mode(d.mipmapFilter),
                lod_min_clamp: d.lodMinClamp,
                lod_max_clamp: d.lodMaxClamp,
                compare: convert::compare_function(d.compare),
                anisotropy_clamp: d.maxAnisotropy,
                border_color: None,
            }
        }
    };
    let sampler = device.device.create_sampler(&descriptor);
    Sampler { sampler }.into_handle()
}
