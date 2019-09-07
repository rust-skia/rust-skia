use crate::interop::DynamicMemoryWStream;
use crate::prelude::*;
use crate::{scalar, Data, Matrix, Point, RRect, Rect, Vector};
use skia_bindings::{
    C_SkPath_ConvertToNonInverseFillType, C_SkPath_Equals, C_SkPath_Iter_destruct,
    C_SkPath_Iter_isCloseLine, C_SkPath_Iter_next, C_SkPath_RawIter_Construct,
    C_SkPath_RawIter_destruct, C_SkPath_RawIter_next, C_SkPath_RawIter_peek, C_SkPath_destruct,
    C_SkPath_getBounds, C_SkPath_getConvexity, C_SkPath_getConvexityOrUnknown,
    C_SkPath_getFillType, C_SkPath_getSegmentMasks, C_SkPath_isEmpty, C_SkPath_isFinite,
    C_SkPath_isValid, C_SkPath_serialize, SkPath, SkPath_AddPathMode, SkPath_ArcSize,
    SkPath_Convexity, SkPath_Direction, SkPath_FillType, SkPath_Iter, SkPath_RawIter,
    SkPath_SegmentMask_kConic_SegmentMask, SkPath_SegmentMask_kCubic_SegmentMask,
    SkPath_SegmentMask_kLine_SegmentMask, SkPath_SegmentMask_kQuad_SegmentMask, SkPath_Verb,
};
use std::marker::PhantomData;
use std::mem::forget;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Direction {
    CW = SkPath_Direction::kCW_Direction as _,
    CCW = SkPath_Direction::kCCW_Direction as _,
}

impl NativeTransmutable<SkPath_Direction> for Direction {}
#[test]
fn test_direction_layout() {
    Direction::test_layout()
}

impl Default for Direction {
    fn default() -> Self {
        Direction::CW
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FillType {
    Winding = SkPath_FillType::kWinding_FillType as _,
    EvenOdd = SkPath_FillType::kEvenOdd_FillType as _,
    InverseWinding = SkPath_FillType::kInverseWinding_FillType as _,
    InverseEvenOdd = SkPath_FillType::kInverseEvenOdd_FillType as _,
}

impl NativeTransmutable<SkPath_FillType> for FillType {}
#[test]
fn test_fill_type_layout() {
    FillType::test_layout()
}

impl FillType {
    pub fn is_inverse(self) -> bool {
        (self as i32 & 2) != 0
    }

    pub fn to_non_inverse(self) -> Self {
        Self::from_native(unsafe { C_SkPath_ConvertToNonInverseFillType(self.into_native()) })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Convexity {
    Unknown = SkPath_Convexity::kUnknown_Convexity as _,
    Convex = SkPath_Convexity::kConvex_Convexity as _,
    Concave = SkPath_Convexity::kConcave_Convexity as _,
}

impl NativeTransmutable<SkPath_Convexity> for Convexity {}
#[test]
fn test_convexity_layout() {
    Convexity::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ArcSize {
    Small = SkPath_ArcSize::kSmall_ArcSize as _,
    Large = SkPath_ArcSize::kLarge_ArcSize as _,
}

impl NativeTransmutable<SkPath_ArcSize> for ArcSize {}
#[test]
fn test_arc_size_layout() {
    ArcSize::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum AddPathMode {
    Append = SkPath_AddPathMode::kAppend_AddPathMode as _,
    Extend = SkPath_AddPathMode::kExtend_AddPathMode as _,
}

impl NativeTransmutable<SkPath_AddPathMode> for AddPathMode {}
#[test]
fn test_add_path_mode_layout() {
    AddPathMode::test_layout()
}

bitflags! {
    pub struct SegmentMask: u32 {
        const LINE = SkPath_SegmentMask_kLine_SegmentMask as _;
        const QUAD = SkPath_SegmentMask_kQuad_SegmentMask as _;
        const CONIC = SkPath_SegmentMask_kConic_SegmentMask as _;
        const CUBIC = SkPath_SegmentMask_kCubic_SegmentMask as _;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Verb {
    Move = SkPath_Verb::kMove_Verb as _,
    Line = SkPath_Verb::kLine_Verb as _,
    Quad = SkPath_Verb::kQuad_Verb as _,
    Conic = SkPath_Verb::kConic_Verb as _,
    Qubic = SkPath_Verb::kCubic_Verb as _,
    Close = SkPath_Verb::kClose_Verb as _,
    Done = SkPath_Verb::kDone_Verb as _,
}

impl NativeTransmutable<SkPath_Verb> for Verb {}
#[test]
fn test_verb_layout() {
    Verb::test_layout()
}

impl Verb {
    /// The maximum number of points an interator will return for the verb.
    pub const MAX_POINTS: usize = 4;
    /// The number of points an iterator will return for the verb.
    pub fn points(self) -> usize {
        match self {
            Verb::Move => 1,
            Verb::Line => 2,
            Verb::Quad => 3,
            Verb::Conic => 4,
            Verb::Qubic => 4,
            Verb::Close => 0,
            Verb::Done => 0,
        }
    }
}

#[repr(C)]
pub struct Iter<'a>(SkPath_Iter, PhantomData<&'a Handle<SkPath>>);

impl<'a> NativeAccess<SkPath_Iter> for Iter<'a> {
    fn native(&self) -> &SkPath_Iter {
        &self.0
    }
    fn native_mut(&mut self) -> &mut SkPath_Iter {
        &mut self.0
    }
}

impl<'a> Drop for Iter<'a> {
    fn drop(&mut self) {
        unsafe { C_SkPath_Iter_destruct(&mut self.0) }
    }
}

impl<'a> Default for Iter<'a> {
    fn default() -> Self {
        Iter(unsafe { SkPath_Iter::new() }, PhantomData)
    }
}

impl<'a> Iter<'a> {
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

    pub fn next(
        &mut self,
        do_consume_generates: impl Into<Option<bool>>,
        exact: impl Into<Option<bool>>,
    ) -> (Verb, Vec<Point>) {
        let mut points = [Point::default(); Verb::MAX_POINTS];
        let verb = Verb::from_native(unsafe {
            C_SkPath_Iter_next(
                self.native_mut(),
                points.native_mut().as_mut_ptr(),
                do_consume_generates.into().unwrap_or(true),
                exact.into().unwrap_or(false),
            )
        });
        (verb, points[0..verb.points()].into())
    }

    pub fn conic_weight(&self) -> Option<scalar> {
        #[allow(clippy::map_clone)]
        self.native()
            .fConicWeights
            .into_option()
            .map(|p| unsafe { *p })
    }

    pub fn is_close_line(&self) -> bool {
        unsafe { C_SkPath_Iter_isCloseLine(self.native()) }
    }

    pub fn is_closed_contour(&self) -> bool {
        unsafe { self.native().isClosedContour() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Verb, Vec<Point>);

    fn next(&mut self) -> Option<Self::Item> {
        let (verb, points) = self.next(None, None);
        if verb != Verb::Done {
            Some((verb, points))
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct RawIter<'a>(SkPath_RawIter, PhantomData<&'a Handle<SkPath>>);

impl<'a> NativeAccess<SkPath_RawIter> for RawIter<'a> {
    fn native(&self) -> &SkPath_RawIter {
        &self.0
    }
    fn native_mut(&mut self) -> &mut SkPath_RawIter {
        &mut self.0
    }
}

impl<'a> Drop for RawIter<'a> {
    fn drop(&mut self) {
        unsafe { C_SkPath_RawIter_destruct(&mut self.0) }
    }
}

impl<'a> Default for RawIter<'a> {
    fn default() -> Self {
        RawIter(
            construct(|ri| unsafe { C_SkPath_RawIter_Construct(ri) }),
            PhantomData,
        )
    }
}

impl<'a> RawIter<'a> {
    pub fn new(path: &Path) -> RawIter {
        RawIter::default().set_path(path)
    }

    pub fn set_path(mut self, path: &Path) -> RawIter {
        unsafe { self.0.fRawIter.setPathRef(path.native().fPathRef.fPtr) };
        let r = RawIter(self.0, PhantomData);
        forget(self);
        r
    }

    pub fn peek(&self) -> Verb {
        Verb::from_native(unsafe { C_SkPath_RawIter_peek(self.native()) })
    }

    pub fn conic_weight(&self) -> Option<scalar> {
        #[allow(clippy::map_clone)]
        self.native()
            .fRawIter
            .fConicWeights
            .into_option()
            .map(|cw| unsafe { *cw })
    }
}

impl<'a> Iterator for RawIter<'a> {
    type Item = (Verb, Vec<Point>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut points = [Point::default(); Verb::MAX_POINTS];

        let verb = Verb::from_native(unsafe {
            C_SkPath_RawIter_next(self.native_mut(), points.native_mut().as_mut_ptr())
        });

        if verb != Verb::Done {
            Some((verb, points[0..verb.points()].into()))
        } else {
            None
        }
    }
}

pub type Path = Handle<SkPath>;

impl NativeDrop for SkPath {
    fn drop(&mut self) {
        unsafe { C_SkPath_destruct(self) }
    }
}

impl NativeClone for SkPath {
    fn clone(&self) -> Self {
        unsafe { SkPath::new1(self) }
    }
}

impl NativePartialEq for SkPath {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkPath_Equals(self, rhs) }
    }
}

impl Default for Handle<SkPath> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<SkPath> {
    pub fn new() -> Self {
        Self::from_native(unsafe { SkPath::new() })
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

    pub fn fill_type(&self) -> FillType {
        FillType::from_native(unsafe { C_SkPath_getFillType(self.native()) })
    }

    pub fn set_fill_type(&mut self, ft: FillType) -> &mut Self {
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

    pub fn convexity(&self) -> Convexity {
        Convexity::from_native(unsafe { C_SkPath_getConvexity(self.native()) })
    }

    pub fn convexity_or_unknown(&self) -> Convexity {
        Convexity::from_native(unsafe { C_SkPath_getConvexityOrUnknown(self.native()) })
    }

    pub fn set_convexity(&mut self, convexity: Convexity) -> &mut Self {
        unsafe { self.native_mut().setConvexity(convexity.into_native()) }
        self
    }

    pub fn is_convex(&self) -> bool {
        self.convexity() == Convexity::Convex
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
        unsafe { C_SkPath_isEmpty(self.native()) }
    }

    pub fn is_last_contour_closed(&self) -> bool {
        unsafe { self.native().isLastContourClosed() }
    }

    pub fn is_finite(&self) -> bool {
        unsafe { C_SkPath_isFinite(self.native()) }
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

    #[deprecated(note = "use get_point()")]
    pub fn point(&self, index: usize) -> Option<Point> {
        self.get_point(index)
    }

    pub fn get_point(&self, index: usize) -> Option<Point> {
        let p = Point::from_native(unsafe { self.native().getPoint(index.try_into().unwrap()) });
        // assuming that count_points() is somewhat slow, we
        // check the index when a Point(0,0) is returned.
        if p != Point::default() || index < self.count_points() {
            Some(p)
        } else {
            None
        }
    }

    #[deprecated(note = "use get_points()")]
    pub fn points(&self, points: &mut [Point]) -> usize {
        self.get_points(points)
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

    #[deprecated(note = "use get_verbs()")]
    pub fn verbs(&self, verbs: &mut [u8]) -> usize {
        self.get_verbs(verbs)
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
        Rect::from_native_ref(unsafe { &*C_SkPath_getBounds(self.native()) })
    }

    pub fn update_bounds_cache(&mut self) -> &mut Self {
        self.bounds();
        self
    }

    pub fn compute_tight_bounds(&self) -> Rect {
        Rect::from_native(unsafe { self.native().computeTightBounds() })
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

    pub fn shrink_to_fit(&mut self) -> &mut Self {
        unsafe { self.native_mut().shrinkToFit() }
        self
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
        unsafe { self.native_mut().quadTo(p1.x, p1.y, p2.x, p2.y) };
        self
    }

    pub fn r_quad_to(&mut self, dx1: impl Into<Vector>, dx2: impl Into<Vector>) -> &mut Self {
        let (dx1, dx2) = (dx1.into(), dx2.into());
        unsafe { self.native_mut().rQuadTo(dx1.x, dx1.y, dx2.x, dx2.y) };
        self
    }

    pub fn conic_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>, w: scalar) -> &mut Self {
        let p1 = p1.into();
        let p2 = p2.into();
        unsafe { self.native_mut().conicTo(p1.x, p1.y, p2.x, p2.y, w) };
        self
    }

    pub fn r_conic_to(
        &mut self,
        d1: impl Into<Vector>,
        d2: impl Into<Vector>,
        w: scalar,
    ) -> &mut Self {
        let (d1, d2) = (d1.into(), d2.into());
        unsafe { self.native_mut().rConicTo(d1.x, d1.y, d2.x, d2.y, w) };
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
                .cubicTo(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y)
        };
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
                .rCubicTo(d1.x, d1.y, d2.x, d2.y, d3.x, d3.y)
        };
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
            )
        };
        self
    }

    pub fn arc_to_tangent(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        radius: scalar,
    ) -> &mut Self {
        let (p1, p2) = (p1.into(), p2.into());
        unsafe { self.native_mut().arcTo1(p1.x, p1.y, p2.x, p2.y, radius) };
        self
    }

    pub fn arc_to_rotated(
        &mut self,
        r: impl Into<Point>,
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: Direction,
        xy: impl Into<Point>,
    ) -> &mut Self {
        let (r, xy) = (r.into(), xy.into());
        unsafe {
            self.native_mut().arcTo2(
                r.x,
                r.y,
                x_axis_rotate,
                large_arc.into_native(),
                sweep.into_native(),
                xy.x,
                xy.y,
            )
        };
        self
    }

    pub fn r_arc_to_rotated(
        &mut self,
        r: impl Into<Point>,
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: Direction,
        xy: impl Into<Point>,
    ) -> &mut Self {
        let (r, xy) = (r.into(), xy.into());
        unsafe {
            self.native_mut().rArcTo(
                r.x,
                r.y,
                x_axis_rotate,
                large_arc.into_native(),
                sweep.into_native(),
                xy.x,
                xy.y,
            )
        };
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
    pub fn is_rect(&self) -> Option<(Rect, bool, Direction)> {
        let mut rect = Rect::default();
        let mut is_closed = Default::default();
        let mut direction = Direction::default();
        unsafe {
            self.native()
                .isRect(rect.native_mut(), &mut is_closed, direction.native_mut())
        }
        .if_true_some((rect, is_closed, direction))
    }

    // TODO: return type is probably worth a struct.
    pub fn is_nested_fill_rects(&self) -> Option<([Rect; 2], [Direction; 2])> {
        let mut rects = [Rect::default(); 2];
        let mut dirs = [Direction::CW; 2];
        unsafe {
            self.native().isNestedFillRects(
                rects.native_mut().as_mut_ptr(),
                dirs.native_mut().as_mut_ptr(),
            )
        }
        .if_true_some((rects, dirs))
    }

    pub fn add_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        dir_start: Option<(Direction, usize)>,
    ) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut().addRect1(
                rect.as_ref().native(),
                dir.into_native(),
                start.try_into().unwrap(),
            )
        };
        self
    }

    pub fn add_oval(
        &mut self,
        oval: impl AsRef<Rect>,
        dir_start: Option<(Direction, usize)>,
    ) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut().addOval1(
                oval.as_ref().native(),
                dir.into_native(),
                start.try_into().unwrap(),
            )
        };
        self
    }

    pub fn add_circle(
        &mut self,
        p: impl Into<Point>,
        radius: scalar,
        dir: impl Into<Option<Direction>>,
    ) -> &mut Self {
        let p = p.into();
        let dir = dir.into().unwrap_or_default();
        unsafe {
            self.native_mut()
                .addCircle(p.x, p.y, radius, dir.into_native())
        };
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
        dir: impl Into<Option<Direction>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or_default();
        unsafe {
            self.native_mut()
                .addRoundRect(rect.as_ref().native(), rx, ry, dir.into_native())
        };
        self
    }

    pub fn add_rrect(
        &mut self,
        rrect: impl AsRef<RRect>,
        dir_start: Option<(Direction, usize)>,
    ) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut().addRRect1(
                rrect.as_ref().native(),
                dir.into_native(),
                start.try_into().unwrap(),
            )
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
        unsafe {
            self.native_mut()
                .addPath(src.native(), d.x, d.y, mode.into_native())
        };
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
                .addPath1(src.native(), matrix.native(), mode.into_native())
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
        let mut path = Path::default();
        unsafe { self.native().transform(matrix.native(), path.native_mut()) };
        path
    }

    pub fn transform(&mut self, matrix: &Matrix) -> &mut Self {
        let self_ptr = self.native_mut() as *mut _;
        unsafe { self.native().transform(matrix.native(), self_ptr) };
        self
    }

    pub fn last_pt(&self) -> Option<Point> {
        let mut last_pt = Point::default();
        unsafe { self.native().getLastPt(last_pt.native_mut()) }.if_true_some(last_pt)
    }

    pub fn set_last_pt(&mut self, p: impl Into<Point>) -> &mut Self {
        let p = p.into();
        unsafe { self.native_mut().setLastPt(p.x, p.y) };
        self
    }

    pub fn segment_masks(&self) -> SegmentMask {
        SegmentMask::from_bits_truncate(unsafe { C_SkPath_getSegmentMasks(self.native()) })
    }

    pub fn contains(&self, p: impl Into<Point>) -> bool {
        let p = p.into();
        unsafe { self.native().contains(p.x, p.y) }
    }

    pub fn dump_as_data(&self, force_close: bool, dump_as_hex: bool) -> Data {
        let mut stream = DynamicMemoryWStream::new();
        unsafe {
            self.native()
                .dump(&mut stream.native_mut()._base, force_close, dump_as_hex);
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
        Data::from_ptr(unsafe { C_SkPath_serialize(self.native()) }).unwrap()
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
        unsafe { C_SkPath_isValid(self.native()) }
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
fn fill_type() {
    let mut p = Path::default();
    assert_eq!(p.fill_type(), FillType::Winding);
    p.set_fill_type(FillType::EvenOdd);
    assert_eq!(p.fill_type(), FillType::EvenOdd);
    assert!(!p.is_inverse_fill_type());
    p.toggle_inverse_fill_type();
    assert_eq!(p.fill_type(), FillType::InverseEvenOdd);
    assert!(p.is_inverse_fill_type());
}

#[test]
fn is_volatile() {
    let mut p = Path::default();
    assert!(!p.is_volatile());
    p.set_is_volatile(true);
    assert!(p.is_volatile());
}
