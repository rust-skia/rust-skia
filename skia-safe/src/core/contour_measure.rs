use crate::prelude::*;
use crate::{scalar, Matrix, Path, Point, Vector};
use skia_bindings::{
    C_SkContourMeasureIter_destruct, C_SkContourMeasureIter_next, SkContourMeasure,
    SkContourMeasureIter, SkRefCntBase,
};

pub type CountourMeasure = RCHandle<SkContourMeasure>;

impl NativeRefCountedBase for SkContourMeasure {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

#[allow(clippy::module_inception)]
pub mod contour_measure {
    bitflags! {
        pub struct MatrixFlags : i32 {
            const GET_POSITION = skia_bindings::SkContourMeasure_MatrixFlags_kGetPosition_MatrixFlag as _;
            const GET_TANGENT = skia_bindings::SkContourMeasure_MatrixFlags_kGetTangent_MatrixFlag as _;
            const GET_POS_AND_TAN = Self::GET_POSITION.bits | Self::GET_TANGENT.bits;
        }
    }

    impl Default for MatrixFlags {
        fn default() -> Self {
            Self::GET_POS_AND_TAN
        }
    }
}

impl RCHandle<SkContourMeasure> {
    pub fn length(&self) -> scalar {
        unsafe { self.native().length() }
    }

    pub fn pos_tan(&self, distance: scalar) -> Option<(Point, Vector)> {
        let mut p = Point::default();
        let mut v = Vector::default();
        unsafe {
            self.native()
                .getPosTan(distance, p.native_mut(), v.native_mut())
        }
        .if_true_some((p, v))
    }

    pub fn matrix<F: Into<Option<contour_measure::MatrixFlags>>>(
        &self,
        distance: scalar,
        flags: F,
    ) -> Option<Matrix> {
        let mut m = Matrix::default();
        unsafe {
            self.native().getMatrix(
                distance,
                m.native_mut(),
                flags.into().unwrap_or_default().bits(),
            )
        }
        .if_true_some(m)
    }

    pub fn segment(
        &self,
        start_d: scalar,
        stop_d: scalar,
        start_with_move_to: bool,
    ) -> Option<Path> {
        let mut p = Path::default();
        unsafe {
            self.native()
                .getSegment(start_d, stop_d, p.native_mut(), start_with_move_to)
        }
        .if_true_some(p)
    }

    pub fn is_closed(&self) -> bool {
        unsafe { self.native().isClosed() }
    }
}

pub type ContourMeasureIter = Handle<SkContourMeasureIter>;

impl NativeDrop for SkContourMeasureIter {
    fn drop(&mut self) {
        // does not link:
        // unsafe { SkContourMeasureIter::destruct(self); }
        unsafe {
            C_SkContourMeasureIter_destruct(self);
        }
    }
}

impl Iterator for Handle<SkContourMeasureIter> {
    type Item = CountourMeasure;

    fn next(&mut self) -> Option<Self::Item> {
        CountourMeasure::from_ptr(unsafe { C_SkContourMeasureIter_next(self.native_mut()) })
    }
}

impl Handle<SkContourMeasureIter> {
    // TODO: rename to of_path? for_path?
    pub fn from_path(path: &Path, force_closed: bool, res_scale: Option<scalar>) -> Self {
        let res_scale = res_scale.unwrap_or(1.0);
        Self::from_native(unsafe {
            SkContourMeasureIter::new1(path.native(), force_closed, res_scale)
        })
    }

    pub fn reset(
        &mut self,
        path: &Path,
        force_closed: bool,
        res_scale: Option<scalar>,
    ) -> &mut Self {
        let res_scale = res_scale.unwrap_or(1.0);
        unsafe {
            self.native_mut()
                .reset(path.native(), force_closed, res_scale)
        }
        self
    }
}
