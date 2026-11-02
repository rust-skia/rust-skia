use std::{fmt, marker::PhantomData};

use skia_bindings::{
    self as sb, SkPath, SkPathContourIter, SkPathContourIter_Rec, SkPathIter, SkPathIter_Rec,
};

use crate::{PathVerb, Point, prelude::*};

#[repr(transparent)]
pub struct PathIter<'a>(SkPathIter, PhantomData<&'a Handle<SkPath>>);

impl NativeAccess for PathIter<'_> {
    type Native = SkPathIter;

    fn native(&self) -> &SkPathIter {
        &self.0
    }
}

impl NativeAccessMut for PathIter<'_> {
    fn native_mut(&mut self) -> &mut SkPathIter {
        &mut self.0
    }
}

impl Drop for PathIter<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathIter_destruct(&mut self.0) }
    }
}

impl fmt::Debug for PathIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathIter").finish()
    }
}

/// A cubic uses the most points of all verbs, four.
const MAX_REC_POINTS: usize = 4;

// Fails to compile when Skia adds a verb, so MAX_REC_POINTS gets rechecked.
const _: () = match PathVerb::Move {
    PathVerb::Move
    | PathVerb::Line
    | PathVerb::Quad
    | PathVerb::Conic
    | PathVerb::Cubic
    | PathVerb::Close => (),
};

/// One verb of a path and the points it uses, as returned by [`PathIter`].
///
/// The record owns a copy of its points. The lifetime parameter is kept for API compatibility.
#[derive(Copy, Clone)]
pub struct PathIterRec<'a> {
    points: [Point; MAX_REC_POINTS],
    points_len: usize,
    conic_weight: f32,
    verb: PathVerb,
    _lifetime: PhantomData<&'a ()>,
}

impl fmt::Debug for PathIterRec<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathIterRec")
            .field("points", &self.points())
            .field("conic_weight", &self.conic_weight())
            .field("verb", &self.verb())
            .finish()
    }
}

impl PathIterRec<'_> {
    /// Copies the points of a native record.
    ///
    /// For move and close verbs, the native points live inside the `SkPathIter`, not in the path,
    /// and the next call to `next()` overwrites them. An [`Iterator`] can not return items that
    /// borrow from itself, so the record has to own its points.
    fn from_native(rec: &SkPathIter_Rec) -> Self {
        let source: &[Point] = unsafe {
            safer::from_raw_parts(Point::from_native_ptr(rec.fPoints.fPtr), rec.fPoints.fSize)
        };
        let mut points = [Point::default(); MAX_REC_POINTS];
        points[..source.len()].copy_from_slice(source);
        Self {
            points,
            points_len: source.len(),
            conic_weight: rec.fConicWeight,
            verb: rec.fVerb,
            _lifetime: PhantomData,
        }
    }

    pub fn points(&self) -> &[Point] {
        &self.points[..self.points_len]
    }

    pub fn conic_weight(&self) -> f32 {
        self.conic_weight
    }

    pub fn verb(&self) -> PathVerb {
        self.verb
    }
}

impl<'a> PathIter<'a> {
    pub fn new(points: &'a [Point], verbs: &'a [PathVerb], conics: &'a [f32]) -> Self {
        let iter: SkPathIter = construct(|iter| unsafe {
            sb::C_SkPathIter_Construct(
                points.native().as_ptr(),
                points.len(),
                verbs.as_ptr(),
                verbs.len(),
                conics.as_ptr(),
                conics.len(),
                iter,
            )
        });
        Self(iter, PhantomData)
    }

    pub(crate) fn from_native_c(native: SkPathIter) -> Self {
        Self(native, PhantomData)
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = PathIterRec<'a>;

    /// Holds the current verb, and its associated points
    /// move:  `points()[0]`
    /// line:  `points()[0..1]`
    /// quad:  `points()[0..2]`
    /// conic: `points()[0..2]` `conic_weight()`
    /// cubic: `points()[0..3]`
    /// close: `points()[0..1]` ... as if close were a line from `pts[0]` to `pts[1]`
    fn next(&mut self) -> Option<Self::Item> {
        try_construct(|r| unsafe { sb::C_SkPathIter_next(self.native_mut(), r) })
            .map(|r| PathIterRec::from_native(&r.into_inner()))
    }
}

impl PathIter<'_> {
    pub fn peek_next_verb(&mut self) -> Option<PathVerb> {
        let mut verb = PathVerb::Close;
        unsafe { sb::C_SkPathIter_peekNextVerb(self.native_mut(), &mut verb) }.then_some(verb)
    }
}

#[repr(transparent)]
pub struct PathContourIter<'a>(SkPathContourIter, PhantomData<&'a Handle<SkPath>>);

impl NativeAccess for PathContourIter<'_> {
    type Native = SkPathContourIter;

    fn native(&self) -> &SkPathContourIter {
        &self.0
    }
}

impl NativeAccessMut for PathContourIter<'_> {
    fn native_mut(&mut self) -> &mut SkPathContourIter {
        &mut self.0
    }
}

impl Drop for PathContourIter<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathContourIter_destruct(&mut self.0) }
    }
}

impl fmt::Debug for PathContourIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathContourIter").finish()
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PathContourIterRec<'a>(SkPathContourIter_Rec, PhantomData<&'a ()>);

impl NativeAccess for PathContourIterRec<'_> {
    type Native = SkPathContourIter_Rec;

    fn native(&self) -> &Self::Native {
        &self.0
    }
}

impl NativeAccessMut for PathContourIterRec<'_> {
    fn native_mut(&mut self) -> &mut Self::Native {
        &mut self.0
    }
}

impl fmt::Debug for PathContourIterRec<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathContourIterRec")
            .field("points", &self.points())
            .field("verb", &self.verbs())
            .field("conics", &self.conics())
            .finish()
    }
}

impl PathContourIterRec<'_> {
    pub fn points(&self) -> &[Point] {
        unsafe {
            safer::from_raw_parts(
                Point::from_native_ptr(self.native().fPoints.fPtr),
                self.native().fPoints.fSize,
            )
        }
    }

    pub fn verbs(&self) -> &[PathVerb] {
        unsafe { safer::from_raw_parts(self.native().fVerbs.fPtr, self.native().fVerbs.fSize) }
    }

    pub fn conics(&self) -> &[f32] {
        unsafe { safer::from_raw_parts(self.native().fConics.fPtr, self.native().fConics.fSize) }
    }
}

impl<'a> PathContourIter<'a> {
    pub fn new(points: &'a [Point], verbs: &'a [PathVerb], conics: &'a [f32]) -> Self {
        let iter: SkPathContourIter = construct(|iter| unsafe {
            sb::C_SkPathContourIter_Construct(
                points.native().as_ptr(),
                points.len(),
                verbs.as_ptr(),
                verbs.len(),
                conics.as_ptr(),
                conics.len(),
                iter,
            )
        });
        Self(iter, PhantomData)
    }

    #[allow(unused)]
    pub(crate) fn from_native_c(native: SkPathContourIter) -> Self {
        Self(native, PhantomData)
    }
}

impl<'a> Iterator for PathContourIter<'a> {
    type Item = PathContourIterRec<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        try_construct(|r| unsafe { sb::C_SkPathContourIter_next(self.native_mut(), r) })
            .map(|r| PathContourIterRec(r.into_inner(), PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use crate::{PathBuilder, PathVerb, Point};

    /// The second contour overwrites the native iterator's move and close points.
    #[test]
    fn records_keep_their_points_after_the_iterator_advances() {
        let p0 = Point::new(0.0, 0.0);
        let p1 = Point::new(1.0, 0.0);
        let p2 = Point::new(5.0, 5.0);
        let p3 = Point::new(6.0, 5.0);
        let path = PathBuilder::new()
            .move_to(p0)
            .line_to(p1)
            .close()
            .move_to(p2)
            .line_to(p3)
            .close()
            .detach();

        let records_kept: Vec<_> = path.iter().collect();

        let expected = vec![
            (PathVerb::Move, vec![p0]),
            (PathVerb::Line, vec![p0, p1]),
            (PathVerb::Close, vec![p1, p0]),
            (PathVerb::Move, vec![p2]),
            (PathVerb::Line, vec![p2, p3]),
            (PathVerb::Close, vec![p3, p2]),
        ];
        let kept: Vec<_> = records_kept
            .iter()
            .map(|r| (r.verb(), r.points().to_vec()))
            .collect();
        assert_eq!(kept, expected);
    }
}
