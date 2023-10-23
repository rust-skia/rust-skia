use crate::{prelude::*, AlphaType};
use sb::SkColorType;
use skia_bindings as sb;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum ColorType {
    /// uninitialized
    Unknown = SkColorType::kUnknown_SkColorType as _,
    /// pixel with alpha in 8-bit byte
    Alpha8 = SkColorType::kAlpha_8_SkColorType as _,
    /// pixel with 5 bits red, 6 bits green, 5 bits blue, in 16-bit word
    RGB565 = SkColorType::kRGB_565_SkColorType as _,
    /// pixel with 4 bits for alpha, red, green, blue; in 16-bit word
    ARGB4444 = SkColorType::kARGB_4444_SkColorType as _,
    /// pixel with 8 bits for red, green, blue, alpha; in 32-bit word
    RGBA8888 = SkColorType::kRGBA_8888_SkColorType as _,
    /// pixel with 8 bits each for red, green, blue; in 32-bit word
    RGB888x = SkColorType::kRGB_888x_SkColorType as _,
    /// pixel with 8 bits for blue, green, red, alpha; in 32-bit word
    BGRA8888 = SkColorType::kBGRA_8888_SkColorType as _,
    /// 10 bits for red, green, blue; 2 bits for alpha; in 32-bit word
    RGBA1010102 = SkColorType::kRGBA_1010102_SkColorType as _,
    /// 10 bits for blue, green, red; 2 bits for alpha; in 32-bit word
    BGRA1010102 = SkColorType::kBGRA_1010102_SkColorType as _,
    /// pixel with 10 bits each for red, green, blue; in 32-bit word
    RGB101010x = SkColorType::kRGB_101010x_SkColorType as _,
    /// pixel with 10 bits each for blue, green, red; in 32-bit word
    BGR101010x = SkColorType::kBGR_101010x_SkColorType as _,
    /// pixel with 10 bits each for blue, green, red; in 32-bit word, extended range
    BGR101010xXR = SkColorType::kBGR_101010x_XR_SkColorType as _,
    /// pixel with 10 used bits (most significant) followed by 6 unused
    /// bits for red, green, blue, alpha; in 64-bit word
    RGBA10x6 = SkColorType::kRGBA_10x6_SkColorType as _,
    /// pixel with grayscale level in 8-bit byte
    Gray8 = SkColorType::kGray_8_SkColorType as _,
    /// pixel with half floats in `[0,1]` for red, green, blue, alpha in 64-bit word
    RGBAF16Norm = SkColorType::kRGBA_F16Norm_SkColorType as _,
    /// pixel with half floats for red, green, blue, alpha in 64-bit word
    RGBAF16 = SkColorType::kRGBA_F16_SkColorType as _,
    /// pixel using C float for red, green, blue, alpha; in 128-bit word
    RGBAF32 = SkColorType::kRGBA_F32_SkColorType as _,

    // The following 6 color types are just for reading from - not for rendering to
    /// pixel with a uint8_t for red and green
    R8G8UNorm = SkColorType::kR8G8_unorm_SkColorType as _,

    /// pixel with a half float for alpha
    A16Float = SkColorType::kA16_float_SkColorType as _,
    /// pixel with a half float for red and green
    R16G16Float = SkColorType::kR16G16_float_SkColorType as _,

    ///pixel with a little endian uint16_t for alpha
    A16UNorm = SkColorType::kA16_unorm_SkColorType as _,
    ///pixel with a little endian uint16_t for red and green
    R16G16UNorm = SkColorType::kR16G16_unorm_SkColorType as _,
    ///pixel with a little endian uint16_t for red, green, blue and alpha
    R16G16B16A16UNorm = SkColorType::kR16G16B16A16_unorm_SkColorType as _,

    SRGBA8888 = SkColorType::kSRGBA_8888_SkColorType as _,
    R8UNorm = SkColorType::kR8_unorm_SkColorType as _,
}

native_transmutable!(SkColorType, ColorType, color_type_layout);

impl ColorType {
    #[deprecated(since = "0.51.0", note = "Use ColorType::N32 ")]
    pub const fn n32() -> Self {
        Self::N32
    }

    pub const N32: Self = unsafe { *((&SkColorType::kN32_SkColorType) as *const _ as *const _) };

    pub const COUNT: usize =
        unsafe { *((&SkColorType::kLastEnum_SkColorType) as *const _ as *const _) } as usize
            + 1usize;

    pub fn bytes_per_pixel(self) -> usize {
        unsafe {
            sb::SkColorTypeBytesPerPixel(self.into_native())
                .try_into()
                .unwrap()
        }
    }

    pub fn is_always_opaque(self) -> bool {
        unsafe { sb::SkColorTypeIsAlwaysOpaque(self.into_native()) }
    }

    pub fn validate_alpha_type(self, alpha_type: AlphaType) -> Option<AlphaType> {
        let mut alpha_type_r = AlphaType::Unknown;
        unsafe {
            sb::SkColorTypeValidateAlphaType(self.into_native(), alpha_type, &mut alpha_type_r)
        }
        .if_true_some(alpha_type_r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n32_matches() {
        assert_eq!(
            ColorType::from_native_c(skia_bindings::SkColorType::kN32_SkColorType),
            ColorType::N32
        );
    }
}
