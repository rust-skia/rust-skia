use crate::prelude::*;
use crate::{
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
        &EMPTY_IRECT
    }
}

lazy_static! {
    static ref EMPTY_IRECT: IRect = IRect::default();
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

    pub fn new_empty() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn from_wh(w: scalar, h: scalar) -> Self {
        Self::new(0.0, 0.0, w, h)
    }

    pub fn from_iwh(w: i32, h: i32) -> Self {
        Self::from_wh(w as scalar, h as scalar)
    }

    pub fn from_size(size: impl Into<Size>) -> Self {
        (Point::default(), size.into()).into()
    }

    pub fn from_xywh(x: scalar, y: scalar, w: scalar, h: scalar) -> Self {
        Self::new(x, y, x + w, y + h)
    }

    pub fn from_point_and_size(p: impl Into<Point>, sz: impl Into<Size>) -> Self {
        (p.into(), sz.into()).into()
    }

    pub fn from_isize(isize: impl Into<ISize>) -> Self {
        let isize = isize.into();
        Self::from_iwh(isize.width, isize.height)
    }

    pub fn from_irect(irect: impl AsRef<IRect>) -> Self {
        let irect = irect.as_ref();
        Self::new(irect.left as scalar, irect.top as scalar, irect.right as scalar, irect.bottom as scalar)
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

    #[deprecated(since = "0.11.0", note = "removed without replacement")]
    pub fn left_top(&self) -> Point {
        (self.left, self.top).into()
    }

    pub fn x(&self) -> scalar {
        self.left
    }

    pub fn y(&self) -> scalar {
        self.top
    }

    pub fn left(&self) -> scalar {
        self.left
    }

    pub fn top(&self) -> scalar {
        self.top
    }

    pub fn right(&self) -> scalar {
        self.right
    }

    pub fn bottom(&self) -> scalar {
        self.bottom
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
        Point::from((self.center_x(), self.center_y()))
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

    pub fn set_empty(&mut self) {
        *self = Self::new_empty()
    }

    pub fn set_irect(&mut self, irect: impl AsRef<IRect>) {
        *self = Self::from_irect(irect)
    }

    pub fn set(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) {
        *self = Self::new(left, top, right, bottom)
    }

    pub fn set_ltrb(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) {
        self.set(left, top, right, bottom)
    }

    pub fn iset(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        *self = Self::from_irect(IRect::new(left, top, right, bottom))
    }

    pub fn iset_wh(&mut self, width: i32, height: i32) {
        *self = Self::from_isize(ISize::new(width, height))
    }

    pub fn set_bounds(&mut self, points: &[Point]) {
        unsafe { self.native_mut().setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap()); }
    }

    pub fn set_bounds_check(&mut self, points: &[Point]) -> bool {
        unsafe { self.native_mut().setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap()) }
    }

    pub fn set_bounds_no_check(&mut self, points: &[Point]) {
        unsafe { self.native_mut().setBoundsNoCheck(points.native().as_ptr(), points.len().try_into().unwrap()) }
    }

    pub fn set_bounds2(&mut self, p0: impl Into<Point>, p1: impl Into<Point>) {
        unsafe { self.native_mut().set3(p0.into().native(), p1.into().native()) }
    }

    pub fn from_bounds(points: &[Point]) -> Option<Self> {
        let mut r = Self::default();
        unsafe { r.native_mut().setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap()) }
            .if_true_some(r)
    }

    pub fn set_xywh(&mut self, x: scalar, y: scalar, width: scalar, height: scalar) {
        *self = Self::from_xywh(x, y, width, height)
    }

    pub fn set_wh(&mut self, w: scalar, h: scalar) {
        *self = Self::from_wh(w, h)
    }

    pub fn with_offset(&self, d: impl Into<Vector>) -> Self {
        let d = d.into();
        Self::new(self.left + d.x, self.top + d.y, self.right + d.x, self.bottom + d.y)
    }

    pub fn with_inset(&self, d: impl Into<Vector>) -> Self {
        let d = d.into();
        Self::new(self.left + d.x, self.top + d.y, self.right - d.x, self.bottom - d.y)
    }

    pub fn with_outset(&self, d: impl Into<Vector>) -> Self {
        let d = d.into();
        Self::new(self.left - d.x, self.top - d.y, self.right + d.x, self.bottom + d.y)
    }

    pub fn offset(&mut self, d: impl Into<Vector>) {
        *self = self.with_offset(d)
    }

    pub fn offset_to(&mut self, new_p: impl Into<Point>) {
        *self = self.with_offset_to(new_p)
    }

    pub fn with_offset_to(&self, new_p: impl Into<Point>) -> Self {
        let new_p = new_p.into();
        // does not link:
        // let mut r = self.clone();
        // unsafe { r.native_mut().offsetTo(new_x, new_y) };
        Self::new(new_p.x, new_p.y, new_p.x - self.left, new_p.y - self.top)
    }

    pub fn inset(&mut self, d: impl Into<Vector>) {
        *self = self.with_inset(d)
    }

    pub fn outset(&mut self, d: impl Into<Vector>) {
        *self = self.with_outset(d)
    }

    pub fn intersect(&mut self, r: impl AsRef<Rect>) -> bool {
        unsafe {
            self.native_mut().intersect(r.as_ref().native())
        }
    }

    pub fn intersect_ltrb(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) -> bool {
        unsafe {
            self.native_mut().intersect1(left, top, right, bottom)
        }
    }

    #[must_use]
    pub fn intersect2(&mut self, a: impl AsRef<Rect>, b: impl AsRef<Rect>) -> bool
    {
        unsafe { self.native_mut().intersect2(a.as_ref().native(), b.as_ref().native()) }
    }

    pub fn intersects_ltrb(&self, left: scalar, top: scalar, right: scalar, bottom: scalar) -> bool {
        // does not link:
        // unsafe { self.native().intersects(left, top, right, bottom) }
        self.intersects(Rect::new(left, top, right, bottom))
    }

    pub fn intersects(&self, r: impl AsRef<Rect>) -> bool {
        unsafe { self.native().intersects1(r.as_ref().native()) }
    }

    pub fn intersects2(a: impl AsRef<Rect>, b: impl AsRef<Rect>) -> bool {
        unsafe { SkRect::Intersects(a.as_ref().native(), b.as_ref().native()) }
    }

    pub fn join_ltrb(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) {
        unsafe { self.native_mut().join(left, top, right, bottom) }
    }

    pub fn join(&mut self, r: impl AsRef<Rect>) {
        unsafe { self.native_mut().join1(r.as_ref().native()) }
    }

    pub fn join2(a: impl AsRef<Rect>, b: impl AsRef<Rect>) -> Rect {
        let mut result = *a.as_ref();
        result.join(b);
        result
    }

    pub fn join_non_empty_arg(&mut self, r: impl AsRef<Rect>) {
        unsafe { self.native_mut().joinNonEmptyArg(r.as_ref().native()) }
    }

    pub fn join_possibly_empty_rect(&mut self, r: impl AsRef<Rect>) {
        unsafe { self.native_mut().joinPossiblyEmptyRect(r.as_ref().native()) }
    }

    // The set of contains() functions are defined as a trait below.

    #[must_use]
    pub fn round(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().round(r.native_mut()) };
        r
    }

    // The functions round_out() are defined as a trait below.

    #[must_use]
    pub fn round_in(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().roundIn(r.native_mut()) };
        r
    }

    pub fn sort(&mut self) {
        unsafe { self.native_mut().sort() }
    }

    #[must_use]
    pub fn sorted(&self) -> Rect {
        Rect::from_native(unsafe { self.native().makeSorted() })
    }

    pub fn as_scalars(&self) -> &[scalar; 4] {
        unsafe { transmute_ref(&self.left) }
    }

    pub fn dump(&self, as_hex: impl Into<Option<bool>>) {
        unsafe { self.native().dump(as_hex.into().unwrap_or_default()) }
    }

    pub fn dump_hex(&self) {
        self.dump(true)
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

impl From<ISize> for Rect {
    fn from(isize: ISize) -> Self {
        Self::from_isize(isize)
    }
}
impl From<IRect> for Rect {
    fn from(irect: IRect) -> Self {
        Self::from_irect(irect)
    }
}
