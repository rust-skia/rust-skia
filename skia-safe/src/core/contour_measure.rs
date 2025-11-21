use crate::{prelude::*, scalar, Matrix, Path, PathBuilder, PathVerb, Point, Vector};
use skia_bindings::{
    self as sb, SkContourMeasure, SkContourMeasureIter, SkContourMeasure_ForwardVerbIterator,
    SkContourMeasure_VerbMeasure, SkRefCntBase,
};
use std::{fmt, marker::PhantomData};

pub type ContourMeasure = RCHandle<SkContourMeasure>;
unsafe_send_sync!(ContourMeasure);

impl NativeRefCountedBase for SkContourMeasure {
    type Base = SkRefCntBase;
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MatrixFlags : u32 {
        const GET_POSITION = sb::SkContourMeasure_MatrixFlags_kGetPosition_MatrixFlag as _;
        const GET_TANGENT = sb::SkContourMeasure_MatrixFlags_kGetTangent_MatrixFlag as _;
        const GET_POS_AND_TAN = Self::GET_POSITION.bits() | Self::GET_TANGENT.bits();
    }
}

impl Default for MatrixFlags {
    fn default() -> Self {
        Self::GET_POS_AND_TAN
    }
}

impl fmt::Debug for ContourMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContourMeasure")
            .field("length", &self.length())
            .field("is_closed", &self.is_closed())
            .finish()
    }
}

impl ContourMeasure {
    pub fn length(&self) -> scalar {
        unsafe { sb::C_SkContourMeasure_length(self.native()) }
    }

    #[must_use]
    pub fn pos_tan(&self, distance: scalar) -> Option<(Point, Vector)> {
        let mut p = Point::default();
        let mut v = Vector::default();
        unsafe {
            self.native()
                .getPosTan(distance, p.native_mut(), v.native_mut())
        }
        .then_some((p, v))
    }

    #[must_use]
    pub fn get_matrix(
        &self,
        distance: scalar,
        flags: impl Into<Option<MatrixFlags>>,
    ) -> Option<Matrix> {
        let mut m = Matrix::default();
        unsafe {
            self.native().getMatrix(
                distance,
                m.native_mut(),
                // note: depending on the OS, different representation types are generated for
                // MatrixFlags, so the try_into() is required, even though clippy complains about
                // it.
                #[allow(clippy::useless_conversion)]
                flags.into().unwrap_or_default().bits().try_into().unwrap(),
            )
        }
        .then_some(m)
    }

    #[must_use]
    pub fn segment(
        &self,
        start_d: scalar,
        stop_d: scalar,
        path_builder: &mut PathBuilder,
        start_with_move_to: bool,
    ) -> bool {
        unsafe {
            self.native().getSegment(
                start_d,
                stop_d,
                path_builder.native_mut(),
                start_with_move_to,
            )
        }
    }

    pub fn is_closed(&self) -> bool {
        unsafe { sb::C_SkContourMeasure_isClosed(self.native()) }
    }

    pub fn verbs(&self) -> ForwardVerbIterator {
        let iterator =
            construct(|iterator| unsafe { sb::C_SkContourMeasure_begin(self.native(), iterator) });

        ForwardVerbIterator {
            iterator,
            contour_measure: self,
        }
    }
}

pub struct ForwardVerbIterator<'a> {
    iterator: SkContourMeasure_ForwardVerbIterator,
    contour_measure: &'a ContourMeasure,
}
unsafe_send_sync!(ForwardVerbIterator<'_>);

impl fmt::Debug for ForwardVerbIterator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForwardVerbIterator").finish()
    }
}

impl PartialEq for ForwardVerbIterator<'_> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sb::C_SkContourMeasure_ForwardVerbIterator_Equals(&self.iterator, &other.iterator)
        }
    }
}

impl<'a> Iterator for ForwardVerbIterator<'a> {
    type Item = VerbMeasure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let end = construct(|end| unsafe {
            sb::C_SkContourMeasure_end(self.contour_measure.native(), end)
        });
        if unsafe { sb::C_SkContourMeasure_ForwardVerbIterator_Equals(&self.iterator, &end) } {
            return None;
        }
        let item = construct(|item| unsafe {
            sb::C_SkContourMeasure_ForwardVerbIterator_item(&self.iterator, item)
        });
        unsafe { sb::C_SkContourMeasure_ForwardVerbIterator_next(&mut self.iterator) };
        Some(VerbMeasure {
            verb_measure: item,
            _pd: PhantomData,
        })
    }
}

pub struct VerbMeasure<'a> {
    verb_measure: SkContourMeasure_VerbMeasure,
    _pd: PhantomData<ForwardVerbIterator<'a>>,
}
unsafe_send_sync!(VerbMeasure<'_>);

impl fmt::Debug for VerbMeasure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerbMeasure")
            .field("verb", &self.verb())
            .field("distance", &self.distance())
            .field("points", &self.points())
            .finish()
    }
}

impl VerbMeasure<'_> {
    pub fn verb(&self) -> PathVerb {
        self.verb_measure.fVerb
    }

    pub fn distance(&self) -> scalar {
        self.verb_measure.fDistance
    }

    pub fn points(&self) -> &[Point] {
        unsafe {
            safer::from_raw_parts(
                Point::from_native_ptr(self.verb_measure.fPts.fPtr),
                self.verb_measure.fPts.fSize,
            )
        }
    }
}

pub type ContourMeasureIter = Handle<SkContourMeasureIter>;
unsafe_send_sync!(ContourMeasureIter);

impl NativeDrop for SkContourMeasureIter {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkContourMeasureIter_destruct(self);
        }
    }
}

impl Iterator for ContourMeasureIter {
    type Item = ContourMeasure;

    fn next(&mut self) -> Option<Self::Item> {
        ContourMeasure::from_ptr(unsafe { sb::C_SkContourMeasureIter_next(self.native_mut()) })
    }
}

impl fmt::Debug for ContourMeasureIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContourMeasureIter").finish()
    }
}

impl ContourMeasureIter {
    // Canonical new:
    pub fn new(path: &Path, force_closed: bool, res_scale: impl Into<Option<scalar>>) -> Self {
        Self::from_path(path, force_closed, res_scale)
    }

    // TODO: rename to of_path? for_path?
    // TODO: may deprecate in favor of Self::new().
    pub fn from_path(
        path: &Path,
        force_closed: bool,
        res_scale: impl Into<Option<scalar>>,
    ) -> Self {
        Self::from_native_c(unsafe {
            SkContourMeasureIter::new1(path.native(), force_closed, res_scale.into().unwrap_or(1.0))
        })
    }

    pub fn reset(
        &mut self,
        path: &Path,
        force_closed: bool,
        res_scale: impl Into<Option<scalar>>,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .reset(path.native(), force_closed, res_scale.into().unwrap_or(1.0))
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ContourMeasureIter;
    use crate::{Path, Rect};

    #[test]
    fn contour_and_verb_measure() {
        let p = Path::rect(Rect::new(0.0, 0.0, 10.0, 10.0), None);
        let measure = ContourMeasureIter::new(&p, true, None);
        for contour in measure {
            for verb in contour.verbs() {
                println!("verb: {verb:?}")
            }
        }
    }
}
