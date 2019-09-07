use crate::prelude::*;
use crate::{
    ColorFilter, ColorSpace, ColorType, FilterQuality, IRect, Matrix, NativeFlattenable, Rect,
};
use skia_bindings as sb;
use skia_bindings::{
    SkColorFilter, SkColorSpace, SkFlattenable, SkImageFilter, SkImageFilterCache,
    SkImageFilter_Context, SkImageFilter_CropRect, SkImageFilter_MapDirection,
    SkImageFilter_OutputProperties, SkImageFilter_TileUsage, SkRefCntBase,
};
use std::ptr;

#[repr(C)]
pub struct OutputProperties<'a> {
    color_type: ColorType,
    color_space: &'a SkColorSpace,
}

impl<'a> NativeTransmutable<SkImageFilter_OutputProperties> for OutputProperties<'a> {}

#[test]
fn test_output_properties_layout() {
    OutputProperties::test_layout();
}

impl<'a> OutputProperties<'a> {
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(self.color_space as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct Context<'a> {
    ctm: Matrix,
    clip_bounds: IRect,
    cache: &'a mut SkImageFilterCache,
    output_properties: OutputProperties<'a>,
}

impl<'a> NativeTransmutable<SkImageFilter_Context> for Context<'a> {}

#[test]
fn test_context_layout() {
    Context::test_layout();
}

impl<'a> Context<'a> {
    pub fn ctm(&self) -> &Matrix {
        &self.ctm
    }

    pub fn clip_bounds(&self) -> &IRect {
        &self.clip_bounds
    }

    // TODO support access to SkImageFilterCache, even though it's declared in src/core?

    pub fn output_properties(&self) -> &OutputProperties {
        &self.output_properties
    }

    pub fn is_valid(&self) -> bool {
        self.ctm.is_finite()
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct CropRect(SkImageFilter_CropRect);

impl NativeTransmutable<SkImageFilter_CropRect> for CropRect {}

#[test]
fn test_crop_rect_layout() {
    CropRect::test_layout();
}

impl Default for CropRect {
    fn default() -> Self {
        CropRect::from_native(SkImageFilter_CropRect {
            fRect: Rect::default().into_native(),
            fFlags: 0,
        })
    }
}

impl CropRect {
    pub fn new(rect: impl AsRef<Rect>, flags: impl Into<Option<crop_rect::CropEdge>>) -> Self {
        CropRect::from_native(SkImageFilter_CropRect {
            fRect: rect.as_ref().into_native(),
            fFlags: flags.into().unwrap_or(crop_rect::CropEdge::HAS_ALL).bits(),
        })
    }

    pub fn flags(&self) -> crop_rect::CropEdge {
        crop_rect::CropEdge::from_bits_truncate(self.native().fFlags)
    }

    pub fn rect(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fRect)
    }

    pub fn apply_to(
        &self,
        image_bounds: impl AsRef<IRect>,
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

pub mod crop_rect {
    use skia_bindings::SkImageFilter_CropRect_CropEdge;

    bitflags! {
        pub struct CropEdge : u32 {
            const HAS_LEFT = SkImageFilter_CropRect_CropEdge::kHasLeft_CropEdge as _;
            const HAS_TOP = SkImageFilter_CropRect_CropEdge::kHasTop_CropEdge as _;
            const HAS_WIDTH = SkImageFilter_CropRect_CropEdge::kHasWidth_CropEdge as _;
            const HAS_HEIGHT = SkImageFilter_CropRect_CropEdge::kHasHeight_CropEdge as _;
            const HAS_ALL = Self::HAS_LEFT.bits | Self::HAS_TOP.bits | Self::HAS_WIDTH.bits | Self::HAS_HEIGHT.bits;
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum TileUsage {
    Possible = SkImageFilter_TileUsage::kPossible_TileUsage as _,
    Never = SkImageFilter_TileUsage::kNever_TileUsage as _,
}

impl NativeTransmutable<SkImageFilter_TileUsage> for TileUsage {}
#[test]
fn test_tile_usage_layout() {
    TileUsage::test_layout();
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum MapDirection {
    Forward = SkImageFilter_MapDirection::kForward_MapDirection as _,
    Reverse = SkImageFilter_MapDirection::kReverse_MapDirection as _,
}

impl NativeTransmutable<SkImageFilter_MapDirection> for MapDirection {}
#[test]
fn test_map_direction_layout() {
    MapDirection::test_layout();
}

pub type ImageFilter = RCHandle<SkImageFilter>;

impl NativeRefCountedBase for SkImageFilter {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl NativeFlattenable for SkImageFilter {
    fn native_flattenable(&self) -> &SkFlattenable {
        &self._base
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkImageFilter_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl RCHandle<SkImageFilter> {
    // TODO: wrapfilterImage()? SkSpecialImage is declared in src/core/

    pub fn filter_bounds<'a>(
        &self,
        src: impl AsRef<IRect>,
        ctm: &Matrix,
        map_direction: MapDirection,
        input_rect: impl Into<Option<&'a IRect>>,
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
        if unsafe { sb::C_SkImageFilter_isColorFilterNode(self.native(), &mut filter_ptr) } {
            // according to the documentation, this must be set to a ref'd colorfilter
            // (which is one with an increased ref count I assume).
            ColorFilter::from_ptr(filter_ptr)
        } else {
            None
        }
    }

    // TODO: removeKey() SkImageFilterCacheKey is declared in src/core/

    #[deprecated(since = "0.12.0", note = "use to_a_color_filter()")]
    pub fn as_a_color_filter(&self) -> Option<ColorFilter> {
        self.to_a_color_filter()
    }

    pub fn to_a_color_filter(&self) -> Option<ColorFilter> {
        let mut filter_ptr: *mut SkColorFilter = ptr::null_mut();
        if unsafe { self.native().asAColorFilter(&mut filter_ptr) } {
            // If set, filter_ptr is also "ref'd" here, so we don't
            // need to increase the reference count.
            ColorFilter::from_ptr(filter_ptr)
        } else {
            None
        }
    }

    pub fn count_inputs(&self) -> usize {
        unsafe { sb::C_SkImageFilter_countInputs(self.native()) }
            .try_into()
            .unwrap()
    }

    #[deprecated(note = "use get_input()")]
    pub fn input(&self, i: usize) -> Option<ImageFilter> {
        self.get_input(i)
    }

    pub fn get_input(&self, i: usize) -> Option<ImageFilter> {
        assert!(i < self.count_inputs());
        ImageFilter::from_unshared_ptr(unsafe {
            sb::C_SkImageFilter_getInput(self.native(), i.try_into().unwrap())
        })
    }

    // TODO: rename to is_crop_rect_set() ?
    pub fn crop_rect_is_set(&self) -> bool {
        !self.crop_rect().flags().is_empty()
    }

    pub fn crop_rect(&self) -> CropRect {
        CropRect::from_native(self.native().fCropRect)
    }

    pub fn compute_fast_bounds(&self, bounds: impl AsRef<Rect>) -> Rect {
        Rect::from_native(unsafe {
            sb::C_SkImageFilter_computeFastBounds(self.native(), bounds.as_ref().native())
        })
    }

    pub fn can_compute_fast_bounds(&self) -> bool {
        unsafe { self.native().canComputeFastBounds() }
    }

    #[deprecated(since = "0.12.0", note = "use with_local_matrix()")]
    pub fn new_with_local_matrix(&self, matrix: &Matrix) -> Option<ImageFilter> {
        self.with_local_matrix(matrix)
    }

    pub fn with_local_matrix(&self, matrix: &Matrix) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            sb::C_SkImageFilter_makeWithLocalMatrix(self.native(), matrix.native())
        })
    }

    pub fn can_handle_complex_ctm(&self) -> bool {
        unsafe { self.native().canHandleComplexCTM() }
    }

    pub fn with_matrix(self, matrix: &Matrix, quality: FilterQuality) -> ImageFilter {
        ImageFilter::from_ptr(unsafe {
            sb::C_SkImageFilter_MakeMatrixFilter(
                matrix.native(),
                quality.into_native(),
                self.into_ptr(),
            )
        })
        .unwrap()
    }
}
