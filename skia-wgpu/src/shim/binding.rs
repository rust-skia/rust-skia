//! Bind group layouts, bind groups, and pipeline layouts.

use std::num::{NonZeroU32, NonZeroU64};

use skia_bindings as sb;

use super::buffer::Buffer;
use super::device::Device;
use super::handle::{Handle, handles};
use super::texture::{Sampler, TextureView};
use super::{convert, wgpu};

/// Behind a `WGPUBindGroupLayout`.
#[derive(Debug)]
pub struct BindGroupLayout {
    pub layout: wgpu::BindGroupLayout,
}

/// Behind a `WGPUBindGroup`.
#[derive(Debug)]
pub struct BindGroup {
    pub group: wgpu::BindGroup,
}

/// Behind a `WGPUPipelineLayout`.
#[derive(Debug)]
pub struct PipelineLayout {
    pub layout: wgpu::PipelineLayout,
}

handles! {
    BindGroupLayout => WGPUBindGroupLayout:
        wgpuBindGroupLayoutAddRef: WGPUProcBindGroupLayoutAddRef,
        wgpuBindGroupLayoutRelease: WGPUProcBindGroupLayoutRelease;
    BindGroup => WGPUBindGroup:
        wgpuBindGroupAddRef: WGPUProcBindGroupAddRef,
        wgpuBindGroupRelease: WGPUProcBindGroupRelease;
    PipelineLayout => WGPUPipelineLayout:
        wgpuPipelineLayoutAddRef: WGPUProcPipelineLayoutAddRef,
        wgpuPipelineLayoutRelease: WGPUProcPipelineLayoutRelease;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroupLayout(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUBindGroupLayoutDescriptor,
) -> sb::WGPUBindGroupLayout {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUBindGroupLayoutDescriptor") };
    let entries: Vec<wgpu::BindGroupLayoutEntry> =
        unsafe { convert::slice(d.entries, d.entryCount) }
            .iter()
            .map(|entry| unsafe { layout_entry(entry) })
            .collect();
    let layout = device
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: unsafe { convert::label(d.label) },
            entries: &entries,
        });
    BindGroupLayout { layout }.into_handle()
}
const _: sb::WGPUProcDeviceCreateBindGroupLayout = Some(wgpuDeviceCreateBindGroupLayout);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreateBindGroup(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUBindGroupDescriptor,
) -> sb::WGPUBindGroup {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUBindGroupDescriptor") };
    let layout = unsafe { BindGroupLayout::get(d.layout) };
    let entries: Vec<wgpu::BindGroupEntry> = unsafe { convert::slice(d.entries, d.entryCount) }
        .iter()
        .map(|entry| unsafe { group_entry(entry) })
        .collect();
    let group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: unsafe { convert::label(d.label) },
        layout: &layout.layout,
        entries: &entries,
    });
    BindGroup { group }.into_handle()
}
const _: sb::WGPUProcDeviceCreateBindGroup = Some(wgpuDeviceCreateBindGroup);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgpuDeviceCreatePipelineLayout(
    device: sb::WGPUDevice,
    descriptor: *const sb::WGPUPipelineLayoutDescriptor,
) -> sb::WGPUPipelineLayout {
    let device = unsafe { Device::get(device) };
    let d = unsafe { &*descriptor };
    unsafe { convert::no_chain(d.nextInChain, "WGPUPipelineLayoutDescriptor") };
    let layouts: Vec<Option<&wgpu::BindGroupLayout>> =
        unsafe { convert::slice(d.bindGroupLayouts, d.bindGroupLayoutCount) }
            .iter()
            .map(|&layout| {
                unsafe { super::handle::get_optional::<BindGroupLayout, _>(layout) }
                    .map(|layout| &layout.layout)
            })
            .collect();
    let layout = device
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: unsafe { convert::label(d.label) },
            bind_group_layouts: &layouts,
            immediate_size: d.immediateSize,
        });
    PipelineLayout { layout }.into_handle()
}
const _: sb::WGPUProcDeviceCreatePipelineLayout = Some(wgpuDeviceCreatePipelineLayout);

/// Converts a layout entry. Exactly one of its four binding layouts is in use. The others have the
/// type `BindingNotUsed`, and `Undefined` in the one in use stands for WebGPU's default.
unsafe fn layout_entry(c: &sb::WGPUBindGroupLayoutEntry) -> wgpu::BindGroupLayoutEntry {
    unsafe { convert::no_chain(c.nextInChain, "WGPUBindGroupLayoutEntry") };
    use sb::WGPUBufferBindingType as B;
    use sb::WGPUSamplerBindingType as S;
    use sb::WGPUStorageTextureAccess as A;
    use sb::WGPUTextureSampleType as T;
    let buffer = match c.buffer.type_ {
        B::WGPUBufferBindingType_BindingNotUsed => None,
        B::WGPUBufferBindingType_Undefined | B::WGPUBufferBindingType_Uniform => {
            Some(wgpu::BufferBindingType::Uniform)
        }
        B::WGPUBufferBindingType_Storage => {
            Some(wgpu::BufferBindingType::Storage { read_only: false })
        }
        B::WGPUBufferBindingType_ReadOnlyStorage => {
            Some(wgpu::BufferBindingType::Storage { read_only: true })
        }
        other @ B::WGPUBufferBindingType_Force32 => panic!("unsupported {other:?}"),
    }
    .map(|ty| wgpu::BindingType::Buffer {
        ty,
        has_dynamic_offset: convert::bool(c.buffer.hasDynamicOffset),
        min_binding_size: NonZeroU64::new(c.buffer.minBindingSize),
    });
    let sampler = match c.sampler.type_ {
        S::WGPUSamplerBindingType_BindingNotUsed => None,
        S::WGPUSamplerBindingType_Undefined | S::WGPUSamplerBindingType_Filtering => {
            Some(wgpu::SamplerBindingType::Filtering)
        }
        S::WGPUSamplerBindingType_NonFiltering => Some(wgpu::SamplerBindingType::NonFiltering),
        S::WGPUSamplerBindingType_Comparison => Some(wgpu::SamplerBindingType::Comparison),
        other @ S::WGPUSamplerBindingType_Force32 => panic!("unsupported {other:?}"),
    }
    .map(wgpu::BindingType::Sampler);
    let texture = match c.texture.sampleType {
        T::WGPUTextureSampleType_BindingNotUsed => None,
        T::WGPUTextureSampleType_Undefined | T::WGPUTextureSampleType_Float => {
            Some(wgpu::TextureSampleType::Float { filterable: true })
        }
        T::WGPUTextureSampleType_UnfilterableFloat => {
            Some(wgpu::TextureSampleType::Float { filterable: false })
        }
        T::WGPUTextureSampleType_Depth => Some(wgpu::TextureSampleType::Depth),
        T::WGPUTextureSampleType_Sint => Some(wgpu::TextureSampleType::Sint),
        T::WGPUTextureSampleType_Uint => Some(wgpu::TextureSampleType::Uint),
        other @ T::WGPUTextureSampleType_Force32 => panic!("unsupported {other:?}"),
    }
    .map(|sample_type| wgpu::BindingType::Texture {
        sample_type,
        view_dimension: convert::texture_view_dimension(c.texture.viewDimension)
            .unwrap_or(wgpu::TextureViewDimension::D2),
        multisampled: convert::bool(c.texture.multisampled),
    });
    let storage_texture = match c.storageTexture.access {
        A::WGPUStorageTextureAccess_BindingNotUsed => None,
        A::WGPUStorageTextureAccess_Undefined | A::WGPUStorageTextureAccess_WriteOnly => {
            Some(wgpu::StorageTextureAccess::WriteOnly)
        }
        A::WGPUStorageTextureAccess_ReadOnly => Some(wgpu::StorageTextureAccess::ReadOnly),
        A::WGPUStorageTextureAccess_ReadWrite => Some(wgpu::StorageTextureAccess::ReadWrite),
        other @ A::WGPUStorageTextureAccess_Force32 => panic!("unsupported {other:?}"),
    }
    .map(|access| wgpu::BindingType::StorageTexture {
        access,
        format: convert::required_texture_format(c.storageTexture.format),
        view_dimension: convert::texture_view_dimension(c.storageTexture.viewDimension)
            .unwrap_or(wgpu::TextureViewDimension::D2),
    });

    let mut types = [buffer, sampler, texture, storage_texture]
        .into_iter()
        .flatten();
    let ty = types
        .next()
        .unwrap_or_else(|| panic!("bind group layout entry {} binds nothing", c.binding));
    assert!(
        types.next().is_none(),
        "bind group layout entry {} binds more than one resource",
        c.binding
    );
    wgpu::BindGroupLayoutEntry {
        binding: c.binding,
        visibility: convert::shader_stages(c.visibility),
        ty,
        count: NonZeroU32::new(c.bindingArraySize),
    }
}

/// Converts a bind group entry, which sets exactly one of buffer, sampler, and texture view.
unsafe fn group_entry<'a>(c: &sb::WGPUBindGroupEntry) -> wgpu::BindGroupEntry<'a> {
    unsafe { convert::no_chain(c.nextInChain, "WGPUBindGroupEntry") };
    let resource = if !c.buffer.is_null() {
        wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &unsafe { Buffer::get(c.buffer) }.buffer,
            offset: c.offset,
            size: convert::size(c.size).and_then(NonZeroU64::new),
        })
    } else if !c.sampler.is_null() {
        wgpu::BindingResource::Sampler(&unsafe { Sampler::get(c.sampler) }.sampler)
    } else if !c.textureView.is_null() {
        wgpu::BindingResource::TextureView(&unsafe { TextureView::get(c.textureView) }.view)
    } else {
        panic!("bind group entry {} binds nothing", c.binding);
    };
    wgpu::BindGroupEntry {
        binding: c.binding,
        resource,
    }
}
