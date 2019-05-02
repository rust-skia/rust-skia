use crate::prelude::*;
use crate::{ColorFilter, ColorSpace, ColorType, FilterQuality, IRect, Matrix, Rect};
use skia_bindings::{
    C_SkImageFilter_MakeMatrixFilter, C_SkImageFilter_computeFastBounds,
    C_SkImageFilter_makeWithLocalMatrix, SkColorFilter, SkColorSpace, SkImageFilter,
    SkImageFilterCache, SkImageFilter_Context, SkImageFilter_CropRect,
    SkImageFilter_CropRect_CropEdge, SkImageFilter_MapDirection, SkImageFilter_OutputProperties,
    SkImageFilter_TileUsage, SkRefCntBase,
};
use std::ptr;

#[repr(C)]
pub struct ImageFilterOutputProperties<'a> {
    color_type: ColorType,
    color_space: &'a SkColorSpace,
}

impl<'a> NativeTransmutable<SkImageFilter_OutputProperties> for ImageFilterOutputProperties<'a> {}

#[test]
fn test_output_properties_layout() {
    ImageFilterOutputProperties::test_layout();
}

impl<'a> ImageFilterOutputProperties<'a> {
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(self.color_space as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct ImageFilterContext<'a> {
    ctm: Matrix,
    clip_bounds: IRect,
    cache: &'a mut SkImageFilterCache,
    output_properties: ImageFilterOutputProperties<'a>,
}

impl<'a> NativeTransmutable<SkImageFilter_Context> for ImageFilterContext<'a> {}

#[test]
fn test_context_layout() {
    ImageFilterContext::test_layout();
}

impl<'a> ImageFilterContext<'a> {
    pub fn ctm(&self) -> &Matrix {
        &self.ctm
    }

    pub fn clip_bounds(&self) -> &IRect {
        &self.clip_bounds
    }

    // TODO support access to SkImageFilterCache, even though it's declared in src/core?

    pub fn output_properties(&self) -> &ImageFilterOutputProperties {
        &self.output_properties
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }
}

bitflags! {
    pub struct ImageFilterCropRectCropEdge : u32 {
        const HAS_LEFT = SkImageFilter_CropRect_CropEdge::kHasLeft_CropEdge as _;
        const HAS_TOP = SkImageFilter_CropRect_CropEdge::kHasTop_CropEdge as _;
        const HAS_WIDTH = SkImageFilter_CropRect_CropEdge::kHasWidth_CropEdge as _;
        const HAS_HEIGHT = SkImageFilter_CropRect_CropEdge::kHasHeight_CropEdge as _;
        const HAS_ALL = Self::HAS_LEFT.bits | Self::HAS_TOP.bits | Self::HAS_WIDTH.bits | Self::HAS_HEIGHT.bits;
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ImageFilterCropRect(SkImageFilter_CropRect);

impl NativeTransmutable<SkImageFilter_CropRect> for ImageFilterCropRect {}

#[test]
fn test_crop_rect_layout() {
    ImageFilterCropRect::test_layout();
}

impl Default for ImageFilterCropRect {
    fn default() -> Self {
        ImageFilterCropRect::from_native(unsafe { SkImageFilter_CropRect::new() })
    }
}

impl ImageFilterCropRect {
    pub fn new<R: AsRef<Rect>>(rect: R, flags: ImageFilterCropRectCropEdge) -> Self {
        ImageFilterCropRect::from_native(unsafe {
            SkImageFilter_CropRect::new1(rect.as_ref().native(), flags.bits())
        })
    }

    pub fn flags(&self) -> ImageFilterCropRectCropEdge {
        ImageFilterCropRectCropEdge::from_bits_truncate(unsafe { self.native().flags() })
    }

    pub fn rect(&self) -> &Rect {
        Rect::from_native_ref(unsafe { &*self.native().rect() })
    }

    pub fn apply_to<IR: AsRef<IRect>>(
        &self,
        image_bounds: IR,
        matrix: &Matrix,
        embiggen: bool,
    ) -> IRect {
        let mut cropped = IRect::default();
        unsafe {
            self.native().applyTo(
                image_bounds.as_ref().native(),
                matrix.native(),
                embiggen,
                cropped.native_mut(),
            )
        }
        cropped
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ImageFilterTileUsage {
    Possible = SkImageFilter_TileUsage::kPossible_TileUsage as _,
    Never = SkImageFilter_TileUsage::kNever_TileUsage as _,
}

impl NativeTransmutable<SkImageFilter_TileUsage> for ImageFilterTileUsage {}
#[test]
fn test_tile_usage_layout() {
    ImageFilterTileUsage::test_layout();
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ImageFilterMapDirection {
    Forward = SkImageFilter_MapDirection::kForward_MapDirection as _,
    Reverse = SkImageFilter_MapDirection::kReverse_MapDirection as _,
}

impl NativeTransmutable<SkImageFilter_MapDirection> for ImageFilterMapDirection {}
#[test]
fn test_map_direction_layout() {
    ImageFilterMapDirection::test_layout();
}

pub type ImageFilter = RCHandle<SkImageFilter>;

impl NativeRefCountedBase for SkImageFilter {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl RCHandle<SkImageFilter> {
    // TODO: filterImage() SkSpecialImage is declared in src/core/

    pub fn filter_bounds<'a, IRS: AsRef<IRect>, IR: Into<Option<&'a IRect>>>(
        &self,
        src: IRS,
        ctm: &Matrix,
        map_direction: ImageFilterMapDirection,
        input_rect: IR,
    ) -> IRect {
        IRect::from_native(unsafe {
            self.native().filterBounds(
                src.as_ref().native(),
                ctm.native(),
                map_direction.into_native(),
                input_rect.into().native_ptr_or_null(),
            )
        })
    }

    // TODO: DrawWithFP()

    pub fn color_filter_node(&self) -> Option<ColorFilter> {
        let mut filter_ptr: *mut SkColorFilter = ptr::null_mut();
        if unsafe { self.native().isColorFilterNode(&mut filter_ptr) } {
            // according to the documentation, this must be set to a ref'd colorfilter
            // (which is one with an increased ref count I assume).
            ColorFilter::from_ptr(filter_ptr)
        } else {
            None
        }
    }

    // TODO: removeKey() SkImageFilterCacheKey is declared in src/core/

    pub fn as_a_color_filter(&self) -> Option<ColorFilter> {
        let mut filter_ptr: *mut SkColorFilter = ptr::null_mut();
        if unsafe { self.native().asAColorFilter(&mut filter_ptr) } {
            ColorFilter::from_ptr(filter_ptr)
        } else {
            None
        }
    }

    pub fn count_inputs(&self) -> usize {
        unsafe { self.native().countInputs() }.try_into().unwrap()
    }

    pub fn input(&self, i: usize) -> Option<ImageFilter> {
        ImageFilter::from_unshared_ptr(unsafe { self.native().getInput(i.try_into().unwrap()) })
    }

    pub fn crop_rect_is_set(&self) -> bool {
        unsafe { self.native().cropRectIsSet() }
    }

    pub fn crop_rect(&self) -> ImageFilterCropRect {
        ImageFilterCropRect::from_native(unsafe { self.native().getCropRect() })
    }

    pub fn compute_fast_bounds<R: AsRef<Rect>>(&self, bounds: R) -> Rect {
        Rect::from_native(unsafe {
            C_SkImageFilter_computeFastBounds(self.native(), bounds.as_ref().native())
        })
    }

    pub fn can_compute_fast_bounds(&self) -> bool {
        unsafe { self.native().canComputeFastBounds() }
    }

    pub fn with_local_matrix(&self, matrix: &Matrix) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkImageFilter_makeWithLocalMatrix(self.native(), matrix.native())
        })
    }

    pub fn can_handle_complex_ctm(&self) -> bool {
        unsafe { self.native().canHandleComplexCTM() }
    }

    // TODO: rename from_matrix?
    pub fn new_matrix_filter(
        matrix: &Matrix,
        quality: FilterQuality,
        input: &ImageFilter,
    ) -> ImageFilter {
        ImageFilter::from_ptr(unsafe {
            C_SkImageFilter_MakeMatrixFilter(
                matrix.native(),
                quality.into_native(),
                input.shared_native(),
            )
        })
        .unwrap()
    }

    // TODO: GetFlattenabletype()?
    // TODO: getFlattenableType()?
    // TODO: Deserialize?
}
