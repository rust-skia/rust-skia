use std::mem::uninitialized;
use crate::prelude::*;
use rust_skia::{
    SkAlphaType,
    SkImageInfo,
    SkColorType,
    SkYUVColorSpace,
};
use crate::{
    skia::{
        ColorSpace,
        IRect,
        ISize,
        IPoint
    },
};

#[derive(Copy, Clone, PartialEq)]
pub struct AlphaType(pub(crate) SkAlphaType);

#[allow(non_upper_case_globals)]
impl AlphaType {
    pub const Unknown: AlphaType = AlphaType(SkAlphaType::kUnknown_SkAlphaType);
    pub const Opaque: AlphaType = AlphaType(SkAlphaType::kOpaque_SkAlphaType);
    pub const Premul: AlphaType = AlphaType(SkAlphaType::kPremul_SkAlphaType);
    pub const Unpremul: AlphaType = AlphaType(SkAlphaType::kUnpremul_SkAlphaType);

    pub fn is_opaque(self) -> bool {
        self == Self::Opaque
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct ColorType(pub(crate) SkColorType);

#[allow(non_upper_case_globals)]
impl ColorType {
    pub const Unknown: ColorType = ColorType(SkColorType::kUnknown_SkColorType);
    pub const Alpha8: ColorType = ColorType(SkColorType::kAlpha_8_SkColorType);
    pub const RGB565: ColorType = ColorType(SkColorType::kRGB_565_SkColorType);
    pub const ARGB4444: ColorType = ColorType(SkColorType::kARGB_4444_SkColorType);
    pub const RGBA8888: ColorType = ColorType(SkColorType::kRGBA_8888_SkColorType);
    pub const RGB888x: ColorType = ColorType(SkColorType::kRGB_888x_SkColorType);
    pub const BRGA8888: ColorType = ColorType(SkColorType::kBGRA_8888_SkColorType);
    pub const RGBA1010102: ColorType = ColorType(SkColorType::kRGBA_1010102_SkColorType);
    pub const RGB101010x: ColorType = ColorType(SkColorType::kRGB_101010x_SkColorType);
    pub const Gray8: ColorType = ColorType(SkColorType::kGray_8_SkColorType);
    pub const RGBAF16: ColorType = ColorType(SkColorType::kRGBA_F16_SkColorType);
    pub const RGBAF32: ColorType = ColorType(SkColorType::kRGBA_F32_SkColorType);
    pub const N32: ColorType = ColorType(SkColorType::kN32_SkColorType);

    pub fn bytes_per_pixel(self) -> usize {
        unsafe { rust_skia::SkColorTypeBytesPerPixel(self.0) as _ }
    }

    pub fn is_always_opaque(self) -> bool {
        unsafe { rust_skia::SkColorTypeIsAlwaysOpaque(self.0) }
    }

    pub fn validate_alpha_type(self, alpha_type: AlphaType) -> Option<AlphaType> {
        let mut alpha_type_r = AlphaType::Unknown;
        unsafe { rust_skia::SkColorTypeValidateAlphaType(self.0, alpha_type.0, &mut alpha_type_r.0) }
            .if_true_some(alpha_type_r)
    }
}

pub struct YUVColorSpace(pub(crate) SkYUVColorSpace);
#[allow(non_upper_case_globals)]

impl YUVColorSpace {
    pub const JPEG: YUVColorSpace = YUVColorSpace(SkYUVColorSpace::kJPEG_SkYUVColorSpace);
    pub const Rec601: YUVColorSpace = YUVColorSpace(SkYUVColorSpace::kRec601_SkYUVColorSpace);
    pub const Rec709: YUVColorSpace = YUVColorSpace(SkYUVColorSpace::kRec709_SkYUVColorSpace);
}

pub type ImageInfo = Handle<SkImageInfo>;

impl NativeDrop for SkImageInfo {
    fn drop(&mut self) {
        unsafe { rust_skia::C_SkImageInfo_Destruct(self) }
    }
}

impl NativeClone for SkImageInfo {
    fn clone(&self) -> Self {
        let mut image_info = unsafe { SkImageInfo::new() };
        unsafe { rust_skia::C_SkImageInfo_Copy(self, &mut image_info); }
        image_info
    }
}

impl ImageInfo {
    pub fn new_empty() -> ImageInfo {
        let mut image_info : SkImageInfo = unsafe { uninitialized() };
        unsafe { rust_skia::C_SkImageInfo_Construct(&mut image_info); }
        ImageInfo::from_native(image_info)
    }

    pub fn new(dimensions: ISize, ct: ColorType, at: AlphaType, cs: Option<ColorSpace>) -> ImageInfo {
        let mut image_info = Self::new_empty();

        unsafe {
            rust_skia::C_SkImageInfo_Make(image_info.native_mut(), dimensions.width, dimensions.height, ct.0, at.0, cs.shared_ptr())
        }
        image_info
    }

    pub fn new_n32(dimensions: ISize, at: AlphaType, cs: Option<ColorSpace>) -> ImageInfo {
        Self::new(dimensions, ColorType::N32, at, cs)
    }

    pub fn new_s32(dimensions: ISize, at: AlphaType) -> ImageInfo {
        let mut image_info = Self::new_empty();
        unsafe { rust_skia::C_SkImageInfo_MakeS32(image_info.native_mut(), dimensions.width, dimensions.height, at.0); }
        image_info
    }

    pub fn new_n32_premul(dimensions: ISize, cs: Option<ColorSpace>) -> ImageInfo {
        Self::new(dimensions, ColorType::N32, AlphaType::Premul, cs)
    }

    pub fn new_a8(dimensions: ISize) -> ImageInfo {
        Self::new(dimensions, ColorType::Alpha8, AlphaType::Premul, None)
    }

    pub fn new_unknown(dimensions: Option<ISize>) -> ImageInfo {
        Self::new(
            dimensions.unwrap_or(ISize::new(0, 0)),
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
        AlphaType(self.native().fAlphaType)
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkImageInfo_colorSpace(self.native()) })
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

    pub fn with_dimensions(&self, new_dimensions: ISize) -> ImageInfo {
        ImageInfo::new(new_dimensions, self.color_type(), self.alpha_type(), self.color_space())
    }

    pub fn with_alpha_type(&self, new_alpha_type: AlphaType) -> ImageInfo {
        ImageInfo::new(self.dimensions(), self.color_type(), new_alpha_type, self.color_space())
    }

    pub fn with_color_type(&self, new_color_type: ColorType) -> ImageInfo {
        ImageInfo::new(self.dimensions(), new_color_type, self.alpha_type(), self.color_space())
    }

    pub fn with_color_space(&self, new_color_space: Option<ColorSpace>) -> ImageInfo {
        ImageInfo::new(self.dimensions(), self.color_type(), self.alpha_type(), new_color_space)
    }

    pub fn bytes_per_pixel(&self) -> usize {
        unsafe { self.native().bytesPerPixel() as _ }
    }

    pub fn shift_per_pixel(&self) -> usize {
        unsafe { self.native().shiftPerPixel() as _ }
    }

    pub fn min_row_bytes(&self) -> usize {
        unsafe { self.native().minRowBytes() }
    }

    pub fn compute_offset(&self, point: IPoint, row_bytes: usize) -> usize {
        unsafe { self.native().computeOffset(point.x, point.y, row_bytes) }
    }

    pub fn compute_byte_size(&self, row_bytes: usize) -> usize {
        unsafe { self.native().computeByteSize(row_bytes) }
    }

    pub fn valid_row_bytes(&self, row_bytes: usize) -> bool {
        unsafe { self.native().validRowBytes(row_bytes) }
    }
}

#[test]
fn ref_cnt_in_relation_to_color_space() {
    let cs = ColorSpace::new_srgb();
    let before = cs.ref_cnt();
    {
        let ii = ImageInfo::new_n32(ISize::new(10, 10), AlphaType::Premul, Some(cs.clone()));
        let cs = ii.color_space();
        // one for clone, one for the reference in image info.
        assert_eq!(before+2, cs.unwrap().ref_cnt())
    }
    assert_eq!(before, cs.ref_cnt())
}