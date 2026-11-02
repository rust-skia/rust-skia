//! Measures a [`crate::Path`]: computes its length and the position and tangent at a given
//! distance.

use crate::{ContourMeasure, Matrix, Path, PathBuilder, Point, Vector, prelude::*, scalar};
use skia_bindings::{self as sb, SkPathMeasure};
use std::fmt;

pub type PathMeasure = Handle<SkPathMeasure>;

impl NativeDrop for SkPathMeasure {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathMeasure_destruct(self) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MatrixFlags : u32 {
        const GET_POSITION = sb::SkPathMeasure_MatrixFlags_kGetPosition_MatrixFlag as _;
        const GET_TANGENT = sb::SkPathMeasure_MatrixFlags_kGetTangent_MatrixFlag as _;
        const GET_POS_AND_TAN = Self::GET_POSITION.bits() | Self::GET_TANGENT.bits();
    }
}

impl Default for MatrixFlags {
    fn default() -> Self {
        Self::GET_POS_AND_TAN
    }
}

impl Default for PathMeasure {
    fn default() -> Self {
        Self::from_native_c(unsafe { SkPathMeasure::new() })
    }
}

impl fmt::Debug for PathMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathMeasure")
            // TODO: self must be mut
            // .field("length", &self.length())
            // .field("is_closed", &self.is_closed())
            // .field("next_contour", &self.next_contour())
            .field("current_measure", &self.current_measure())
            .finish()
    }
}

/// Warning: Even if you pass in a `PathMeasure` with multiple contours, most of this struct's functions, including `length` only return the value for the first contour on the path (which is why they aren't `const`). You must exhaust `PathMeasure::next_contour`.
///
/// ```
/// use skia_safe::{PathMeasure, Point, Path};
/// use std::f64::consts::PI;
/// let mut path = Path::circle((0., 0.), 10.0, None);
/// path.add_path(&Path::circle((100., 100.), 27.0, None), Point::default(), None);
/// let mut measure = PathMeasure::new(&path, false, None);
/// let mut lengths = vec![measure.length()];
/// while measure.next_contour() {
///     lengths.push(measure.length());
/// }
/// assert_eq!(*lengths.first().unwrap() as i64, (2. * PI * 10.0) as i64);
/// assert_eq!(*lengths.get(1).unwrap() as i64, (2. * PI * 27.0) as i64);
/// eprintln!("Circle lengths: {:?}", &lengths);
/// ```
impl PathMeasure {
    /// Initializes the path measure with the specified path. The parts of the path that are needed
    /// are copied, so the client is free to modify/delete the path after this call.
    ///
    /// `res_scale` controls the precision of the measure. Values > 1 increase the precision (and
    /// possibly slow down the computation).
    ///
    /// - `path` path to measure
    /// - `force_closed` whether to treat the path as closed
    /// - `res_scale` precision of the measure
    pub fn new(path: &Path, force_closed: bool, res_scale: impl Into<Option<scalar>>) -> Self {
        Self::from_native_c(unsafe {
            SkPathMeasure::new1(path.native(), force_closed, res_scale.into().unwrap_or(1.0))
        })
    }

    #[deprecated(since = "0.48.0", note = "Use PathMeasure::new")]
    pub fn from_path(
        path: &Path,
        force_closed: bool,
        res_scale: impl Into<Option<scalar>>,
    ) -> Self {
        Self::new(path, force_closed, res_scale)
    }

    /// Resets the path measure with the specified path. The parts of the path that are needed are
    /// copied, so the client is free to modify/delete the path after this call.
    ///
    /// - `path` path to measure
    /// - `force_closed` whether to treat the path as closed
    pub fn set_path(&mut self, path: &Path, force_closed: bool) -> &mut Self {
        unsafe { self.native_mut().setPath(path.native(), force_closed) }
        self
    }

    /// Returns the total length of the current contour, or 0 if no path is associated.
    pub fn length(&mut self) -> scalar {
        unsafe { self.native_mut().getLength() }
    }

    // TODO: rename to get_pos_tan(), because the function expects arguments?
    /// Pins `distance` to `0 <= distance <= length()`, and then computes the corresponding position
    /// and tangent. Returns `None` if there is no path, or a zero-length path was specified, in
    /// which case the position and tangent are unchanged.
    ///
    /// - `distance` distance along the contour
    #[must_use]
    pub fn pos_tan(&mut self, distance: scalar) -> Option<(Point, Vector)> {
        let mut position = Point::default();
        let mut tangent = Vector::default();
        unsafe {
            self.native_mut()
                .getPosTan(distance, position.native_mut(), tangent.native_mut())
        }
        .then_some((position, tangent))
    }

    #[deprecated(since = "0.88.0", note = "Use get_matrix()")]
    #[must_use]
    pub fn matrix(
        &mut self,
        distance: scalar,
        flags: impl Into<Option<MatrixFlags>>,
    ) -> Option<Matrix> {
        let mut m = Matrix::default();
        unsafe {
            self.native_mut().getMatrix(
                distance,
                m.native_mut(),
                // note: depending on the OS, different representation types are generated for MatrixFlags
                #[allow(clippy::useless_conversion)]
                flags.into().unwrap_or_default().bits().try_into().unwrap(),
            )
        }
        .then_some(m)
    }

    /// Pins `distance` to `0 <= distance <= length()`, and then computes the corresponding matrix
    /// (by calling [`Self::pos_tan()`]). Returns false if there is no path, or a zero-length path
    /// was specified, in which case `matrix` is unchanged.
    ///
    /// - `distance` distance along the contour
    /// - `matrix` storage for the computed matrix
    /// - `flags` which matrix components to compute
    #[must_use]
    pub fn get_matrix(
        &mut self,
        distance: scalar,
        matrix: &mut Matrix,
        flags: impl Into<Option<MatrixFlags>>,
    ) -> bool {
        unsafe {
            self.native_mut().getMatrix(
                distance,
                matrix.native_mut(),
                // note: depending on the OS, different representation types are generated for MatrixFlags
                #[allow(clippy::useless_conversion)]
                flags.into().unwrap_or_default().bits().try_into().unwrap(),
            )
        }
    }

    #[deprecated(since = "0.88.0", note = "Use get_segment()")]
    pub fn segment(
        &mut self,
        start_d: scalar,
        stop_d: scalar,
        start_with_move_to: bool,
    ) -> Option<Path> {
        let mut p = PathBuilder::default();
        unsafe {
            self.native_mut()
                .getSegment(start_d, stop_d, p.native_ptr(), start_with_move_to)
        }
        .then(|| p.detach())
    }

    /// Given a start and stop distance, appends to `dst` the intervening segment(s). If the segment
    /// is zero-length, returns false, else returns true. `start_d` and `stop_d` are pinned to legal
    /// values (`0..length()`). If `start_d > stop_d` then returns false (and leaves `dst`
    /// untouched). Begins the segment with a move-to if `start_with_move_to` is true.
    ///
    /// - `start_d` start distance
    /// - `stop_d` stop distance
    /// - `dst` path builder receiving the segment
    /// - `start_with_move_to` whether to begin with a move-to
    pub fn get_segment(
        &mut self,
        start_d: scalar,
        stop_d: scalar,
        dst: &mut PathBuilder,
        start_with_move_to: bool,
    ) -> bool {
        unsafe {
            self.native_mut()
                .getSegment(start_d, stop_d, dst.native_ptr(), start_with_move_to)
        }
    }

    /// Returns true if the current contour is closed.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed(&mut self) -> bool {
        unsafe { self.native_mut().isClosed() }
    }

    // TODO: rename to has_next_contour()?
    /// Moves to the next contour in the path. Returns true if one exists, or false if we're done
    /// with the path.
    pub fn next_contour(&mut self) -> bool {
        unsafe { self.native_mut().nextContour() }
    }

    pub fn current_measure(&self) -> &Option<ContourMeasure> {
        ContourMeasure::from_unshared_ptr_ref(&self.native().fContour.fPtr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Path, PathBuilder, PathMeasure};

    #[test]
    fn current_measure() {
        let mut builder = PathBuilder::new_path(&Path::circle((0., 0.), 10.0, None));
        builder.add_path(&Path::circle((100., 100.), 27.0, None), None);
        let path = builder.detach();

        let mut measure = PathMeasure::new(&path, false, None);
        while measure.next_contour() {
            eprintln!("contour: {:?}", measure.current_measure());
        }
        assert!(measure.current_measure().is_none());
    }
}
