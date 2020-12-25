use crate::interop::DynamicMemoryWStream;
use crate::matrix::ApplyPerspectiveClip;
use crate::prelude::*;
use crate::{
    path_types, scalar, Data, Matrix, PathDirection, PathFillType, Point, RRect, Rect, Vector,
};
use skia_bindings as sb;
use skia_bindings::{SkPath, SkPath_Iter, SkPath_RawIter};
use std::marker::PhantomData;
use std::mem::forget;

#[deprecated(since = "0.25.0", note = "use PathDirection")]
pub use path_types::PathDirection as Direction;

#[deprecated(since = "0.25.0", note = "use PathFillType")]
pub use path_types::PathFillType as FillType;

pub use skia_bindings::SkPath_ArcSize as ArcSize;
#[test]
fn test_arc_size_naming() {
    let _ = ArcSize::Small;
}

pub use skia_bindings::SkPath_AddPathMode as AddPathMode;
#[test]
fn test_add_path_mode_naming() {
    let _ = AddPathMode::Append;
}

pub use path_types::PathSegmentMask as SegmentMask;

pub use skia_bindings::SkPath_Verb as Verb;
#[test]
pub fn test_verb_naming() {
    let _ = Verb::Line;
}

#[repr(C)]
pub struct Iter<'a>(SkPath_Iter, PhantomData<&'a Handle<SkPath>>);

impl NativeAccess<SkPath_Iter> for Iter<'_> {
    fn native(&self) -> &SkPath_Iter {
        &self.0
    }
    fn native_mut(&mut self) -> &mut SkPath_Iter {
        &mut self.0
    }
}

impl Drop for Iter<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_SkPath_Iter_destruct(&mut self.0) }
    }
}

impl Default for Iter<'_> {
    fn default() -> Self {
        Iter(unsafe { SkPath_Iter::new() }, PhantomData)
    }
}

impl Iter<'_> {
    pub fn new(path: &Path, force_close: bool) -> Iter {
        Iter(
            unsafe { SkPath_Iter::new1(path.native(), force_close) },
            PhantomData,
        )
    }

    pub fn set_path(mut self, path: &Path, force_close: bool) -> Iter {
        unsafe {
            self.0.setPath(path.native(), force_close);
        }
        let r = Iter(self.0, PhantomData);
        forget(self);
        r
    }

    pub fn conic_weight(&self) -> Option<scalar> {
        #[allow(clippy::map_clone)]
        self.native()
            .fConicWeights
            .into_option()
            .map(|p| unsafe { *p })
    }

    pub fn is_close_line(&self) -> bool {
        unsafe { sb::C_SkPath_Iter_isCloseLine(self.native()) }
    }

    pub fn is_closed_contour(&self) -> bool {
        unsafe { self.native().isClosedContour() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Verb, Vec<Point>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut points = [Point::default(); Verb::MAX_POINTS];
        let verb = unsafe { self.native_mut().next(points.native_mut().as_mut_ptr()) };
        if verb != Verb::Done {
            Some((verb, points[0..verb.points()].into()))
        } else {
            None
        }
    }
}

#[repr(C)]
#[deprecated(
    since = "0.30.0",
    note = "User Iter instead, RawIter will soon be removed."
)]
pub struct RawIter<'a>(SkPath_RawIter, PhantomData<&'a Handle<SkPath>>);

#[allow(deprecated)]
impl NativeAccess<SkPath_RawIter> for RawIter<'_> {
    fn native(&self) -> &SkPath_RawIter {
        &self.0
    }
    fn native_mut(&mut self) -> &mut SkPath_RawIter {
        &mut self.0
    }
}

#[allow(deprecated)]
impl Drop for RawIter<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_SkPath_RawIter_destruct(&mut self.0) }
    }
}

#[allow(deprecated)]
impl Default for RawIter<'_> {
    fn default() -> Self {
        RawIter(
            construct(|ri| unsafe { sb::C_SkPath_RawIter_Construct(ri) }),
            PhantomData,
        )
    }
}

#[allow(deprecated)]
impl RawIter<'_> {
    pub fn new(path: &Path) -> RawIter {
        RawIter::default().set_path(path)
    }

    pub fn set_path(mut self, path: &Path) -> RawIter {
        unsafe { self.native_mut().setPath(path.native()) }
        let r = RawIter(self.0, PhantomData);
        forget(self);
        r
    }

    pub fn peek(&self) -> Verb {
        unsafe { sb::C_SkPath_RawIter_peek(self.native()) }
    }

    pub fn conic_weight(&self) -> scalar {
        self.native().fConicWeight
    }
}

#[allow(deprecated)]
impl Iterator for RawIter<'_> {
    type Item = (Verb, Vec<Point>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut points = [Point::default(); Verb::MAX_POINTS];

        let verb = unsafe { self.native_mut().next(points.native_mut().as_mut_ptr()) };
        (verb != Verb::Done).if_true_some((verb, points[0..verb.points()].into()))
    }
}

pub type Path = Handle<SkPath>;
unsafe impl Send for Path {}
unsafe impl Sync for Path {}

impl NativeDrop for SkPath {
    fn drop(&mut self) {
        unsafe { sb::C_SkPath_destruct(self) }
    }
}

impl NativeClone for SkPath {
    fn clone(&self) -> Self {
        unsafe { SkPath::new1(self) }
    }
}

impl NativePartialEq for SkPath {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkPath_Equals(self, rhs) }
    }
}

impl Default for Handle<SkPath> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<SkPath> {
    pub fn new_from(
        points: &[Point],
        verbs: &[u8],
        conic_weights: &[scalar],
        fill_type: FillType,
        is_volatile: impl Into<Option<bool>>,
    ) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_Make(
                path,
                points.native().as_ptr(),
                points.len().try_into().unwrap(),
                verbs.as_ptr(),
                verbs.len().try_into().unwrap(),
                conic_weights.as_ptr(),
                conic_weights.len().try_into().unwrap(),
                fill_type,
                is_volatile.into().unwrap_or(false),
            )
        })
    }

    pub fn rect(rect: impl AsRef<Rect>, dir: impl Into<Option<PathDirection>>) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_Rect(
                path,
                rect.as_ref().native(),
                dir.into().unwrap_or(PathDirection::CW),
            )
        })
    }

    pub fn oval(oval: impl AsRef<Rect>, dir: impl Into<Option<PathDirection>>) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_Oval(
                path,
                oval.as_ref().native(),
                dir.into().unwrap_or(PathDirection::CW),
            )
        })
    }

    pub fn oval_with_start_index(
        oval: impl AsRef<Rect>,
        dir: PathDirection,
        start_index: usize,
    ) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_OvalWithStartIndex(
                path,
                oval.as_ref().native(),
                dir,
                start_index.try_into().unwrap(),
            )
        })
    }

    pub fn circle(
        center: impl Into<Point>,
        radius: scalar,
        dir: impl Into<Option<PathDirection>>,
    ) -> Self {
        let center = center.into();
        Self::construct(|path| unsafe {
            sb::C_SkPath_Circle(
                path,
                center.x,
                center.y,
                radius,
                dir.into().unwrap_or(PathDirection::CW),
            )
        })
    }

    pub fn rrect(rect: impl AsRef<RRect>, dir: impl Into<Option<PathDirection>>) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_RRect(
                path,
                rect.as_ref().native(),
                dir.into().unwrap_or(PathDirection::CW),
            )
        })
    }

    pub fn rrect_with_start_index(
        rect: impl AsRef<RRect>,
        dir: PathDirection,
        start_index: usize,
    ) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_RRectWithStartIndex(
                path,
                rect.as_ref().native(),
                dir,
                start_index.try_into().unwrap(),
            )
        })
    }

    pub fn polygon(
        pts: &[Point],
        is_closed: bool,
        fill_type: impl Into<Option<FillType>>,
        is_volatile: impl Into<Option<bool>>,
    ) -> Self {
        Self::construct(|path| unsafe {
            sb::C_SkPath_Polygon(
                path,
                pts.native().as_ptr(),
                pts.len().try_into().unwrap(),
                is_closed,
                fill_type.into().unwrap_or(FillType::Winding),
                is_volatile.into().unwrap_or(false),
            )
        })
    }

    pub fn line(a: impl Into<Point>, b: impl Into<Point>) -> Self {
        Self::polygon(&[a.into(), b.into()], false, None, None)
    }

    pub fn new() -> Self {
        Self::construct(|path| unsafe { sb::C_SkPath_Construct(path) })
    }

    pub fn is_interpolatable(&self, compare: &Path) -> bool {
        unsafe { self.native().isInterpolatable(compare.native()) }
    }

    pub fn interpolate(&self, ending: &Path, weight: scalar) -> Option<Path> {
        let mut out = Path::default();
        unsafe {
            self.native()
                .interpolate(ending.native(), weight, out.native_mut())
        }
        .if_true_some(out)
    }

    pub fn fill_type(&self) -> PathFillType {
        unsafe { sb::C_SkPath_getFillType(self.native()) }
    }

    pub fn set_fill_type(&mut self, ft: PathFillType) -> &mut Self {
        self.native_mut().set_fFillType(ft as _);
        self
    }

    pub fn is_inverse_fill_type(&self) -> bool {
        self.fill_type().is_inverse()
    }

    pub fn toggle_inverse_fill_type(&mut self) -> &mut Self {
        let inverse = self.native().fFillType() ^ 2;
        self.native_mut().set_fFillType(inverse);
        self
    }

    #[deprecated(since = "0.36.0", note = "Removed, use is_convex()")]
    pub fn convexity_type(&self) -> ! {
        panic!("Removed")
    }

    #[deprecated(since = "0.36.0", note = "Removed, use is_convex()")]
    pub fn convexity_type_or_unknown(&self) -> ! {
        panic!("Removed")
    }

    pub fn is_convex(&self) -> bool {
        unsafe { sb::C_SkPath_isConvex(self.native()) }
    }

    pub fn is_oval(&self) -> Option<Rect> {
        let mut bounds = Rect::default();
        unsafe { self.native().isOval(bounds.native_mut()) }.if_true_some(bounds)
    }

    pub fn is_rrect(&self) -> Option<RRect> {
        let mut rrect = RRect::default();
        unsafe { self.native().isRRect(rrect.native_mut()) }.if_true_some(rrect)
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() };
        self
    }

    pub fn rewind(&mut self) -> &mut Self {
        unsafe { self.native_mut().rewind() };
        self
    }

    pub fn is_empty(&self) -> bool {
        unsafe { sb::C_SkPath_isEmpty(self.native()) }
    }

    pub fn is_last_contour_closed(&self) -> bool {
        unsafe { self.native().isLastContourClosed() }
    }

    pub fn is_finite(&self) -> bool {
        unsafe { sb::C_SkPath_isFinite(self.native()) }
    }

    pub fn is_volatile(&self) -> bool {
        self.native().fIsVolatile() != 0
    }

    pub fn set_is_volatile(&mut self, is_volatile: bool) -> &mut Self {
        self.native_mut().set_fIsVolatile(is_volatile as _);
        self
    }

    pub fn is_line_degenerate(p1: impl Into<Point>, p2: impl Into<Point>, exact: bool) -> bool {
        unsafe { SkPath::IsLineDegenerate(p1.into().native(), p2.into().native(), exact) }
    }

    pub fn is_quad_degenerate(
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        p3: impl Into<Point>,
        exact: bool,
    ) -> bool {
        unsafe {
            SkPath::IsQuadDegenerate(
                p1.into().native(),
                p2.into().native(),
                p3.into().native(),
                exact,
            )
        }
    }

    pub fn is_cubic_degenerate(
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        p3: impl Into<Point>,
        p4: impl Into<Point>,
        exact: bool,
    ) -> bool {
        unsafe {
            SkPath::IsCubicDegenerate(
                p1.into().native(),
                p2.into().native(),
                p3.into().native(),
                p4.into().native(),
                exact,
            )
        }
    }

    pub fn is_line(&self) -> Option<(Point, Point)> {
        let mut line = [Point::default(); 2];
        unsafe { self.native().isLine(line.native_mut().as_mut_ptr()) }
            .if_true_some((line[0], line[1]))
    }

    pub fn count_points(&self) -> usize {
        unsafe { self.native().countPoints().try_into().unwrap() }
    }

    pub fn get_point(&self, index: usize) -> Option<Point> {
        let p = Point::from_native_c(unsafe {
            sb::C_SkPath_getPoint(self.native(), index.try_into().unwrap())
        });
        // assuming that count_points() is somewhat slow, we
        // check the index when a Point(0,0) is returned.
        if p != Point::default() || index < self.count_points() {
            Some(p)
        } else {
            None
        }
    }

    pub fn get_points(&self, points: &mut [Point]) -> usize {
        unsafe {
            self.native().getPoints(
                points.native_mut().as_mut_ptr(),
                points.len().try_into().unwrap(),
            )
        }
        .try_into()
        .unwrap()
    }

    pub fn count_verbs(&self) -> usize {
        unsafe { self.native().countVerbs() }.try_into().unwrap()
    }

    pub fn get_verbs(&self, verbs: &mut [u8]) -> usize {
        unsafe {
            self.native()
                .getVerbs(verbs.as_mut_ptr(), verbs.len().try_into().unwrap())
        }
        .try_into()
        .unwrap()
    }

    pub fn approximate_bytes_used(&self) -> usize {
        unsafe { self.native().approximateBytesUsed() }
    }

    pub fn swap(&mut self, other: &mut Path) -> &mut Self {
        unsafe { self.native_mut().swap(other.native_mut()) }
        self
    }

    pub fn bounds(&self) -> &Rect {
        Rect::from_native_ref(unsafe { &*sb::C_SkPath_getBounds(self.native()) })
    }

    pub fn update_bounds_cache(&mut self) -> &mut Self {
        self.bounds();
        self
    }

    pub fn compute_tight_bounds(&self) -> Rect {
        Rect::from_native_c(unsafe { sb::C_SkPath_computeTightBounds(self.native()) })
    }

    pub fn conservatively_contains_rect(&self, rect: impl AsRef<Rect>) -> bool {
        unsafe {
            self.native()
                .conservativelyContainsRect(rect.as_ref().native())
        }
    }

    pub fn inc_reserve(&mut self, extra_pt_count: usize) -> &mut Self {
        unsafe {
            self.native_mut()
                .incReserve(extra_pt_count.try_into().unwrap())
        }
        self
    }

    #[deprecated(since = "0.37.0", note = "Removed without replacement")]
    pub fn shrink_to_fit(&mut self) -> ! {
        panic!("Removed without replacement");
    }

    pub fn move_to(&mut self, p: impl Into<Point>) -> &mut Self {
        let p = p.into();
        unsafe {
            self.native_mut().moveTo(p.x, p.y);
        }
        self
    }

    pub fn r_move_to(&mut self, d: impl Into<Vector>) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().rMoveTo(d.x, d.y);
        }
        self
    }

    pub fn line_to(&mut self, p: impl Into<Point>) -> &mut Self {
        let p = p.into();
        unsafe {
            self.native_mut().lineTo(p.x, p.y);
        }
        self
    }

    pub fn r_line_to(&mut self, d: impl Into<Vector>) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().rLineTo(d.x, d.y);
        }
        self
    }

    pub fn quad_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>) -> &mut Self {
        let p1 = p1.into();
        let p2 = p2.into();
        unsafe {
            self.native_mut().quadTo(p1.x, p1.y, p2.x, p2.y);
        }
        self
    }

    pub fn r_quad_to(&mut self, dx1: impl Into<Vector>, dx2: impl Into<Vector>) -> &mut Self {
        let (dx1, dx2) = (dx1.into(), dx2.into());
        unsafe {
            self.native_mut().rQuadTo(dx1.x, dx1.y, dx2.x, dx2.y);
        }
        self
    }

    pub fn conic_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>, w: scalar) -> &mut Self {
        let p1 = p1.into();
        let p2 = p2.into();
        unsafe {
            self.native_mut().conicTo(p1.x, p1.y, p2.x, p2.y, w);
        }
        self
    }

    pub fn r_conic_to(
        &mut self,
        d1: impl Into<Vector>,
        d2: impl Into<Vector>,
        w: scalar,
    ) -> &mut Self {
        let (d1, d2) = (d1.into(), d2.into());
        unsafe {
            self.native_mut().rConicTo(d1.x, d1.y, d2.x, d2.y, w);
        }
        self
    }

    pub fn cubic_to(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        p3: impl Into<Point>,
    ) -> &mut Self {
        let (p1, p2, p3) = (p1.into(), p2.into(), p3.into());
        unsafe {
            self.native_mut()
                .cubicTo(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        }
        self
    }

    pub fn r_cubic_to(
        &mut self,
        d1: impl Into<Vector>,
        d2: impl Into<Vector>,
        d3: impl Into<Vector>,
    ) -> &mut Self {
        let (d1, d2, d3) = (d1.into(), d2.into(), d3.into());
        unsafe {
            self.native_mut()
                .rCubicTo(d1.x, d1.y, d2.x, d2.y, d3.x, d3.y);
        }
        self
    }

    pub fn arc_to(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle: scalar,
        sweep_angle: scalar,
        force_move_to: bool,
    ) -> &mut Self {
        unsafe {
            self.native_mut().arcTo(
                oval.as_ref().native(),
                start_angle,
                sweep_angle,
                force_move_to,
            );
        }
        self
    }

    pub fn arc_to_tangent(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        radius: scalar,
    ) -> &mut Self {
        let (p1, p2) = (p1.into(), p2.into());
        unsafe {
            self.native_mut().arcTo1(p1.x, p1.y, p2.x, p2.y, radius);
        }
        self
    }

    pub fn arc_to_rotated(
        &mut self,
        r: impl Into<Point>,
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: PathDirection,
        xy: impl Into<Point>,
    ) -> &mut Self {
        let (r, xy) = (r.into(), xy.into());
        unsafe {
            self.native_mut()
                .arcTo2(r.x, r.y, x_axis_rotate, large_arc, sweep, xy.x, xy.y);
        }
        self
    }

    pub fn r_arc_to_rotated(
        &mut self,
        r: impl Into<Point>,
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: PathDirection,
        xy: impl Into<Point>,
    ) -> &mut Self {
        let (r, xy) = (r.into(), xy.into());
        unsafe {
            self.native_mut()
                .rArcTo(r.x, r.y, x_axis_rotate, large_arc, sweep, xy.x, xy.y);
        }
        self
    }

    pub fn close(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().close();
        }
        self
    }

    pub fn convert_conic_to_quads(
        p0: impl Into<Point>,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        w: scalar,
        pts: &mut [Point],
        pow2: usize,
    ) -> Option<usize> {
        let (p0, p1, p2) = (p0.into(), p1.into(), p2.into());
        let max_pts_count = 1 + 2 * (1 << pow2);
        if pts.len() >= max_pts_count {
            Some(unsafe {
                SkPath::ConvertConicToQuads(
                    p0.native(),
                    p1.native(),
                    p2.native(),
                    w,
                    pts.native_mut().as_mut_ptr(),
                    pow2.try_into().unwrap(),
                )
                .try_into()
                .unwrap()
            })
        } else {
            None
        }
    }

    // TODO: return type is probably worth a struct.
    pub fn is_rect(&self) -> Option<(Rect, bool, PathDirection)> {
        let mut rect = Rect::default();
        let mut is_closed = Default::default();
        let mut direction = PathDirection::default();
        unsafe {
            self.native()
                .isRect(rect.native_mut(), &mut is_closed, &mut direction)
        }
        .if_true_some((rect, is_closed, direction))
    }

    pub fn add_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        dir_start: Option<(PathDirection, usize)>,
    ) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut()
                .addRect(rect.as_ref().native(), dir, start.try_into().unwrap())
        };
        self
    }

    pub fn add_oval(
        &mut self,
        oval: impl AsRef<Rect>,
        dir_start: Option<(PathDirection, usize)>,
    ) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut()
                .addOval1(oval.as_ref().native(), dir, start.try_into().unwrap())
        };
        self
    }

    pub fn add_circle(
        &mut self,
        p: impl Into<Point>,
        radius: scalar,
        dir: impl Into<Option<PathDirection>>,
    ) -> &mut Self {
        let p = p.into();
        let dir = dir.into().unwrap_or_default();
        unsafe { self.native_mut().addCircle(p.x, p.y, radius, dir) };
        self
    }

    pub fn add_arc(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle: scalar,
        sweep_angle: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .addArc(oval.as_ref().native(), start_angle, sweep_angle)
        };
        self
    }

    // decided to only provide the simpler variant of the two, if radii needs to be specified,
    // add_rrect can be used.
    pub fn add_round_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        (rx, ry): (scalar, scalar),
        dir: impl Into<Option<PathDirection>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or_default();
        unsafe {
            self.native_mut()
                .addRoundRect(rect.as_ref().native(), rx, ry, dir)
        };
        self
    }

    pub fn add_rrect(
        &mut self,
        rrect: impl AsRef<RRect>,
        dir_start: Option<(PathDirection, usize)>,
    ) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut()
                .addRRect1(rrect.as_ref().native(), dir, start.try_into().unwrap())
        };
        self
    }

    pub fn add_poly(&mut self, pts: &[Point], close: bool) -> &mut Self {
        unsafe {
            self.native_mut()
                .addPoly(pts.native().as_ptr(), pts.len().try_into().unwrap(), close)
        };
        self
    }

    // TODO: addPoly(initializer_list)

    pub fn add_path(
        &mut self,
        src: &Path,
        d: impl Into<Vector>,
        mode: impl Into<Option<AddPathMode>>,
    ) -> &mut Self {
        let d = d.into();
        let mode = mode.into().unwrap_or(AddPathMode::Append);
        unsafe { self.native_mut().addPath(src.native(), d.x, d.y, mode) };
        self
    }

    // TODO: rename to add_path_with_matrix() ?
    pub fn add_path_matrix(
        &mut self,
        src: &Path,
        matrix: &Matrix,
        mode: impl Into<Option<AddPathMode>>,
    ) -> &mut Self {
        let mode = mode.into().unwrap_or(AddPathMode::Append);
        unsafe {
            self.native_mut()
                .addPath1(src.native(), matrix.native(), mode)
        };
        self
    }

    pub fn reverse_add_path(&mut self, src: &Path) -> &mut Self {
        unsafe { self.native_mut().reverseAddPath(src.native()) };
        self
    }

    #[must_use]
    pub fn with_offset(&self, d: impl Into<Vector>) -> Path {
        let d = d.into();
        let mut path = Path::default();
        unsafe { self.native().offset(d.x, d.y, path.native_mut()) };
        path
    }

    pub fn offset(&mut self, d: impl Into<Vector>) -> &mut Self {
        let d = d.into();
        let self_ptr = self.native_mut() as *mut _;
        unsafe { self.native().offset(d.x, d.y, self_ptr) };
        self
    }

    #[must_use]
    pub fn with_transform(&self, matrix: &Matrix) -> Path {
        self.with_transform_with_perspective_clip(matrix, ApplyPerspectiveClip::Yes)
    }

    pub fn with_transform_with_perspective_clip(
        &self,
        matrix: &Matrix,
        perspective_clip: ApplyPerspectiveClip,
    ) -> Path {
        let mut path = Path::default();
        unsafe {
            self.native()
                .transform(matrix.native(), path.native_mut(), perspective_clip)
        };
        path
    }

    pub fn transform(&mut self, matrix: &Matrix) -> &mut Self {
        self.transform_with_perspective_clip(matrix, ApplyPerspectiveClip::Yes)
    }

    pub fn transform_with_perspective_clip(
        &mut self,
        matrix: &Matrix,
        perspective_clip: ApplyPerspectiveClip,
    ) -> &mut Self {
        let self_ptr = self.native_mut() as *mut _;
        unsafe {
            self.native()
                .transform(matrix.native(), self_ptr, perspective_clip)
        };
        self
    }

    pub fn last_pt(&self) -> Option<Point> {
        let mut last_pt = Point::default();
        unsafe { self.native().getLastPt(last_pt.native_mut()) }.if_true_some(last_pt)
    }

    pub fn make_transform(
        &mut self,
        m: &Matrix,
        pc: impl Into<Option<ApplyPerspectiveClip>>,
    ) -> Path {
        self.with_transform_with_perspective_clip(
            &m,
            pc.into().unwrap_or(ApplyPerspectiveClip::Yes),
        )
    }

    pub fn make_scale(&mut self, (sx, sy): (scalar, scalar)) -> Path {
        self.make_transform(&Matrix::scale((sx, sy)), ApplyPerspectiveClip::No)
    }

    pub fn set_last_pt(&mut self, p: impl Into<Point>) -> &mut Self {
        let p = p.into();
        unsafe { self.native_mut().setLastPt(p.x, p.y) };
        self
    }

    pub fn segment_masks(&self) -> SegmentMask {
        SegmentMask::from_bits_truncate(unsafe { sb::C_SkPath_getSegmentMasks(self.native()) })
    }

    pub fn contains(&self, p: impl Into<Point>) -> bool {
        let p = p.into();
        unsafe { self.native().contains(p.x, p.y) }
    }

    pub fn dump_as_data(&self, force_close: bool, dump_as_hex: bool) -> Data {
        let mut stream = DynamicMemoryWStream::new();
        unsafe {
            self.native()
                .dump(stream.native_mut().base_mut(), force_close, dump_as_hex);
        }
        stream.detach_as_data()
    }

    pub fn dump(&self) {
        unsafe { self.native().dump1() }
    }

    pub fn dump_hex(&self) {
        unsafe { self.native().dumpHex() }
    }

    // TODO: writeToMemory()?

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkPath_serialize(self.native()) }).unwrap()
    }

    // TODO: readFromMemory()?

    pub fn deserialize(data: &Data) -> Option<Path> {
        let mut path = Path::default();
        let bytes = data.as_bytes();
        unsafe {
            path.native_mut()
                .readFromMemory(bytes.as_ptr() as _, bytes.len())
                > 0
        }
        .if_true_some(path)
    }

    pub fn generation_id(&self) -> u32 {
        unsafe { self.native().getGenerationID() }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_SkPath_isValid(self.native()) }
    }
}

#[test]
fn test_get_points() {
    let mut p = Path::new();
    p.add_rect(Rect::new(0.0, 0.0, 10.0, 10.0), None);
    let points_count = p.count_points();
    let mut points = vec![Point::default(); points_count];
    let count_returned = p.get_points(&mut points);
    assert_eq!(count_returned, points.len());
    assert_eq!(count_returned, 4);
}

#[test]
fn test_fill_type() {
    let mut p = Path::default();
    assert_eq!(p.fill_type(), PathFillType::Winding);
    p.set_fill_type(PathFillType::EvenOdd);
    assert_eq!(p.fill_type(), PathFillType::EvenOdd);
    assert!(!p.is_inverse_fill_type());
    p.toggle_inverse_fill_type();
    assert_eq!(p.fill_type(), PathFillType::InverseEvenOdd);
    assert!(p.is_inverse_fill_type());
}

#[test]
fn test_is_volatile() {
    let mut p = Path::default();
    assert!(!p.is_volatile());
    p.set_is_volatile(true);
    assert!(p.is_volatile());
}

#[test]
fn test_path_rect() {
    let r = Rect::new(0.0, 0.0, 100.0, 100.0);
    let path = Path::rect(r, None);
    assert_eq!(*path.bounds(), r);
}
