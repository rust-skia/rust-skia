//! Textures, texture views, and samplers.

use skia_bindings as sb;

use super::device::Device;
use super::handle::{Handle, handles};
use super::{convert, wgpu};

/// Behind a `WGPUTexture`: a texture that Skia created or that the application handed to Skia.
#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    /// The texture's format as WebGPU names it, which `wgpuTextureGetFormat` returns. Some `wgpu`
    /// formats have no WebGPU equivalent, so the conversion happens once, where the texture enters.
    pub format: sb::WGPUTextureFormat,
}

/// Behind a `WGPUTextureView`.
#[derive(Debug)]
pub struct TextureView {
    pub view: wgpu::TextureView,
}

/// Behind a `WGPUSampler`.
#[derive(Debug)]
pub struct Sampler {
    pub sampler: wgpu::Sampler,
}

handles! {
    Texture => WGPUTexture:
        wgpuTextureAddRef: WGPUProcTextureAddRef,
        wgpuTextureRelease: WGPUProcTextureRelease;
    TextureView => WGPUTextureView:
        wgpuTextureViewAddRef: WGPUProcTextureViewAddRef,
        wgpuTextureViewRelease: WGPUProcTextureViewRelease;
    Sampler => WGPUSampler:
        wgpuSamplerAddRef: WGPUProcSamplerAddRef,
        wgpuSamplerRelease: WGPUProcSamplerRelease;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateTexture(
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
    Texture {
        texture,
        format: d.format,
    }
    .into_handle()
}
const _: sb::WGPUProcDeviceCreateTexture = Some(wgpuDeviceCreateTexture);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureCreateView(
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
const _: sb::WGPUProcTextureCreateView = Some(wgpuTextureCreateView);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureGetFormat(texture: sb::WGPUTexture) -> sb::WGPUTextureFormat {
    unsafe { Texture::get(texture) }.format
}
const _: sb::WGPUProcTextureGetFormat = Some(wgpuTextureGetFormat);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureGetWidth(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.width()
}
const _: sb::WGPUProcTextureGetWidth = Some(wgpuTextureGetWidth);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureGetHeight(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.height()
}
const _: sb::WGPUProcTextureGetHeight = Some(wgpuTextureGetHeight);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureGetMipLevelCount(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.mip_level_count()
}
const _: sb::WGPUProcTextureGetMipLevelCount = Some(wgpuTextureGetMipLevelCount);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureGetSampleCount(texture: sb::WGPUTexture) -> u32 {
    unsafe { Texture::get(texture) }.texture.sample_count()
}
const _: sb::WGPUProcTextureGetSampleCount = Some(wgpuTextureGetSampleCount);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureGetUsage(texture: sb::WGPUTexture) -> sb::WGPUTextureUsage {
    convert::c_texture_usage(unsafe { Texture::get(texture) }.texture.usage())
}
const _: sb::WGPUProcTextureGetUsage = Some(wgpuTextureGetUsage);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureDestroy(texture: sb::WGPUTexture) {
    unsafe { Texture::get(texture) }.texture.destroy();
}
const _: sb::WGPUProcTextureDestroy = Some(wgpuTextureDestroy);

/// See `wgpuBufferSetLabel`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureSetLabel(
    _texture: sb::WGPUTexture,
    _label: sb::WGPUStringView,
) {
}
const _: sb::WGPUProcTextureSetLabel = Some(wgpuTextureSetLabel);

/// See `wgpuBufferSetLabel`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuTextureViewSetLabel(
    _view: sb::WGPUTextureView,
    _label: sb::WGPUStringView,
) {
}
const _: sb::WGPUProcTextureViewSetLabel = Some(wgpuTextureViewSetLabel);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateSampler(
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
const _: sb::WGPUProcDeviceCreateSampler = Some(wgpuDeviceCreateSampler);
