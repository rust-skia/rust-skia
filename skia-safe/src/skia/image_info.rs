use std::mem;
use crate::prelude::*;
use skia_bindings::{
    SkAlphaType,
    SkImageInfo,
    SkColorType,
    SkYUVColorSpace,
    C_SkImageInfo_Construct
};
use crate::skia::{
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

#[derive(Copy, Clone, PartialEq)]
pub struct ColorType(pub(crate) SkColorType);

#[allow(non_upper_case_globals)]
impl ColorType {
    pub const Unknown: Self = Self(SkColorType::kUnknown_SkColorType);
    pub const Alpha8: Self = Self(SkColorType::kAlpha_8_SkColorType);
    pub const RGB565: Self = Self(SkColorType::kRGB_565_SkColorType);
    pub const ARGB4444: Self = Self(SkColorType::kARGB_4444_SkColorType);
    pub const RGBA8888: Self = Self(SkColorType::kRGBA_8888_SkColorType);
    pub const RGB888x: Self = Self(SkColorType::kRGB_888x_SkColorType);
    pub const BRGA8888: Self = Self(SkColorType::kBGRA_8888_SkColorType);
    pub const RGBA1010102: Self = Self(SkColorType::kRGBA_1010102_SkColorType);
    pub const RGB101010x: Self = Self(SkColorType::kRGB_101010x_SkColorType);
    pub const Gray8: Self = Self(SkColorType::kGray_8_SkColorType);
    pub const RGBAF16: Self = Self(SkColorType::kRGBA_F16_SkColorType);
    pub const RGBAF32: Self = Self(SkColorType::kRGBA_F32_SkColorType);
    pub const N32: Self = Self(SkColorType::kN32_SkColorType);

    pub fn bytes_per_pixel(self) -> usize {
        unsafe { skia_bindings::SkColorTypeBytesPerPixel(self.0) as _ }
    }

    pub fn is_always_opaque(self) -> bool {
        unsafe { skia_bindings::SkColorTypeIsAlwaysOpaque(self.0) }
    }

    pub fn validate_alpha_type(self, alpha_type: AlphaType) -> Option<AlphaType> {
        let mut alpha_type_r = AlphaType::Unknown;
        unsafe { skia_bindings::SkColorTypeValidateAlphaType(self.0, alpha_type.into_native(), alpha_type_r.native_mut()) }
            .if_true_some(alpha_type_r)
    }
}

pub struct YUVColorSpace(pub(crate) SkYUVColorSpace);
#[allow(non_upper_case_globals)]

impl YUVColorSpace {
    pub const JPEG: Self = Self(SkYUVColorSpace::kJPEG_SkYUVColorSpace);
    pub const Rec601: Self = Self(SkYUVColorSpace::kRec601_SkYUVColorSpace);
    pub const Rec709: Self = Self(SkYUVColorSpace::kRec709_SkYUVColorSpace);
}

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
            skia_bindings::C_SkImageInfo_Make(image_info.native_mut(), dimensions.width, dimensions.height, ct.0, at.into_native(), cs.shared_ptr())
        }
        image_info
    }

    pub fn new_n32<IS: Into<ISize>>(dimensions: IS, at: AlphaType, cs: Option<ColorSpace>) -> ImageInfo {
        Self::new(dimensions, ColorType::N32, at, cs)
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
        Self::new(dimensions, ColorType::N32, AlphaType::Premul, cs)
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
        ColorType(self.native().fColorType)
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

    pub fn gamma_close_to_srgb(&self) -> bool {
        self.color_space()
            .map(|cs| cs.gamma_close_to_srgb())
            .unwrap_or(false)
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