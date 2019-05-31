use crate::prelude::*;
use crate::core::{
    scalar,
    ISize,
    IPoint,
    Point,
    Size,
    IVector,
    Contains,
    Vector
};
use skia_bindings::{SkIRect, SkRect, SkIRect_MakeXYWH};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32
}

impl NativeTransmutable<SkIRect> for IRect {}

#[test]
fn test_irect_layout() {
    IRect::test_layout();
}

impl AsRef<IRect> for IRect {
    fn as_ref(&self) -> &IRect {
        self
    }
}

impl IRect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn new_empty() -> Self {
        Self::default()
    }

    pub fn from_wh(w: i32, h: i32) -> Self {
        Self::from_size((w, h))
    }

    pub fn from_size(size: impl Into<ISize>) -> Self {
        let size = size.into();
        Self::new(0, 0, size.width, size.height)
    }

    pub fn from_ltrb(l: i32, t: i32, r: i32, b: i32) -> Self {
        Self::new(l, t, r, b)
    }

    pub fn from_xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_native(unsafe { SkIRect_MakeXYWH(x, y, w, h) })
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn right(&self) -> i32 {
        self.right
    }

    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    pub fn x(&self) -> i32 {
        self.left
    }

    pub fn y(&self) -> i32 {
        self.top
    }

    #[deprecated(since = "0.11.0", note = "will be removed without replacement")]
    pub fn left_top(&self) -> IPoint {
        (self.left, self.top).into()
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn size(&self) -> ISize {
        (self.width(), self.height()).into()
    }

    pub fn width_64(&self) -> i64 {
        unsafe { self.native().width64() }
    }

    pub fn height_64(&self) -> i64 {
        unsafe { self.native().height64() }
    }

    pub fn is_empty_64(&self) -> bool {
        unsafe { self.native().isEmpty64() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    pub fn set_empty(&mut self) {
        *self = Self::new_empty()
    }

    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        *self = Self::new(left, top, right, bottom)
    }

    pub fn set_ltrb(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.set(left, top, right, bottom)
    }

    pub fn set_xywh(&mut self, x: i32, y: i32, w: i32, h: i32) {
        *self = Self::from_xywh(x, y, w, h);
    }

    #[must_use]
    pub fn with_offset(&self, delta: impl Into<IVector>) -> Self {
        let delta = delta.into();
        let copied = *self;
        Self::from_native(unsafe {
            copied.native().makeOffset(delta.x, delta.y)
        })
    }

    #[must_use]
    pub fn with_inset(&self, delta: impl Into<IVector>) -> Self {
        /* does not link:
        Self::from_native(unsafe {
            cloned.native().makeInset(delta.x, delta.y)
        }) */
        self.with_outset(-delta.into())
    }

    #[must_use]
    pub fn with_outset(&self, delta: impl Into<IVector>) -> Self {
        let delta = delta.into();
        Self::from_native(unsafe {
            self.native().makeOutset(delta.x, delta.y)
        })
    }

    pub fn offset(&mut self, delta: impl Into<IPoint>) {
        let delta = delta.into();
        unsafe { self.native_mut().offset1(delta.native()) }
    }

    pub fn offset_to(&mut self, new_p: impl Into<IPoint>) {
        *self = self.with_offset_to(new_p)
    }

    #[must_use]
    pub fn with_offset_to(&self, new_p: impl Into<IPoint>) -> Self {
        let new_p = new_p.into();
        let mut copied = *self;
        unsafe {
            copied.native_mut().offsetTo(new_p.x, new_p.y)
        }
        copied
    }

    pub fn inset(&mut self, delta: impl Into<IVector>) {
        *self = self.with_inset(delta)
    }

    pub fn outset(&mut self, delta: impl Into<IVector>) {
        *self = self.with_outset(delta)
    }

    #[must_use]
    pub fn with_adjustment(&self, d_l: i32, d_t: i32, d_r: i32, d_b: i32) -> Self {
        let mut copied = *self;
        unsafe {
            copied.native_mut().adjust(d_l, d_t, d_r, d_b)
        }
        copied
    }

    pub fn adjust(&mut self, d_l: i32, d_t: i32, d_r: i32, d_b: i32) {
        *self = self.with_adjustment(d_l, d_t, d_r, d_b)
    }

    // contains() is implemented through a trait below.

    pub fn contains_no_empty_check(&self, r: &Self) -> bool {
        unsafe { self.native().containsNoEmptyCheck1(r.native()) }
    }

    pub fn intersect(a: &Self, b: &Self) -> Option<Self> {
        let mut intersection = Self::default();
        unsafe { intersection.native_mut().intersect1(a.native(), b.native())}
            .if_true_some(intersection)
    }

    pub fn intersect_no_empty_check(a: &Self, b: &Self) -> Option<Self> {
        let mut intersection = Self::default();
        unsafe { intersection.native_mut().intersectNoEmptyCheck(a.native(), b.native())}
            .if_true_some(intersection)
    }

    pub fn intersects(a: &Self, b: &Self) -> bool {
        unsafe { SkIRect::Intersects(a.native(), b.native()) }
    }

    pub fn intersects_no_empty_check(a: &Self, b: &Self) -> bool {
        unsafe { SkIRect::IntersectsNoEmptyCheck(a.native(), b.native()) }
    }

    pub fn join(a: &Self, b: &Self) -> Self {
        let mut copied = *a;
        unsafe { copied.native_mut().join1(b.native()) }
        copied
    }

    pub fn sort(&mut self) {
        *self = self.sorted()
    }

    #[must_use]
    pub fn sorted(&self) -> Self {
        let mut copied = *self;
        // makeSorted does not link:
        // IRect::from_native(unsafe { self.native().makeSorted() })
        unsafe { copied.native_mut().sort() }
        copied
    }

    #[must_use]
    pub fn empty() -> &'static Self {
        &EMPTY
    }
}

lazy_static! {
    static ref EMPTY: IRect = IRect::default();
}

impl Contains<IPoint> for IRect {
    fn contains(&self, other: IPoint) -> bool {
        unsafe { self.native().contains(other.x, other.y) }
    }
}

impl Contains<&IRect> for IRect {
    fn contains(&self, other: &IRect) -> bool {
        unsafe { self.native().contains2(other.native()) }
    }
}

impl Contains<&Rect> for IRect {
    // TODO: can AsRef<Rect> supported here?
    fn contains(&self, other: &Rect) -> bool {
        unsafe { self.native().contains3(other.native()) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    pub left: scalar,
    pub top: scalar,
    pub right: scalar,
    pub bottom: scalar
}

impl NativeTransmutable<SkRect> for Rect {}

#[test]
fn test_rect_layout() {
    Rect::test_layout();
}

impl AsRef<Rect> for Rect {
    fn as_ref(&self) -> &Rect {
        self
    }
}

impl Rect {
    pub fn new(left: scalar, top: scalar, right: scalar, bottom: scalar) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn from_size<S: Into<Size>>(size: S) -> Self {
        (Point::default(), size.into()).into()
    }

    // replacement for new_xywh
    pub fn from_point_and_size<P: Into<Point>, S: Into<Size>>(p: P, sz: S) -> Self {
        (p.into(), sz.into()).into()
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    pub fn is_sorted(&self) -> bool {
        unsafe { self.native().isSorted() }
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn left_top(&self) -> Point {
        (self.left, self.top).into()
    }

    pub fn x(&self) -> scalar {
        self.left
    }

    pub fn y(&self) -> scalar {
        self.top
    }

    pub fn size(&self) -> Size {
        (self.width(), self.height()).into()
    }

    pub fn width(&self) -> scalar {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> scalar {
        unsafe { self.native().height() }
    }

    pub fn center(&self) -> Point {
        (self.center_x(), self.center_y()).into()
    }

    pub fn center_x(&self) -> scalar {
        unsafe {self.native().centerX() }
    }

    pub fn center_y(&self) -> scalar {
        unsafe {self.native().centerY() }
    }

    pub fn to_quad(&self) -> [Point; 4] {
        let mut quad = [Point::default(); 4];
        unsafe { self.native().toQuad(quad.native_mut().as_mut_ptr()) }
        quad
    }

    pub fn from_bounds(points: &[Point]) -> Option<Self> {
        let mut r = Self::default();
        unsafe { r.native_mut().setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap()) }
            .if_true_some(r)
    }

    #[must_use]
    pub fn with_offset<V: Into<Vector>>(&self, d: V) -> Self {
        let d = d.into();
        Self::new(self.left + d.x, self.top + d.y, self.right + d.x, self.bottom + d.y)
    }

    #[must_use]
    pub fn with_inset<V: Into<Vector>>(&self, d: V) -> Self {
        let d = d.into();
        Self::new(self.left + d.x, self.top + d.y, self.right - d.x, self.bottom - d.y)
    }

    #[must_use]
    pub fn with_outset<V: Into<Vector>>(&self, d: V) -> Self {
        let d = d.into();
        Self::new(self.left - d.x, self.top - d.y, self.right + d.x, self.bottom + d.y)
    }

    #[must_use]
    pub fn with_offset_to<P: Into<Point>>(&self, new_p: P) -> Self {
        let new_p = new_p.into();
        // does not link:
        // let mut r = self.clone();
        // unsafe { r.native_mut().offsetTo(new_x, new_y) };
        Self::new(new_p.x, new_p.y, new_p.x - self.left, new_p.y - self.top)
    }

    pub fn intersect<A: AsRef<Rect>, B: AsRef<Rect>>(a: A, b: B) -> Option<Rect> {
        let mut intersection = Rect::default();
        unsafe { intersection.native_mut().intersect2(a.as_ref().native(), b.as_ref().native()) }
            .if_true_some(intersection)
    }

    pub fn intersects<A: AsRef<Rect>, B: AsRef<Rect>>(a: &Rect, b: &Rect) -> bool {
        unsafe { SkRect::Intersects(a.as_ref().native(), b.as_ref().native()) }
    }

    pub fn join<A: AsRef<Rect>, B: AsRef<Rect>>(a: A, b: B) -> Rect {
        let mut joined = *(a.as_ref());
        unsafe { joined.native_mut().join1(b.as_ref().native()) }
        joined
    }

    #[must_use]
    pub fn round(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().round(r.native_mut()) };
        r
    }

    #[must_use]
    pub fn round_in(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().roundIn(r.native_mut()) };
        r
    }

    #[must_use]
    pub fn sorted(&self) -> Rect {
        Rect::from_native(unsafe { self.native().makeSorted() })
    }

    pub fn as_scalars(&self) -> &[scalar; 4] {
        unsafe { transmute_ref(self) }
    }
}

impl Contains<Point> for Rect {
    fn contains(&self, other: Point) -> bool {
        // does not link:
        // unsafe { self.native().contains(other.x, other.y) }
        other.x >= self.left &&
        other.x < self.right &&
        other.y >= self.top &&
        other.y < self.bottom
    }
}

impl Contains<Rect> for Rect {
    fn contains(&self, other: Rect) -> bool {
        unsafe { self.native().contains1(other.native()) }
    }
}

impl Contains<IRect> for Rect {
    fn contains(&self, other: IRect) -> bool {
        unsafe { self.native().contains2(other.native()) }
    }
}

#[test]
fn contains_overloads_compile() {
    let r = Rect::default();
    r.contains(Point::default());
    r.contains(Rect::default());
    r.contains(IRect::default());
}

pub trait RoundOut<R> {
    fn round_out(&self) -> R;
}

impl RoundOut<IRect> for Rect {
    fn round_out(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().roundOut(r.native_mut()) };
        r
    }
}

impl RoundOut<Rect> for Rect {
    fn round_out(&self) -> Rect {
        let mut r = Rect::default();
        unsafe { self.native().roundOut1(r.native_mut()) };
        r
    }
}

//
// From
//

impl From<(IPoint, ISize)> for IRect {
    fn from((point, size): (IPoint, ISize)) -> Self {
        IRect::new(point.x, point.y, point.x + size.width, point.y + size.height)
    }
}

impl From<(Point, Size)> for Rect {
    fn from((point, size): (Point, Size)) -> Self {
        Rect::new(point.x, point.y, point.x + size.width, point.y + size.height)
    }
}

impl From<IRect> for Rect {
    fn from(source: IRect) -> Self {
        Self::new(source.left as _, source.top as _, source.right as _, source.bottom as _)
    }
}
