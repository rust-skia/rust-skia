//! Conversions between the WebGPU C types of Dawn's `webgpu.h` and the `wgpu` types.
//!
//! The C enums have an `Undefined` variant that stands for WebGPU's default. The conversions map
//! it to that default, or to `None` where `wgpu` takes an `Option`. A value that `wgpu` cannot
//! represent, such as one of Dawn's own texture formats, panics with its name. The shim reports
//! every Dawn feature as unsupported, so Skia only passes such a value when that report is wrong.

use std::ffi::{CStr, CString, c_char};
use std::ops::{BitAnd, BitOr};

use skia_bindings as sb;

use super::wgpu;

/// Reads a label or an entry point name. Empty strings count as absent, as in WebGPU.
///
/// # Safety
///
/// See [`str`].
pub unsafe fn label<'a>(view: sb::WGPUStringView) -> Option<&'a str> {
    unsafe { str(view) }.filter(|s| !s.is_empty())
}

/// Reads a C string view. A null `data` pointer means the string is absent.
///
/// # Safety
///
/// `view` must point to UTF-8 text that lives for `'a`.
pub unsafe fn str<'a>(view: sb::WGPUStringView) -> Option<&'a str> {
    if view.data.is_null() {
        return None;
    }
    // `WGPU_STRLEN` marks a null-terminated string. Bindgen types the constant as `i32`.
    let bytes = if view.length == usize::MAX {
        unsafe { CStr::from_ptr(view.data).to_bytes() }
    } else {
        unsafe { std::slice::from_raw_parts(view.data as *const u8, view.length) }
    };
    Some(std::str::from_utf8(bytes).expect("WebGPU string is not UTF-8"))
}

/// A string that the shim passes to a C callback, which may only read it during the call.
#[derive(Debug)]
pub struct CallbackString(CString);

impl CallbackString {
    pub fn new(s: &str) -> Self {
        Self(CString::new(s.replace('\0', " ")).unwrap())
    }

    pub fn view(&self) -> sb::WGPUStringView {
        sb::WGPUStringView {
            data: self.0.as_ptr(),
            length: self.0.as_bytes().len(),
        }
    }
}

/// The empty string view, for callbacks without a message.
pub fn empty_string() -> sb::WGPUStringView {
    sb::WGPUStringView {
        data: std::ptr::null::<c_char>(),
        length: 0,
    }
}

pub fn bool(value: sb::WGPUBool) -> bool {
    value != 0
}

pub fn c_bool(value: bool) -> sb::WGPUBool {
    value.into()
}

/// `WGPU_WHOLE_SIZE`, which bindgen types as `i32`.
pub const WHOLE_SIZE: u64 = u64::MAX;

/// Converts a WebGPU size, where `WGPU_WHOLE_SIZE` means "to the end", into `wgpu`'s `None`.
pub fn size(size: u64) -> Option<u64> {
    (size != WHOLE_SIZE).then_some(size)
}

/// Converts one of WebGPU's `*_UNDEFINED` counts (`u32::MAX`) into `None`.
pub fn count(count: u32) -> Option<u32> {
    (count != u32::MAX).then_some(count)
}

//
// Flags
//

const BUFFER_USAGES: &[(u64, wgpu::BufferUsages)] = &[
    (sb::WGPUBufferUsage_MapRead, wgpu::BufferUsages::MAP_READ),
    (sb::WGPUBufferUsage_MapWrite, wgpu::BufferUsages::MAP_WRITE),
    (sb::WGPUBufferUsage_CopySrc, wgpu::BufferUsages::COPY_SRC),
    (sb::WGPUBufferUsage_CopyDst, wgpu::BufferUsages::COPY_DST),
    (sb::WGPUBufferUsage_Index, wgpu::BufferUsages::INDEX),
    (sb::WGPUBufferUsage_Vertex, wgpu::BufferUsages::VERTEX),
    (sb::WGPUBufferUsage_Uniform, wgpu::BufferUsages::UNIFORM),
    (sb::WGPUBufferUsage_Storage, wgpu::BufferUsages::STORAGE),
    (sb::WGPUBufferUsage_Indirect, wgpu::BufferUsages::INDIRECT),
    (
        sb::WGPUBufferUsage_QueryResolve,
        wgpu::BufferUsages::QUERY_RESOLVE,
    ),
];

pub fn buffer_usages(c: sb::WGPUBufferUsage) -> wgpu::BufferUsages {
    flags(
        c,
        BUFFER_USAGES,
        wgpu::BufferUsages::empty(),
        "WGPUBufferUsage",
    )
}

pub fn c_buffer_usage(w: wgpu::BufferUsages) -> sb::WGPUBufferUsage {
    c_flags(w, BUFFER_USAGES)
}

const TEXTURE_USAGES: &[(u64, wgpu::TextureUsages)] = &[
    (sb::WGPUTextureUsage_CopySrc, wgpu::TextureUsages::COPY_SRC),
    (sb::WGPUTextureUsage_CopyDst, wgpu::TextureUsages::COPY_DST),
    (
        sb::WGPUTextureUsage_TextureBinding,
        wgpu::TextureUsages::TEXTURE_BINDING,
    ),
    (
        sb::WGPUTextureUsage_StorageBinding,
        wgpu::TextureUsages::STORAGE_BINDING,
    ),
    (
        sb::WGPUTextureUsage_RenderAttachment,
        wgpu::TextureUsages::RENDER_ATTACHMENT,
    ),
];

pub fn texture_usages(c: sb::WGPUTextureUsage) -> wgpu::TextureUsages {
    flags(
        c,
        TEXTURE_USAGES,
        wgpu::TextureUsages::empty(),
        "WGPUTextureUsage",
    )
}

pub fn c_texture_usage(w: wgpu::TextureUsages) -> sb::WGPUTextureUsage {
    c_flags(w, TEXTURE_USAGES)
}

const SHADER_STAGES: &[(u64, wgpu::ShaderStages)] = &[
    (sb::WGPUShaderStage_Vertex, wgpu::ShaderStages::VERTEX),
    (sb::WGPUShaderStage_Fragment, wgpu::ShaderStages::FRAGMENT),
    (sb::WGPUShaderStage_Compute, wgpu::ShaderStages::COMPUTE),
];

pub fn shader_stages(c: sb::WGPUShaderStage) -> wgpu::ShaderStages {
    flags(
        c,
        SHADER_STAGES,
        wgpu::ShaderStages::empty(),
        "WGPUShaderStage",
    )
}

const COLOR_WRITES: &[(u64, wgpu::ColorWrites)] = &[
    (sb::WGPUColorWriteMask_Red, wgpu::ColorWrites::RED),
    (sb::WGPUColorWriteMask_Green, wgpu::ColorWrites::GREEN),
    (sb::WGPUColorWriteMask_Blue, wgpu::ColorWrites::BLUE),
    (sb::WGPUColorWriteMask_Alpha, wgpu::ColorWrites::ALPHA),
];

pub fn color_writes(c: sb::WGPUColorWriteMask) -> wgpu::ColorWrites {
    flags(
        c,
        COLOR_WRITES,
        wgpu::ColorWrites::empty(),
        "WGPUColorWriteMask",
    )
}

pub fn map_mode(c: sb::WGPUMapMode) -> wgpu::MapMode {
    match c {
        sb::WGPUMapMode_Read => wgpu::MapMode::Read,
        sb::WGPUMapMode_Write => wgpu::MapMode::Write,
        _ => panic!("unsupported WGPUMapMode {c:#x}"),
    }
}

//
// Texture formats
//

/// Generates both directions of the texture format conversion from one table.
macro_rules! texture_formats {
    ($($c:ident => $w:expr,)* ; unsupported: $($unsupported:ident),* $(,)?) => {
        /// Converts a texture format. `None` stands for `Undefined`, which WebGPU uses for "the
        /// format of the texture" or "no attachment". A format that `wgpu` lacks panics. Listing
        /// every variant makes a format added to Dawn's header a compile error.
        pub fn texture_format(c: sb::WGPUTextureFormat) -> Option<wgpu::TextureFormat> {
            use sb::WGPUTextureFormat as F;
            match c {
                $(F::$c => Some($w),)*
                F::WGPUTextureFormat_Undefined => None,
                $(F::$unsupported)|* => panic!("unsupported texture format {c:?}"),
            }
        }

        /// Converts a `wgpu` texture format. `None` stands for the formats that have no WebGPU
        /// equivalent, such as `wgpu`'s native-only formats.
        pub fn c_texture_format(w: wgpu::TextureFormat) -> Option<sb::WGPUTextureFormat> {
            use sb::WGPUTextureFormat as F;
            $(if w == $w {
                return Some(F::$c);
            })*
            None
        }
    };
}

macro_rules! astc {
    ($block:ident, $channel:ident) => {
        wgpu::TextureFormat::Astc {
            block: wgpu::AstcBlock::$block,
            channel: wgpu::AstcChannel::$channel,
        }
    };
}

use wgpu::TextureFormat as TF;

texture_formats! {
    WGPUTextureFormat_R8Unorm => TF::R8Unorm,
    WGPUTextureFormat_R8Snorm => TF::R8Snorm,
    WGPUTextureFormat_R8Uint => TF::R8Uint,
    WGPUTextureFormat_R8Sint => TF::R8Sint,
    WGPUTextureFormat_R16Unorm => TF::R16Unorm,
    WGPUTextureFormat_R16Snorm => TF::R16Snorm,
    WGPUTextureFormat_R16Uint => TF::R16Uint,
    WGPUTextureFormat_R16Sint => TF::R16Sint,
    WGPUTextureFormat_R16Float => TF::R16Float,
    WGPUTextureFormat_RG8Unorm => TF::Rg8Unorm,
    WGPUTextureFormat_RG8Snorm => TF::Rg8Snorm,
    WGPUTextureFormat_RG8Uint => TF::Rg8Uint,
    WGPUTextureFormat_RG8Sint => TF::Rg8Sint,
    WGPUTextureFormat_R32Float => TF::R32Float,
    WGPUTextureFormat_R32Uint => TF::R32Uint,
    WGPUTextureFormat_R32Sint => TF::R32Sint,
    WGPUTextureFormat_RG16Unorm => TF::Rg16Unorm,
    WGPUTextureFormat_RG16Snorm => TF::Rg16Snorm,
    WGPUTextureFormat_RG16Uint => TF::Rg16Uint,
    WGPUTextureFormat_RG16Sint => TF::Rg16Sint,
    WGPUTextureFormat_RG16Float => TF::Rg16Float,
    WGPUTextureFormat_RGBA8Unorm => TF::Rgba8Unorm,
    WGPUTextureFormat_RGBA8UnormSrgb => TF::Rgba8UnormSrgb,
    WGPUTextureFormat_RGBA8Snorm => TF::Rgba8Snorm,
    WGPUTextureFormat_RGBA8Uint => TF::Rgba8Uint,
    WGPUTextureFormat_RGBA8Sint => TF::Rgba8Sint,
    WGPUTextureFormat_BGRA8Unorm => TF::Bgra8Unorm,
    WGPUTextureFormat_BGRA8UnormSrgb => TF::Bgra8UnormSrgb,
    WGPUTextureFormat_RGB10A2Uint => TF::Rgb10a2Uint,
    WGPUTextureFormat_RGB10A2Unorm => TF::Rgb10a2Unorm,
    WGPUTextureFormat_RG11B10Ufloat => TF::Rg11b10Ufloat,
    WGPUTextureFormat_RGB9E5Ufloat => TF::Rgb9e5Ufloat,
    WGPUTextureFormat_RG32Float => TF::Rg32Float,
    WGPUTextureFormat_RG32Uint => TF::Rg32Uint,
    WGPUTextureFormat_RG32Sint => TF::Rg32Sint,
    WGPUTextureFormat_RGBA16Unorm => TF::Rgba16Unorm,
    WGPUTextureFormat_RGBA16Snorm => TF::Rgba16Snorm,
    WGPUTextureFormat_RGBA16Uint => TF::Rgba16Uint,
    WGPUTextureFormat_RGBA16Sint => TF::Rgba16Sint,
    WGPUTextureFormat_RGBA16Float => TF::Rgba16Float,
    WGPUTextureFormat_RGBA32Float => TF::Rgba32Float,
    WGPUTextureFormat_RGBA32Uint => TF::Rgba32Uint,
    WGPUTextureFormat_RGBA32Sint => TF::Rgba32Sint,
    WGPUTextureFormat_Stencil8 => TF::Stencil8,
    WGPUTextureFormat_Depth16Unorm => TF::Depth16Unorm,
    WGPUTextureFormat_Depth24Plus => TF::Depth24Plus,
    WGPUTextureFormat_Depth24PlusStencil8 => TF::Depth24PlusStencil8,
    WGPUTextureFormat_Depth32Float => TF::Depth32Float,
    WGPUTextureFormat_Depth32FloatStencil8 => TF::Depth32FloatStencil8,
    WGPUTextureFormat_BC1RGBAUnorm => TF::Bc1RgbaUnorm,
    WGPUTextureFormat_BC1RGBAUnormSrgb => TF::Bc1RgbaUnormSrgb,
    WGPUTextureFormat_BC2RGBAUnorm => TF::Bc2RgbaUnorm,
    WGPUTextureFormat_BC2RGBAUnormSrgb => TF::Bc2RgbaUnormSrgb,
    WGPUTextureFormat_BC3RGBAUnorm => TF::Bc3RgbaUnorm,
    WGPUTextureFormat_BC3RGBAUnormSrgb => TF::Bc3RgbaUnormSrgb,
    WGPUTextureFormat_BC4RUnorm => TF::Bc4RUnorm,
    WGPUTextureFormat_BC4RSnorm => TF::Bc4RSnorm,
    WGPUTextureFormat_BC5RGUnorm => TF::Bc5RgUnorm,
    WGPUTextureFormat_BC5RGSnorm => TF::Bc5RgSnorm,
    WGPUTextureFormat_BC6HRGBUfloat => TF::Bc6hRgbUfloat,
    WGPUTextureFormat_BC6HRGBFloat => TF::Bc6hRgbFloat,
    WGPUTextureFormat_BC7RGBAUnorm => TF::Bc7RgbaUnorm,
    WGPUTextureFormat_BC7RGBAUnormSrgb => TF::Bc7RgbaUnormSrgb,
    WGPUTextureFormat_ETC2RGB8Unorm => TF::Etc2Rgb8Unorm,
    WGPUTextureFormat_ETC2RGB8UnormSrgb => TF::Etc2Rgb8UnormSrgb,
    WGPUTextureFormat_ETC2RGB8A1Unorm => TF::Etc2Rgb8A1Unorm,
    WGPUTextureFormat_ETC2RGB8A1UnormSrgb => TF::Etc2Rgb8A1UnormSrgb,
    WGPUTextureFormat_ETC2RGBA8Unorm => TF::Etc2Rgba8Unorm,
    WGPUTextureFormat_ETC2RGBA8UnormSrgb => TF::Etc2Rgba8UnormSrgb,
    WGPUTextureFormat_EACR11Unorm => TF::EacR11Unorm,
    WGPUTextureFormat_EACR11Snorm => TF::EacR11Snorm,
    WGPUTextureFormat_EACRG11Unorm => TF::EacRg11Unorm,
    WGPUTextureFormat_EACRG11Snorm => TF::EacRg11Snorm,
    WGPUTextureFormat_ASTC4x4Unorm => astc!(B4x4, Unorm),
    WGPUTextureFormat_ASTC4x4UnormSrgb => astc!(B4x4, UnormSrgb),
    WGPUTextureFormat_ASTC5x4Unorm => astc!(B5x4, Unorm),
    WGPUTextureFormat_ASTC5x4UnormSrgb => astc!(B5x4, UnormSrgb),
    WGPUTextureFormat_ASTC5x5Unorm => astc!(B5x5, Unorm),
    WGPUTextureFormat_ASTC5x5UnormSrgb => astc!(B5x5, UnormSrgb),
    WGPUTextureFormat_ASTC6x5Unorm => astc!(B6x5, Unorm),
    WGPUTextureFormat_ASTC6x5UnormSrgb => astc!(B6x5, UnormSrgb),
    WGPUTextureFormat_ASTC6x6Unorm => astc!(B6x6, Unorm),
    WGPUTextureFormat_ASTC6x6UnormSrgb => astc!(B6x6, UnormSrgb),
    WGPUTextureFormat_ASTC8x5Unorm => astc!(B8x5, Unorm),
    WGPUTextureFormat_ASTC8x5UnormSrgb => astc!(B8x5, UnormSrgb),
    WGPUTextureFormat_ASTC8x6Unorm => astc!(B8x6, Unorm),
    WGPUTextureFormat_ASTC8x6UnormSrgb => astc!(B8x6, UnormSrgb),
    WGPUTextureFormat_ASTC8x8Unorm => astc!(B8x8, Unorm),
    WGPUTextureFormat_ASTC8x8UnormSrgb => astc!(B8x8, UnormSrgb),
    WGPUTextureFormat_ASTC10x5Unorm => astc!(B10x5, Unorm),
    WGPUTextureFormat_ASTC10x5UnormSrgb => astc!(B10x5, UnormSrgb),
    WGPUTextureFormat_ASTC10x6Unorm => astc!(B10x6, Unorm),
    WGPUTextureFormat_ASTC10x6UnormSrgb => astc!(B10x6, UnormSrgb),
    WGPUTextureFormat_ASTC10x8Unorm => astc!(B10x8, Unorm),
    WGPUTextureFormat_ASTC10x8UnormSrgb => astc!(B10x8, UnormSrgb),
    WGPUTextureFormat_ASTC10x10Unorm => astc!(B10x10, Unorm),
    WGPUTextureFormat_ASTC10x10UnormSrgb => astc!(B10x10, UnormSrgb),
    WGPUTextureFormat_ASTC12x10Unorm => astc!(B12x10, Unorm),
    WGPUTextureFormat_ASTC12x10UnormSrgb => astc!(B12x10, UnormSrgb),
    WGPUTextureFormat_ASTC12x12Unorm => astc!(B12x12, Unorm),
    WGPUTextureFormat_ASTC12x12UnormSrgb => astc!(B12x12, UnormSrgb),
    ;
    // Dawn's multi-planar formats and bindgen's sentinel `Force32`.
    unsupported: WGPUTextureFormat_R8BG8Biplanar420Unorm,
    WGPUTextureFormat_R10X6BG10X6Biplanar420Unorm,
    WGPUTextureFormat_R8BG8A8Triplanar420Unorm,
    WGPUTextureFormat_R8BG8Biplanar422Unorm,
    WGPUTextureFormat_R8BG8Biplanar444Unorm,
    WGPUTextureFormat_R10X6BG10X6Biplanar422Unorm,
    WGPUTextureFormat_R10X6BG10X6Biplanar444Unorm,
    WGPUTextureFormat_OpaqueYCbCrAndroid,
    WGPUTextureFormat_Force32,
}

/// Converts a texture format that must be defined.
pub fn required_texture_format(c: sb::WGPUTextureFormat) -> wgpu::TextureFormat {
    texture_format(c).expect("undefined texture format")
}

//
// Enums
//

pub fn texture_dimension(c: sb::WGPUTextureDimension) -> wgpu::TextureDimension {
    use sb::WGPUTextureDimension as D;
    match c {
        D::WGPUTextureDimension_1D => wgpu::TextureDimension::D1,
        D::WGPUTextureDimension_Undefined | D::WGPUTextureDimension_2D => {
            wgpu::TextureDimension::D2
        }
        D::WGPUTextureDimension_3D => wgpu::TextureDimension::D3,
        D::WGPUTextureDimension_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn texture_view_dimension(
    c: sb::WGPUTextureViewDimension,
) -> Option<wgpu::TextureViewDimension> {
    use sb::WGPUTextureViewDimension as D;
    use wgpu::TextureViewDimension as W;
    Some(match c {
        D::WGPUTextureViewDimension_Undefined => return None,
        D::WGPUTextureViewDimension_1D => W::D1,
        D::WGPUTextureViewDimension_2D => W::D2,
        D::WGPUTextureViewDimension_2DArray => W::D2Array,
        D::WGPUTextureViewDimension_Cube => W::Cube,
        D::WGPUTextureViewDimension_CubeArray => W::CubeArray,
        D::WGPUTextureViewDimension_3D => W::D3,
        D::WGPUTextureViewDimension_Force32 => panic!("unsupported {c:?}"),
    })
}

pub fn texture_aspect(c: sb::WGPUTextureAspect) -> wgpu::TextureAspect {
    use sb::WGPUTextureAspect as A;
    match c {
        A::WGPUTextureAspect_Undefined | A::WGPUTextureAspect_All => wgpu::TextureAspect::All,
        A::WGPUTextureAspect_StencilOnly => wgpu::TextureAspect::StencilOnly,
        A::WGPUTextureAspect_DepthOnly => wgpu::TextureAspect::DepthOnly,
        A::WGPUTextureAspect_Plane0Only
        | A::WGPUTextureAspect_Plane1Only
        | A::WGPUTextureAspect_Plane2Only
        | A::WGPUTextureAspect_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn address_mode(c: sb::WGPUAddressMode) -> wgpu::AddressMode {
    use sb::WGPUAddressMode as A;
    match c {
        A::WGPUAddressMode_Undefined | A::WGPUAddressMode_ClampToEdge => {
            wgpu::AddressMode::ClampToEdge
        }
        A::WGPUAddressMode_Repeat => wgpu::AddressMode::Repeat,
        A::WGPUAddressMode_MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        A::WGPUAddressMode_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn filter_mode(c: sb::WGPUFilterMode) -> wgpu::FilterMode {
    use sb::WGPUFilterMode as F;
    match c {
        F::WGPUFilterMode_Undefined | F::WGPUFilterMode_Nearest => wgpu::FilterMode::Nearest,
        F::WGPUFilterMode_Linear => wgpu::FilterMode::Linear,
        F::WGPUFilterMode_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn mipmap_filter_mode(c: sb::WGPUMipmapFilterMode) -> wgpu::MipmapFilterMode {
    use sb::WGPUMipmapFilterMode as F;
    match c {
        F::WGPUMipmapFilterMode_Undefined | F::WGPUMipmapFilterMode_Nearest => {
            wgpu::MipmapFilterMode::Nearest
        }
        F::WGPUMipmapFilterMode_Linear => wgpu::MipmapFilterMode::Linear,
        F::WGPUMipmapFilterMode_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn compare_function(c: sb::WGPUCompareFunction) -> Option<wgpu::CompareFunction> {
    use sb::WGPUCompareFunction as C;
    use wgpu::CompareFunction as W;
    Some(match c {
        C::WGPUCompareFunction_Undefined => return None,
        C::WGPUCompareFunction_Never => W::Never,
        C::WGPUCompareFunction_Less => W::Less,
        C::WGPUCompareFunction_Equal => W::Equal,
        C::WGPUCompareFunction_LessEqual => W::LessEqual,
        C::WGPUCompareFunction_Greater => W::Greater,
        C::WGPUCompareFunction_NotEqual => W::NotEqual,
        C::WGPUCompareFunction_GreaterEqual => W::GreaterEqual,
        C::WGPUCompareFunction_Always => W::Always,
        C::WGPUCompareFunction_Force32 => panic!("unsupported {c:?}"),
    })
}

pub fn vertex_format(c: sb::WGPUVertexFormat) -> wgpu::VertexFormat {
    use sb::WGPUVertexFormat as C;
    use wgpu::VertexFormat as W;
    match c {
        C::WGPUVertexFormat_Uint8 => W::Uint8,
        C::WGPUVertexFormat_Uint8x2 => W::Uint8x2,
        C::WGPUVertexFormat_Uint8x4 => W::Uint8x4,
        C::WGPUVertexFormat_Sint8 => W::Sint8,
        C::WGPUVertexFormat_Sint8x2 => W::Sint8x2,
        C::WGPUVertexFormat_Sint8x4 => W::Sint8x4,
        C::WGPUVertexFormat_Unorm8 => W::Unorm8,
        C::WGPUVertexFormat_Unorm8x2 => W::Unorm8x2,
        C::WGPUVertexFormat_Unorm8x4 => W::Unorm8x4,
        C::WGPUVertexFormat_Snorm8 => W::Snorm8,
        C::WGPUVertexFormat_Snorm8x2 => W::Snorm8x2,
        C::WGPUVertexFormat_Snorm8x4 => W::Snorm8x4,
        C::WGPUVertexFormat_Uint16 => W::Uint16,
        C::WGPUVertexFormat_Uint16x2 => W::Uint16x2,
        C::WGPUVertexFormat_Uint16x4 => W::Uint16x4,
        C::WGPUVertexFormat_Sint16 => W::Sint16,
        C::WGPUVertexFormat_Sint16x2 => W::Sint16x2,
        C::WGPUVertexFormat_Sint16x4 => W::Sint16x4,
        C::WGPUVertexFormat_Unorm16 => W::Unorm16,
        C::WGPUVertexFormat_Unorm16x2 => W::Unorm16x2,
        C::WGPUVertexFormat_Unorm16x4 => W::Unorm16x4,
        C::WGPUVertexFormat_Snorm16 => W::Snorm16,
        C::WGPUVertexFormat_Snorm16x2 => W::Snorm16x2,
        C::WGPUVertexFormat_Snorm16x4 => W::Snorm16x4,
        C::WGPUVertexFormat_Float16 => W::Float16,
        C::WGPUVertexFormat_Float16x2 => W::Float16x2,
        C::WGPUVertexFormat_Float16x4 => W::Float16x4,
        C::WGPUVertexFormat_Float32 => W::Float32,
        C::WGPUVertexFormat_Float32x2 => W::Float32x2,
        C::WGPUVertexFormat_Float32x3 => W::Float32x3,
        C::WGPUVertexFormat_Float32x4 => W::Float32x4,
        C::WGPUVertexFormat_Uint32 => W::Uint32,
        C::WGPUVertexFormat_Uint32x2 => W::Uint32x2,
        C::WGPUVertexFormat_Uint32x3 => W::Uint32x3,
        C::WGPUVertexFormat_Uint32x4 => W::Uint32x4,
        C::WGPUVertexFormat_Sint32 => W::Sint32,
        C::WGPUVertexFormat_Sint32x2 => W::Sint32x2,
        C::WGPUVertexFormat_Sint32x3 => W::Sint32x3,
        C::WGPUVertexFormat_Sint32x4 => W::Sint32x4,
        C::WGPUVertexFormat_Unorm10_10_10_2 => W::Unorm10_10_10_2,
        C::WGPUVertexFormat_Unorm8x4BGRA => W::Unorm8x4Bgra,
        C::WGPUVertexFormat_Snorm10_10_10_2 | C::WGPUVertexFormat_Force32 => {
            panic!("unsupported {c:?}")
        }
    }
}

pub fn vertex_step_mode(c: sb::WGPUVertexStepMode) -> wgpu::VertexStepMode {
    use sb::WGPUVertexStepMode as S;
    match c {
        S::WGPUVertexStepMode_Undefined | S::WGPUVertexStepMode_Vertex => {
            wgpu::VertexStepMode::Vertex
        }
        S::WGPUVertexStepMode_Instance => wgpu::VertexStepMode::Instance,
        S::WGPUVertexStepMode_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn primitive_topology(c: sb::WGPUPrimitiveTopology) -> wgpu::PrimitiveTopology {
    use sb::WGPUPrimitiveTopology as T;
    use wgpu::PrimitiveTopology as W;
    match c {
        T::WGPUPrimitiveTopology_PointList => W::PointList,
        T::WGPUPrimitiveTopology_LineList => W::LineList,
        T::WGPUPrimitiveTopology_LineStrip => W::LineStrip,
        T::WGPUPrimitiveTopology_Undefined | T::WGPUPrimitiveTopology_TriangleList => {
            W::TriangleList
        }
        T::WGPUPrimitiveTopology_TriangleStrip => W::TriangleStrip,
        T::WGPUPrimitiveTopology_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn index_format(c: sb::WGPUIndexFormat) -> Option<wgpu::IndexFormat> {
    use sb::WGPUIndexFormat as I;
    match c {
        I::WGPUIndexFormat_Undefined => None,
        I::WGPUIndexFormat_Uint16 => Some(wgpu::IndexFormat::Uint16),
        I::WGPUIndexFormat_Uint32 => Some(wgpu::IndexFormat::Uint32),
        I::WGPUIndexFormat_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn front_face(c: sb::WGPUFrontFace) -> wgpu::FrontFace {
    use sb::WGPUFrontFace as F;
    match c {
        F::WGPUFrontFace_Undefined | F::WGPUFrontFace_CCW => wgpu::FrontFace::Ccw,
        F::WGPUFrontFace_CW => wgpu::FrontFace::Cw,
        F::WGPUFrontFace_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn cull_mode(c: sb::WGPUCullMode) -> Option<wgpu::Face> {
    use sb::WGPUCullMode as C;
    match c {
        C::WGPUCullMode_Undefined | C::WGPUCullMode_None => None,
        C::WGPUCullMode_Front => Some(wgpu::Face::Front),
        C::WGPUCullMode_Back => Some(wgpu::Face::Back),
        C::WGPUCullMode_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn stencil_operation(c: sb::WGPUStencilOperation) -> wgpu::StencilOperation {
    use sb::WGPUStencilOperation as S;
    use wgpu::StencilOperation as W;
    match c {
        S::WGPUStencilOperation_Undefined | S::WGPUStencilOperation_Keep => W::Keep,
        S::WGPUStencilOperation_Zero => W::Zero,
        S::WGPUStencilOperation_Replace => W::Replace,
        S::WGPUStencilOperation_Invert => W::Invert,
        S::WGPUStencilOperation_IncrementClamp => W::IncrementClamp,
        S::WGPUStencilOperation_DecrementClamp => W::DecrementClamp,
        S::WGPUStencilOperation_IncrementWrap => W::IncrementWrap,
        S::WGPUStencilOperation_DecrementWrap => W::DecrementWrap,
        S::WGPUStencilOperation_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn blend_operation(c: sb::WGPUBlendOperation) -> wgpu::BlendOperation {
    use sb::WGPUBlendOperation as O;
    use wgpu::BlendOperation as W;
    match c {
        O::WGPUBlendOperation_Undefined | O::WGPUBlendOperation_Add => W::Add,
        O::WGPUBlendOperation_Subtract => W::Subtract,
        O::WGPUBlendOperation_ReverseSubtract => W::ReverseSubtract,
        O::WGPUBlendOperation_Min => W::Min,
        O::WGPUBlendOperation_Max => W::Max,
        O::WGPUBlendOperation_Force32 => panic!("unsupported {c:?}"),
    }
}

/// Converts a blend factor. `Undefined` stands for `default`, which differs between the source
/// (`One`) and the destination (`Zero`) factor.
pub fn blend_factor(c: sb::WGPUBlendFactor, default: wgpu::BlendFactor) -> wgpu::BlendFactor {
    use sb::WGPUBlendFactor as F;
    use wgpu::BlendFactor as W;
    match c {
        F::WGPUBlendFactor_Undefined => default,
        F::WGPUBlendFactor_Zero => W::Zero,
        F::WGPUBlendFactor_One => W::One,
        F::WGPUBlendFactor_Src => W::Src,
        F::WGPUBlendFactor_OneMinusSrc => W::OneMinusSrc,
        F::WGPUBlendFactor_SrcAlpha => W::SrcAlpha,
        F::WGPUBlendFactor_OneMinusSrcAlpha => W::OneMinusSrcAlpha,
        F::WGPUBlendFactor_Dst => W::Dst,
        F::WGPUBlendFactor_OneMinusDst => W::OneMinusDst,
        F::WGPUBlendFactor_DstAlpha => W::DstAlpha,
        F::WGPUBlendFactor_OneMinusDstAlpha => W::OneMinusDstAlpha,
        F::WGPUBlendFactor_SrcAlphaSaturated => W::SrcAlphaSaturated,
        F::WGPUBlendFactor_Constant => W::Constant,
        F::WGPUBlendFactor_OneMinusConstant => W::OneMinusConstant,
        F::WGPUBlendFactor_Src1 => W::Src1,
        F::WGPUBlendFactor_OneMinusSrc1 => W::OneMinusSrc1,
        F::WGPUBlendFactor_Src1Alpha => W::Src1Alpha,
        F::WGPUBlendFactor_OneMinusSrc1Alpha => W::OneMinusSrc1Alpha,
        F::WGPUBlendFactor_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn query_type(c: sb::WGPUQueryType) -> wgpu::QueryType {
    use sb::WGPUQueryType as Q;
    match c {
        Q::WGPUQueryType_Occlusion => wgpu::QueryType::Occlusion,
        Q::WGPUQueryType_Timestamp => wgpu::QueryType::Timestamp,
        Q::WGPUQueryType_Force32 => panic!("unsupported {c:?}"),
    }
}

pub fn extent(c: &sb::WGPUExtent3D) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width: c.width,
        height: c.height,
        depth_or_array_layers: c.depthOrArrayLayers,
    }
}

pub fn color(c: &sb::WGPUColor) -> wgpu::Color {
    wgpu::Color {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    }
}

/// Reads a C array as a slice. A null pointer with a zero count is the empty slice.
///
/// # Safety
///
/// `ptr` must point to `count` initialized elements that live for `'a`.
pub unsafe fn slice<'a, T>(ptr: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, count) }
    }
}

/// Panics when a struct chains an extension that the shim does not read. The shim reports every
/// Dawn feature as unsupported, so Skia only chains one when that report is wrong.
///
/// # Safety
///
/// `chain` must be null or point to a valid chained struct.
pub unsafe fn no_chain(chain: *const sb::WGPUChainedStruct, owner: &str) {
    if let Some(chained) = unsafe { chain.as_ref() } {
        panic!(
            "{owner} chains the unsupported extension {:?}",
            chained.sType
        );
    }
}

fn flags<W: Copy + BitOr<Output = W>>(c: u64, table: &[(u64, W)], empty: W, name: &str) -> W {
    let mut known = 0;
    let mut result = empty;
    for &(c_flag, w_flag) in table {
        if c & c_flag == c_flag {
            result = result | w_flag;
            known |= c_flag;
        }
    }
    assert_eq!(c & !known, 0, "unsupported {name} bits");
    result
}

fn c_flags<W: Copy + BitAnd<Output = W> + PartialEq>(w: W, table: &[(u64, W)]) -> u64 {
    table
        .iter()
        .filter(|&&(_, w_flag)| w & w_flag == w_flag)
        .fold(0, |c, &(c_flag, _)| c | c_flag)
}
