use std::{fmt, mem};

use skia_bindings::{self as sb, SkColorInfo, SkImageInfo};

use crate::prelude::*;
use crate::{AlphaType, ColorSpace, ColorType, IPoint, IRect, ISize};

pub use skia_bindings::SkYUVColorSpace as YUVColorSpace;
variant_name!(YUVColorSpace::JPEG);

/// Describes pixel and encoding. [`ImageInfo`] can be created from [`ColorInfo`] by providing
/// dimensions.
///
/// It encodes how pixel bits describe alpha, transparency; color components red, blue, and green;
/// and [`ColorSpace`], the range and linearity of colors.
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
    /// Creates [`ColorInfo`] from [`ColorType`], [`AlphaType`], and optionally [`ColorSpace`].
    ///
    /// If `cs` is `None` and [`ColorInfo`] is part of a drawing source, [`ColorSpace`] defaults to
    /// sRGB, mapping into the surface [`ColorSpace`]. Parameters are not validated to see if their
    /// values are legal, or that the combination is supported.
    ///
    /// - `ct` color type
    /// - `at` alpha type
    /// - `cs` optional color space
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

    /// Returns the color space.
    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(unsafe { self.native().colorSpace() })
    }

    /// Returns the color type.
    pub fn color_type(&self) -> ColorType {
        ColorType::from_native_c(self.native().fColorType)
    }

    /// Returns the alpha type.
    pub fn alpha_type(&self) -> AlphaType {
        self.native().fAlphaType
    }

    /// Returns true if the color info is opaque.
    pub fn is_opaque(&self) -> bool {
        self.alpha_type().is_opaque() || self.color_type().is_always_opaque()
    }

    /// Returns true if the gamma is close to sRGB.
    pub fn is_gamma_close_to_srgb(&self) -> bool {
        unsafe { self.native().gammaCloseToSRGB() }
    }

    /// Creates [`ColorInfo`] with the same color type and color space, with the alpha type set to
    /// `new_alpha_type`.
    ///
    /// The created [`ColorInfo`] contains `new_alpha_type` even if it is incompatible with the
    /// color type, in which case the alpha type in [`ColorInfo`] is ignored.
    ///
    /// - `new_alpha_type` new alpha type
    #[must_use]
    pub fn with_alpha_type(&self, new_alpha_type: AlphaType) -> Self {
        Self::construct(|ci| unsafe {
            sb::C_SkColorInfo_makeAlphaType(self.native(), new_alpha_type, ci)
        })
    }

    /// Creates [`ColorInfo`] with the same alpha type and color space, with the color type set to
    /// `new_color_type`.
    ///
    /// - `new_color_type` new color type
    #[must_use]
    pub fn with_color_type(&self, new_color_type: ColorType) -> Self {
        Self::construct(|ci| unsafe {
            sb::C_SkColorInfo_makeColorType(self.native(), new_color_type.into_native(), ci)
        })
    }

    /// Creates [`ColorInfo`] with the same alpha type and color type, with the color space set to
    /// `cs`. `cs` may be `None`.
    ///
    /// - `cs` optional color space
    #[must_use]
    pub fn with_color_space(&self, cs: impl Into<Option<ColorSpace>>) -> Self {
        let color_space: Option<ColorSpace> = cs.into();
        Self::construct(|ci| unsafe {
            sb::C_SkColorInfo_makeColorSpace(self.native(), color_space.into_ptr_or_null(), ci)
        })
    }

    /// Returns the number of bytes per pixel required by the color type. Returns zero if the color
    /// type is [`ColorType::Unknown`].
    pub fn bytes_per_pixel(&self) -> usize {
        unsafe { self.native().bytesPerPixel().try_into().unwrap() }
    }

    /// Returns the bit shift converting row bytes to row pixels. Returns zero for
    /// [`ColorType::Unknown`]. Returns one of: 0, 1, 2, 3, 4; left shift to convert pixels to bytes.
    pub fn shift_per_pixel(&self) -> usize {
        unsafe { self.native().shiftPerPixel().try_into().unwrap() }
    }
}

/// Describes pixel dimensions and encoding. [`crate::Bitmap`], [`crate::Image`], [`crate::Pixmap`],
/// and [`crate::Surface`] can be created from [`ImageInfo`]. [`ImageInfo`] can be retrieved from
/// [`crate::Bitmap`] and [`crate::Pixmap`], but not from [`crate::Image`] and [`crate::Surface`].
/// For example, [`crate::Image`] and [`crate::Surface`] implementations may defer pixel depth, so
/// may not completely specify [`ImageInfo`].
///
/// [`ImageInfo`] contains dimensions, the pixel integral width and height. It encodes how pixel
/// bits describe alpha, transparency; color components red, blue, and green; and [`ColorSpace`],
/// the range and linearity of colors.
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
    /// Creates [`ImageInfo`] from integral dimensions, [`ColorType`], [`AlphaType`], and
    /// optionally [`ColorSpace`].
    ///
    /// If `cs` is `None` and [`ImageInfo`] is part of a drawing source, [`ColorSpace`] defaults to
    /// sRGB, mapping into the surface [`ColorSpace`]. Parameters are not validated to see if their
    /// values are legal, or that the combination is supported.
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    /// - `ct` color type
    /// - `at` alpha type
    /// - `cs` optional color space
    pub fn new(
        dimensions: impl Into<ISize>,
        ct: ColorType,
        at: AlphaType,
        cs: impl Into<Option<ColorSpace>>,
    ) -> Self {
        let dimensions = dimensions.into();
        ImageInfo::construct(|ii| unsafe {
            sb::C_SkImageInfo_Make(
                dimensions.width,
                dimensions.height,
                ct.into_native(),
                at,
                cs.into().into_ptr_or_null(),
                ii,
            )
        })
    }

    /// Creates [`ImageInfo`] from integral dimensions and [`ColorInfo`]. Parameters are not
    /// validated to see if their values are legal, or that the combination is supported.
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    /// - `color_info` pixel encoding consisting of [`ColorType`], [`AlphaType`], and [`ColorSpace`]
    ///   which may be `None`
    pub fn from_color_info(dimensions: impl Into<ISize>, color_info: ColorInfo) -> Self {
        // TODO: (perf) actually move of color_info.
        Self::new(
            dimensions,
            color_info.color_type(),
            color_info.alpha_type(),
            color_info.color_space(),
        )
    }

    /// Creates [`ImageInfo`] from integral dimensions, an N32 color type, `at`, and optionally
    /// [`ColorSpace`]. N32 will equal either BGRA 8888 or RGBA 8888, whichever is optimal.
    ///
    /// If `cs` is `None` and [`ImageInfo`] is part of a drawing source, [`ColorSpace`] defaults to
    /// sRGB, mapping into the surface [`ColorSpace`]. Parameters are not validated to see if their
    /// values are legal, or that the combination is supported.
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    /// - `at` alpha type
    /// - `cs` optional color space
    pub fn new_n32(
        dimensions: impl Into<ISize>,
        at: AlphaType,
        cs: impl Into<Option<ColorSpace>>,
    ) -> ImageInfo {
        let dimensions = dimensions.into();
        Self::construct(|ii| unsafe {
            sb::C_SkImageInfo_MakeN32(
                dimensions.width,
                dimensions.height,
                at,
                cs.into().into_ptr_or_null(),
                ii,
            )
        })
    }

    /// Creates [`ImageInfo`] from integral dimensions, an N32 color type, and `at`, with sRGB
    /// [`ColorSpace`]. Parameters are not validated to see if their values are legal, or that the
    /// combination is supported.
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    /// - `at` alpha type
    pub fn new_s32(dimensions: impl Into<ISize>, at: AlphaType) -> ImageInfo {
        let dimensions = dimensions.into();
        Self::construct(|ii| unsafe {
            sb::C_SkImageInfo_MakeS32(dimensions.width, dimensions.height, at, ii)
        })
    }

    /// Creates [`ImageInfo`] from integral dimensions, an N32 color type, premultiplied alpha,
    /// and optional [`ColorSpace`].
    ///
    /// If `cs` is `None` and [`ImageInfo`] is part of a drawing source, [`ColorSpace`] defaults to
    /// sRGB, mapping into the surface [`ColorSpace`]. Parameters are not validated to see if their
    /// values are legal, or that the combination is supported.
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    /// - `cs` optional color space
    pub fn new_n32_premul(
        dimensions: impl Into<ISize>,
        cs: impl Into<Option<ColorSpace>>,
    ) -> ImageInfo {
        let dimensions = dimensions.into();
        Self::construct(|ii| unsafe {
            sb::C_SkImageInfo_MakeN32Premul(
                dimensions.width,
                dimensions.height,
                cs.into().into_ptr_or_null(),
                ii,
            )
        })
    }

    /// Creates [`ImageInfo`] from integral dimensions, an alpha-only color type, premultiplied
    /// alpha, and no [`ColorSpace`].
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    pub fn new_a8(dimensions: impl Into<ISize>) -> ImageInfo {
        let dimensions = dimensions.into();
        Self::construct(|ii| unsafe {
            sb::C_SkImageInfo_MakeA8(dimensions.width, dimensions.height, ii)
        })
    }

    /// Creates [`ImageInfo`] from integral dimensions, an unknown color type, unknown alpha type,
    /// and no [`ColorSpace`]. An [`ImageInfo`] used as a source does not draw, and one used as a
    /// destination cannot be drawn to.
    ///
    /// - `dimensions` pixel column and row count; must be zero or greater
    pub fn new_unknown(dimensions: Option<ISize>) -> ImageInfo {
        let dimensions = dimensions.unwrap_or_default();
        Self::construct(|ii| unsafe {
            sb::C_SkImageInfo_MakeUnknown(dimensions.width, dimensions.height, ii)
        })
    }

    /// Returns the pixel count in each row.
    pub fn width(&self) -> i32 {
        self.dimensions().width
    }

    /// Returns the pixel row count.
    pub fn height(&self) -> i32 {
        self.dimensions().height
    }

    /// Returns the color type.
    pub fn color_type(&self) -> ColorType {
        self.color_info().color_type()
    }

    /// Returns the alpha type.
    pub fn alpha_type(&self) -> AlphaType {
        self.color_info().alpha_type()
    }

    /// Returns [`ColorSpace`], the range of colors. The returned [`ColorSpace`] is immutable.
    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(unsafe { self.native().colorSpace() })
    }

    /// Returns true if either dimension is zero or smaller.
    pub fn is_empty(&self) -> bool {
        self.dimensions().is_empty()
    }

    /// Returns the dimensionless [`ColorInfo`] that represents the same color type, alpha type,
    /// and color space as this [`ImageInfo`].
    pub fn color_info(&self) -> &ColorInfo {
        Handle::from_native_ref(&self.native().fColorInfo)
    }

    /// Returns true if the alpha type is set to hint that all pixels are opaque; their alpha value
    /// is implicitly or explicitly 1.0. If true, and all pixels are not opaque, Skia may draw
    /// incorrectly.
    ///
    /// This does not check if the color type allows alpha, or if any pixel value has transparency.
    pub fn is_opaque(&self) -> bool {
        self.color_info().is_opaque()
    }

    /// Returns the integral size of [`Self::width()`] and [`Self::height()`].
    pub fn dimensions(&self) -> ISize {
        ISize::from_native_c(self.native().fDimensions)
    }

    /// Returns the integral rectangle from the origin to [`Self::width()`] and [`Self::height()`].
    pub fn bounds(&self) -> IRect {
        IRect::from_size(self.dimensions())
    }

    /// Returns true if the associated [`ColorSpace`] is not `None`, and the [`ColorSpace`] gamma
    /// is approximately the same as sRGB.
    pub fn is_gamma_close_to_srgb(&self) -> bool {
        self.color_info().is_gamma_close_to_srgb()
    }

    /// Creates [`ImageInfo`] with the same color type, color space, and alpha type, with dimensions
    /// set to `new_dimensions`.
    ///
    /// - `new_dimensions` pixel column and row count; must be zero or greater
    #[must_use]
    pub fn with_dimensions(&self, new_dimensions: impl Into<ISize>) -> Self {
        Self::from_color_info(new_dimensions, self.color_info().clone())
    }

    /// Creates [`ImageInfo`] with the same color type, color space, width, and height, with the
    /// alpha type set to `new_alpha_type`.
    ///
    /// The created [`ImageInfo`] contains `new_alpha_type` even if it is incompatible with the
    /// color type, in which case the alpha type in [`ImageInfo`] is ignored.
    ///
    /// - `new_alpha_type` new alpha type
    #[must_use]
    pub fn with_alpha_type(&self, new_alpha_type: AlphaType) -> Self {
        Self::from_color_info(
            self.dimensions(),
            self.color_info().with_alpha_type(new_alpha_type),
        )
    }

    /// Creates [`ImageInfo`] with the same alpha type, color space, width, and height, with the
    /// color type set to `new_color_type`.
    ///
    /// - `new_color_type` new color type
    #[must_use]
    pub fn with_color_type(&self, new_color_type: ColorType) -> Self {
        Self::from_color_info(
            self.dimensions(),
            self.color_info().with_color_type(new_color_type),
        )
    }

    /// Creates [`ImageInfo`] with the same alpha type, color type, width, and height, with the
    /// color space set to `new_color_space`. `new_color_space` may be `None`.
    ///
    /// - `new_color_space` optional color space
    #[must_use]
    pub fn with_color_space(&self, new_color_space: impl Into<Option<ColorSpace>>) -> Self {
        Self::construct(|ii| unsafe {
            sb::C_SkImageInfo_makeColorSpace(
                self.native(),
                new_color_space.into().into_ptr_or_null(),
                ii,
            )
        })
    }

    /// Returns the number of bytes per pixel required by the color type. Returns zero if the color
    /// type is [`ColorType::Unknown`].
    pub fn bytes_per_pixel(&self) -> usize {
        self.color_info().bytes_per_pixel()
    }

    /// Returns the bit shift converting row bytes to row pixels. Returns zero for
    /// [`ColorType::Unknown`]. Returns one of: 0, 1, 2, 3; left shift to convert pixels to bytes.
    pub fn shift_per_pixel(&self) -> usize {
        self.color_info().shift_per_pixel()
    }

    /// Returns the minimum number of bytes per row, computed from the pixel width and color type,
    /// which specifies bytes per pixel. The bitmap maximum value for row bytes must fit in 31 bits.
    pub fn min_row_bytes(&self) -> usize {
        usize::try_from(self.width()).unwrap() * self.bytes_per_pixel()
    }

    /// Returns the byte offset of a pixel from the pixel base address.
    ///
    /// Asserts in debug builds if `point` is outside of bounds. Does not assert if `row_bytes` is
    /// smaller than [`Self::min_row_bytes()`], even though the result may be incorrect.
    ///
    /// - `point` pixel column and row; both must be within the image bounds
    /// - `row_bytes` size of the pixel row or larger
    pub fn compute_offset(&self, point: impl Into<IPoint>, row_bytes: usize) -> usize {
        let point = point.into();
        unsafe { self.native().computeOffset(point.x, point.y, row_bytes) }
    }

    /// Returns the storage required by the pixel array, given [`ImageInfo`] dimensions, color
    /// type, and `row_bytes`. `row_bytes` is assumed to be at least [`Self::min_row_bytes()`].
    ///
    /// Returns zero if the height is zero. Returns `usize::MAX` if the answer exceeds the range of
    /// `usize`.
    ///
    /// - `row_bytes` size of the pixel row or larger
    pub fn compute_byte_size(&self, row_bytes: usize) -> usize {
        unsafe { self.native().computeByteSize(row_bytes) }
    }

    /// Returns the least storage required by the pixel buffer, using [`Self::min_row_bytes()`] to
    /// compute the bytes for each pixel row.
    ///
    /// Returns zero if the height is zero. Returns `usize::MAX` if the answer exceeds the range of
    /// `usize`.
    pub fn compute_min_byte_size(&self) -> usize {
        self.compute_byte_size(self.min_row_bytes())
    }

    /// Returns true if `row_bytes` is valid for this [`ImageInfo`].
    ///
    /// - `row_bytes` size of the pixel row including padding
    pub fn valid_row_bytes(&self, row_bytes: usize) -> bool {
        if row_bytes < self.min_row_bytes() {
            return false;
        }
        let shift = self.shift_per_pixel();
        let aligned_row_bytes = row_bytes >> shift << shift;
        aligned_row_bytes == row_bytes
    }

    /// Creates an empty [`ImageInfo`] with [`ColorType::Unknown`], [`AlphaType::Unknown`], a width
    /// and height of zero, and no [`ColorSpace`].
    pub fn reset(&mut self) -> &mut Self {
        unsafe { sb::C_SkImageInfo_reset(self.native_mut()) };
        self
    }

    /// Returns `true` if the `row_bytes` are valid for [`ImageInfo`] _and_ an image would fit into
    /// `pixels`.
    pub(crate) fn valid_pixels<P>(&self, row_bytes: usize, pixels: &[P]) -> bool {
        self.valid_row_bytes(row_bytes)
            && mem::size_of_val(pixels) >= self.compute_byte_size(row_bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::prelude::*;
    use crate::{AlphaType, ColorSpace, ImageInfo, named_gamut, named_transfer_fn};

    #[test]
    fn ref_cnt_in_relation_to_color_space() {
        // Skia shares one sRGB instance between all tests, so this test counts the references of a
        // color space that only it holds.
        let cs = ColorSpace::new_rgb(&named_transfer_fn::LINEAR, &named_gamut::REC2020).unwrap();
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
