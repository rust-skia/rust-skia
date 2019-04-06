use std::mem;
use crate::prelude::*;
use skia_bindings::{
    SkAlphaType,
    SkImageInfo,
    SkColorType,
    SkYUVColorSpace,
    C_SkImageInfo_Construct
};
use crate::core::{
    ColorSpace,
    IRect,
    ISize,
    IPoint
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum AlphaType {
    Unknown = SkAlphaType::kUnknown_SkAlphaType as _,
    Opaque = SkAlphaType::kOpaque_SkAlphaType as _,
    Premul = SkAlphaType::kPremul_SkAlphaType as _,
    Unpremul = SkAlphaType::kUnpremul_SkAlphaType as _
}

impl NativeTransmutable<SkAlphaType> for AlphaType {}
#[test] fn test_alpha_type_layout() { AlphaType::test_layout() }

impl AlphaType {
    pub fn is_opaque(self) -> bool {
        self == AlphaType::Opaque
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ColorType {
    Unknown = SkColorType::kUnknown_SkColorType as _,
    Alpha8 = SkColorType::kAlpha_8_SkColorType as _,
    RGB565 = SkColorType::kRGB_565_SkColorType as _,
    ARGB4444 = SkColorType::kARGB_4444_SkColorType as _,
    RGBA8888 = SkColorType::kRGBA_8888_SkColorType as _,
    RGB888x = SkColorType::kRGB_888x_SkColorType as _,
    BRGA8888 = SkColorType::kBGRA_8888_SkColorType as _,
    RGBA1010102 = SkColorType::kRGBA_1010102_SkColorType as _,
    RGB101010x = SkColorType::kRGB_101010x_SkColorType as _,
    Gray8 = SkColorType::kGray_8_SkColorType as _,
    F16Norm = SkColorType::kRGBA_F16Norm_SkColorType as _,
    RGBAF16 = SkColorType::kRGBA_F16_SkColorType as _,
    RGBAF32 = SkColorType::kRGBA_F32_SkColorType as _,
}

impl NativeTransmutable<SkColorType> for ColorType {}
#[test] fn test_color_type_layout() { ColorType::test_layout() }

impl ColorType {

    // error[E0658]: dereferencing raw pointers in constants is unstable (see issue #51911)
    /*
    pub const N32 : Self = unsafe {
        *((&SkColorType::kN32_SkColorType) as *const _ as *const _)
    };
    */

    pub fn n32() -> Self {
        Self::from_native(SkColorType::kN32_SkColorType)
    }

    pub fn bytes_per_pixel(self) -> usize {
        unsafe {
            skia_bindings::SkColorTypeBytesPerPixel(self.into_native()).try_into().unwrap()
        }
    }

    pub fn is_always_opaque(self) -> bool {
        unsafe {
            skia_bindings::SkColorTypeIsAlwaysOpaque(self.into_native())
        }
    }

    pub fn validate_alpha_type(self, alpha_type: AlphaType) -> Option<AlphaType> {
        let mut alpha_type_r = AlphaType::Unknown;
        unsafe { skia_bindings::SkColorTypeValidateAlphaType(self.into_native(), alpha_type.into_native(), alpha_type_r.native_mut()) }
            .if_true_some(alpha_type_r)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum YUVColorSpace {
    JPEG = SkYUVColorSpace::kJPEG_SkYUVColorSpace as _,
    Rec601 = SkYUVColorSpace::kRec601_SkYUVColorSpace as _,
    Rec709 = SkYUVColorSpace::kRec709_SkYUVColorSpace as _
}

impl NativeTransmutable<SkYUVColorSpace> for YUVColorSpace {}
#[test] fn test_yuv_color_space_layout() { YUVColorSpace::test_layout() }

pub type ImageInfo = Handle<SkImageInfo>;

impl NativeDrop for SkImageInfo {
    fn drop(&mut self) {
        unsafe {
            skia_bindings::C_SkImageInfo_destruct(self)
        }
    }
}

impl NativeClone for SkImageInfo {
    fn clone(&self) -> Self {
        let mut image_info = unsafe { SkImageInfo::new() };
        unsafe {
            skia_bindings::C_SkImageInfo_Copy(self, &mut image_info);
        }
        image_info
    }
}

// TODO: NativePartialEq

impl Default for Handle<SkImageInfo> {
    fn default() -> Self {
        ImageInfo::from_native(unsafe {
            let mut image_info : SkImageInfo = mem::uninitialized();
            // note SkImageInfo::new() does not link under Linux.
            C_SkImageInfo_Construct(&mut image_info);
            image_info
        })
    }
}

impl Handle<SkImageInfo> {

    pub fn new<IS: Into<ISize>>(dimensions: IS, ct: ColorType, at: AlphaType, cs: Option<ColorSpace>) -> Self {
        let dimensions = dimensions.into();
        let mut image_info = Self::default();

        unsafe {
            skia_bindings::C_SkImageInfo_Make(
                image_info.native_mut(),
                dimensions.width, dimensions.height,
                ct.into_native(), at.into_native(), cs.shared_ptr())
        }
        image_info
    }

    pub fn new_n32<IS: Into<ISize>>(dimensions: IS, at: AlphaType, cs: Option<ColorSpace>) -> ImageInfo {
        Self::new(dimensions, ColorType::n32(), at, cs)
    }

    pub fn new_s32<IS: Into<ISize>>(dimensions: IS, at: AlphaType) -> ImageInfo {
        let dimensions = dimensions.into();
        let mut image_info = Self::default();
        unsafe {
            skia_bindings::C_SkImageInfo_MakeS32(image_info.native_mut(), dimensions.width, dimensions.height, at.into_native());
        }
        image_info
    }

    pub fn new_n32_premul<IS: Into<ISize>>(dimensions: IS, cs: Option<ColorSpace>) -> ImageInfo {
        Self::new(dimensions, ColorType::n32(), AlphaType::Premul, cs)
    }

    pub fn new_a8<IS: Into<ISize>>(dimensions: IS) -> ImageInfo {
        Self::new(dimensions, ColorType::Alpha8, AlphaType::Premul, None)
    }

    pub fn new_unknown(dimensions: Option<ISize>) -> ImageInfo {
        Self::new(
            dimensions.unwrap_or_default(),
            ColorType::Unknown,
            AlphaType::Unknown,
            None)
    }

    pub fn width(&self) -> i32 {
        self.dimensions().width
    }

    pub fn height(&self) -> i32 {
        self.dimensions().height
    }

    pub fn color_type(&self) -> ColorType {
        ColorType::from_native(self.native().fColorType)
    }

    pub fn alpha_type(&self) -> AlphaType {
        AlphaType::from_native(self.native().fAlphaType)
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkImageInfo_colorSpace(self.native())
        })
    }

    pub fn is_empty(&self) -> bool {
        self.dimensions().is_empty()
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha_type().is_opaque()
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native(self.native().fDimensions)
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_size(self.dimensions())
    }

    pub fn with_dimensions<IS: Into<ISize>>(&self, new_dimensions: IS) -> Self {
        Self::new(new_dimensions, self.color_type(), self.alpha_type(), self.color_space())
    }

    pub fn with_alpha_type(&self, new_alpha_type: AlphaType) -> Self {
        Self::new(self.dimensions(), self.color_type(), new_alpha_type, self.color_space())
    }

    pub fn with_color_type(&self, new_color_type: ColorType) -> Self {
        Self::new(self.dimensions(), new_color_type, self.alpha_type(), self.color_space())
    }

    pub fn with_color_space(&self, new_color_space: Option<ColorSpace>) -> Self {
        Self::new(self.dimensions(), self.color_type(), self.alpha_type(), new_color_space)
    }

    pub fn bytes_per_pixel(&self) -> usize {
        unsafe {
            self.native().bytesPerPixel().try_into().unwrap()
        }
    }

    pub fn shift_per_pixel(&self) -> usize {
        unsafe {
            self.native().shiftPerPixel().try_into().unwrap()
        }
    }

    pub fn min_row_bytes(&self) -> usize {
        unsafe {
            self.native().minRowBytes()
        }
    }

    pub fn compute_offset<IP: Into<IPoint>>(&self, point: IP, row_bytes: usize) -> usize {
        let point = point.into();
        unsafe {
            self.native().computeOffset(point.x, point.y, row_bytes)
        }
    }

    pub fn compute_byte_size(&self, row_bytes: usize) -> usize {
        unsafe {
            self.native().computeByteSize(row_bytes)
        }
    }

    pub fn valid_row_bytes(&self, row_bytes: usize) -> bool {
        unsafe {
            self.native().validRowBytes(row_bytes)
        }
    }

    /* TODO: does not link, create a C wrapper function for that.
    pub fn reset(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().reset()
        }
        self
    } */
}

#[test]
fn ref_cnt_in_relation_to_color_space() {
    let cs = ColorSpace::new_srgb();
    let before = cs.ref_cnt();
    {
        let ii = ImageInfo::new_n32((10, 10), AlphaType::Premul, Some(cs.clone()));
        let cs = ii.color_space();
        // one for clone, one for the reference in image info.
        assert_eq!(before+2, cs.unwrap().ref_cnt())
    }
    assert_eq!(before, cs.ref_cnt())
}