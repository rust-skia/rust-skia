use crate::prelude::*;
use crate::{ColorFilter, IRect, Matrix, NativeFlattenable, Rect};
use skia_bindings as sb;
use skia_bindings::{SkColorFilter, SkFlattenable, SkImageFilter, SkRefCntBase};
use std::ptr;

pub use skia_bindings::SkImageFilter_MapDirection as MapDirection;
#[test]
fn test_map_direction_naming() {
    let _ = MapDirection::Forward;
}

pub type ImageFilter = RCHandle<SkImageFilter>;
unsafe impl Send for ImageFilter {}
unsafe impl Sync for ImageFilter {}

impl NativeBase<SkRefCntBase> for SkImageFilter {}
impl NativeBase<SkFlattenable> for SkImageFilter {}

impl NativeRefCountedBase for SkImageFilter {
    type Base = SkRefCntBase;
}

impl NativeFlattenable for SkImageFilter {
    fn native_flattenable(&self) -> &SkFlattenable {
        self.base()
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
        IRect::from_native_c(unsafe {
            sb::C_SkImageFilter_filterBounds(
                self.native(),
                src.as_ref().native(),
                ctm.native(),
                map_direction,
                input_rect.into().native_ptr_or_null(),
            )
        })
    }

    pub fn color_filter_node(&self) -> Option<ColorFilter> {
        let mut filter_ptr: *mut SkColorFilter = ptr::null_mut();
        if unsafe { sb::C_SkImageFilter_isColorFilterNode(self.native(), &mut filter_ptr) } {
            // according to the documentation, this must be set to a ref'd color filter
            // (which is one with an increased ref count I assume).
            ColorFilter::from_ptr(filter_ptr)
        } else {
            None
        }
    }

    // TODO: removeKey() SkImageFilterCacheKey is declared in src/core/

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
            sb::C_SkImageFilter_getInput(self.native(), i.try_into().unwrap()) as *mut _
        })
    }

    pub fn compute_fast_bounds(&self, bounds: impl AsRef<Rect>) -> Rect {
        Rect::from_native_c(unsafe {
            sb::C_SkImageFilter_computeFastBounds(self.native(), bounds.as_ref().native())
        })
    }

    pub fn can_compute_fast_bounds(&self) -> bool {
        unsafe { self.native().canComputeFastBounds() }
    }

    pub fn with_local_matrix(&self, matrix: &Matrix) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            sb::C_SkImageFilter_makeWithLocalMatrix(self.native(), matrix.native())
        })
    }
}
