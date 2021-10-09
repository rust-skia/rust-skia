use crate::{prelude::*, ColorSpace, IPoint, IRect, ISize};
use skia_bindings::{self as sb, SkColorInfo, SkColorType, SkImageInfo};
use std::{fmt, mem};

pub use skia_bindings::SkAlphaType as AlphaType;
variant_name!(AlphaType::Premul, alpha_type_naming);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum ColorType {
    Unknown = SkColorType::kUnknown_SkColorType as _,
    Alpha8 = SkColorType::kAlpha_8_SkColorType as _,
    RGB565 = SkColorType::kRGB_565_SkColorType as _,
    ARGB4444 = SkColorType::kARGB_4444_SkColorType as _,
    RGBA8888 = SkColorType::kRGBA_8888_SkColorType as _,
    RGB888x = SkColorType::kRGB_888x_SkColorType as _,
    BGRA8888 = SkColorType::kBGRA_8888_SkColorType as _,
    RGBA1010102 = SkColorType::kRGBA_1010102_SkColorType as _,
    BGRA1010102 = SkColorType::kBGRA_1010102_SkColorType as _,
    RGB101010x = SkColorType::kRGB_101010x_SkColorType as _,
    BGR101010x = SkColorType::kBGR_101010x_SkColorType as _,
    Gray8 = SkColorType::kGray_8_SkColorType as _,
    RGBAF16Norm = SkColorType::kRGBA_F16Norm_SkColorType as _,
    RGBAF16 = SkColorType::kRGBA_F16_SkColorType as _,
    RGBAF32 = SkColorType::kRGBA_F32_SkColorType as _,
    R8G8UNorm = SkColorType::kR8G8_unorm_SkColorType as _,
    A16Float = SkColorType::kA16_float_SkColorType as _,
    R16G16Float = SkColorType::kR16G16_float_SkColorType as _,
    A16UNorm = SkColorType::kA16_unorm_SkColorType as _,
    R16G16UNorm = SkColorType::kR16G16_unorm_SkColorType as _,
    R16G16B16A16UNorm = SkColorType::kR16G16B16A16_unorm_SkColorType as _,
    SRGBA8888 = SkColorType::kSRGBA_8888_SkColorType as _,
}

native_transmutable!(SkColorType, ColorType, color_type_layout);

impl ColorType {
    // error[E0658]: dereferencing raw pointers in constants is unstable (see issue #51911)
    /*
    pub const N32 : Self = unsafe {
        *((&SkColorType::kN32_SkColorType) as *const _ as *const _)
    };
    */

    pub fn n32() -> Self {
        Self::from_native_c(SkColorType::kN32_SkColorType)
    }

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

pub use skia_bindings::SkYUVColorSpace as YUVColorSpace;
variant_name!(YUVColorSpace::JPEG, yuv_color_space_naming);

pub type ColorInfo = Handle<SkColorInfo>;
unsafe_send_sync!(ColorInfo);

impl NativeDrop for SkColorInfo {
    fn drop(&mut self) {
        unsafe { sb::C_SkColorInfo_destruct(self) }
    }
}

impl NativeClone for SkColorInfo {
    fn clone(&self) -> Self {
        unsafe {
            construct(|color_info| {
                sb::C_SkColorInfo_Construct(color_info);
                sb::C_SkColorInfo_Copy(self, color_info);
            })
        }
    }
}

impl NativePartialEq for SkColorInfo {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkColorInfo_Equals(self, rhs) }
    }
}

impl Default for ColorInfo {
    fn default() -> Self {
        Self::construct(|color_info| unsafe { sb::C_SkColorInfo_Construct(color_info) })
    }
}

impl fmt::Debug for ColorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColorInfo")
            .field("color_space", &self.color_space())
            .field("color_type", &self.color_type())
            .field("alpha_type", &self.alpha_type())
            .field("is_opaque", &self.is_opaque())
            .field("is_gamma_close_to_srgb", &self.is_gamma_close_to_srgb())
            .field("bytes_per_pixel", &self.bytes_per_pixel())
            .field("shift_per_pixel", &self.shift_per_pixel())
            .finish()
    }
}

impl ColorInfo {
    pub fn new(ct: ColorType, at: AlphaType, cs: impl Into<Option<ColorSpace>>) -> Self {
        Self::construct(|color_info| unsafe {
            sb::C_SkColorInfo_Construct2(
                color_info,
                ct.into_native(),
                at,
                cs.into().into_ptr_or_null(),
            )
        })
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(self.native().fColorSpace.fPtr)
    }

    pub fn color_type(&self) -> ColorType {
        ColorType::from_native_c(self.native().fColorType)
    }

    pub fn alpha_type(&self) -> AlphaType {
        self.native().fAlphaType
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha_type().is_opaque() || self.color_type().is_always_opaque()
    }

    pub fn is_gamma_close_to_srgb(&self) -> bool {
        unsafe { sb::C_SkColorInfo_gammaCloseToSRGB(self.native()) }
    }

    pub fn with_alpha_type(&self, new_alpha_type: AlphaType) -> Self {
        Self::new(self.color_type(), new_alpha_type, self.color_space())
    }

    pub fn with_color_type(&self, new_color_type: ColorType) -> Self {
        Self::new(new_color_type, self.alpha_type(), self.color_space())
    }

    pub fn with_color_space(&self, cs: impl Into<Option<ColorSpace>>) -> Self {
        Self::new(self.color_type(), self.alpha_type(), cs)
    }

    pub fn bytes_per_pixel(&self) -> usize {
        unsafe { self.native().bytesPerPixel().try_into().unwrap() }
    }

    pub fn shift_per_pixel(&self) -> usize {
        unsafe { self.native().shiftPerPixel().try_into().unwrap() }
    }
}

pub type ImageInfo = Handle<SkImageInfo>;
unsafe_send_sync!(ImageInfo);

impl NativeDrop for SkImageInfo {
    fn drop(&mut self) {
        unsafe { sb::C_SkImageInfo_destruct(self) }
    }
}

impl NativeClone for SkImageInfo {
    fn clone(&self) -> Self {
        unsafe {
            construct(|image_info| {
                sb::C_SkImageInfo_Construct(image_info);
                sb::C_SkImageInfo_Copy(self, image_info);
            })
        }
    }
}

impl NativePartialEq for SkImageInfo {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkImageInfo_Equals(self, rhs) }
    }
}

impl Default for Handle<SkImageInfo> {
    fn default() -> Self {
        Self::construct(|image_info| unsafe { sb::C_SkImageInfo_Construct(image_info) })
    }
}

impl fmt::Debug for ImageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageInfo")
            .field("color_info", self.color_info())
            .field("dimensions", &self.dimensions())
            .finish()
    }
}

impl ImageInfo {
    pub fn new(
        dimensions: impl Into<ISize>,
        ct: ColorType,
        at: AlphaType,
        cs: impl Into<Option<ColorSpace>>,
    ) -> Self {
        let dimensions = dimensions.into();
        let mut image_info = Self::default();

        unsafe {
            sb::C_SkImageInfo_Make(
                image_info.native_mut(),
                dimensions.width,
                dimensions.height,
                ct.into_native(),
                at,
                cs.into().into_ptr_or_null(),
            )
        }
        image_info
    }

    pub fn from_color_info(dimensions: impl Into<ISize>, color_info: ColorInfo) -> Self {
        // TODO: (perf) actually move of color_info.
        Self::new(
            dimensions,
            color_info.color_type(),
            color_info.alpha_type(),
            color_info.color_space(),
        )
    }

    pub fn new_n32(
        dimensions: impl Into<ISize>,
        at: AlphaType,
        cs: impl Into<Option<ColorSpace>>,
    ) -> ImageInfo {
        Self::new(dimensions, ColorType::n32(), at, cs)
    }

    pub fn new_s32(dimensions: impl Into<ISize>, at: AlphaType) -> ImageInfo {
        let dimensions = dimensions.into();
        let mut image_info = Self::default();
        unsafe {
            sb::C_SkImageInfo_MakeS32(
                image_info.native_mut(),
                dimensions.width,
                dimensions.height,
                at,
            );
        }
        image_info
    }

    pub fn new_n32_premul(
        dimensions: impl Into<ISize>,
        cs: impl Into<Option<ColorSpace>>,
    ) -> ImageInfo {
        Self::new(dimensions, ColorType::n32(), AlphaType::Premul, cs)
    }

    pub fn new_a8(dimensions: impl Into<ISize>) -> ImageInfo {
        Self::new(dimensions, ColorType::Alpha8, AlphaType::Premul, None)
    }

    pub fn new_unknown(dimensions: Option<ISize>) -> ImageInfo {
        Self::new(
            dimensions.unwrap_or_default(),
            ColorType::Unknown,
            AlphaType::Unknown,
            None,
        )
    }

    pub fn width(&self) -> i32 {
        self.dimensions().width
    }

    pub fn height(&self) -> i32 {
        self.dimensions().height
    }

    pub fn color_type(&self) -> ColorType {
        self.color_info().color_type()
    }

    pub fn alpha_type(&self) -> AlphaType {
        self.color_info().alpha_type()
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.color_info().color_space()
    }

    pub fn is_empty(&self) -> bool {
        self.dimensions().is_empty()
    }

    pub fn color_info(&self) -> &ColorInfo {
        Handle::from_native_ref(&self.native().fColorInfo)
    }

    pub fn is_opaque(&self) -> bool {
        self.color_info().is_opaque()
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native_c(self.native().fDimensions)
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_size(self.dimensions())
    }

    pub fn is_gamma_close_to_srgb(&self) -> bool {
        self.color_info().is_gamma_close_to_srgb()
    }

    pub fn with_dimensions(&self, new_dimensions: impl Into<ISize>) -> Self {
        Self::from_color_info(new_dimensions, self.color_info().clone())
    }

    pub fn with_alpha_type(&self, new_alpha_type: AlphaType) -> Self {
        Self::from_color_info(
            self.dimensions(),
            self.color_info().with_alpha_type(new_alpha_type),
        )
    }

    pub fn with_color_type(&self, new_color_type: ColorType) -> Self {
        Self::from_color_info(
            self.dimensions(),
            self.color_info().with_color_type(new_color_type),
        )
    }

    pub fn with_color_space(&self, new_color_space: impl Into<Option<ColorSpace>>) -> Self {
        Self::from_color_info(
            self.dimensions(),
            self.color_info().with_color_space(new_color_space),
        )
    }

    pub fn bytes_per_pixel(&self) -> usize {
        self.color_info().bytes_per_pixel()
    }

    pub fn shift_per_pixel(&self) -> usize {
        self.color_info().shift_per_pixel()
    }

    pub fn min_row_bytes(&self) -> usize {
        usize::try_from(self.width()).unwrap() * self.bytes_per_pixel()
    }

    pub fn compute_offset(&self, point: impl Into<IPoint>, row_bytes: usize) -> usize {
        let point = point.into();
        unsafe { self.native().computeOffset(point.x, point.y, row_bytes) }
    }

    pub fn compute_byte_size(&self, row_bytes: usize) -> usize {
        unsafe { self.native().computeByteSize(row_bytes) }
    }

    pub fn compute_min_byte_size(&self) -> usize {
        self.compute_byte_size(self.min_row_bytes())
    }

    pub fn valid_row_bytes(&self, row_bytes: usize) -> bool {
        if row_bytes < self.min_row_bytes() {
            return false;
        }
        let shift = self.shift_per_pixel();
        let aligned_row_bytes = row_bytes >> shift << shift;
        aligned_row_bytes == row_bytes
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { sb::C_SkImageInfo_reset(self.native_mut()) };
        self
    }

    /// Returns `true` if the `row_bytes` are valid for [ImageInfo] _and_ an image would fit into
    /// `pixels`.
    pub(crate) fn valid_pixels<P>(&self, row_bytes: usize, pixels: &[P]) -> bool {
        self.valid_row_bytes(row_bytes)
            && mem::size_of_val(pixels) >= self.compute_byte_size(row_bytes)
    }
}

#[cfg(test)]

mod tests {
    use crate::prelude::*;
    use crate::{AlphaType, ColorSpace, ImageInfo};
    use std::mem;

    #[test]
    fn ref_cnt_in_relation_to_color_space() {
        let cs = ColorSpace::new_srgb();
        let before = cs.native().ref_cnt();
        {
            let ii = ImageInfo::new_n32((10, 10), AlphaType::Premul, Some(cs.clone()));
            // one for the capture in image info
            assert_eq!(before + 1, cs.native().ref_cnt());
            let cs2 = ii.color_space();
            // and one for the returned one.
            assert_eq!(before + 2, cs.native().ref_cnt());
            drop(cs2);
        }
        assert_eq!(before, cs.native().ref_cnt())
    }

    #[test]
    fn size_of_val_actually_counts_slices_bytes() {
        let x: [u16; 4] = Default::default();
        assert_eq!(mem::size_of_val(&x), 8);
    }
}
