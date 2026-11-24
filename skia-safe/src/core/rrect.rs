//! Describes a rounded rectangle with a bounds and a pair of radii for each corner. The bounds
//! and radii can be set so that [`RRect`] describes: a rectangle with sharp corners; a circle; an
//! oval; or a rectangle with one or more rounded corners.
//!
//! [`RRect`] allows implementing CSS properties that describe rounded corners. [`RRect`] may have
//! up to eight different radii, one for each axis on each of its four corners.
//!
//! [`RRect`] may modify the provided parameters when initializing bounds and radii. If either axis
//! radii is zero or less: radii are stored as zero; corner is square. If corner curves overlap,
//! radii are proportionally reduced to fit within bounds.

use crate::{Matrix, Point, Rect, Vector, interop, prelude::*, scalar};
use skia_bindings::{self as sb, SkRRect};
use std::{fmt, mem, ptr};

/// Describes possible specializations of [`RRect`]. Each type is exclusive; an [`RRect`] may only
/// have one type.
///
/// Type members become progressively less restrictive; larger values of type have more degrees of
/// freedom than smaller values.
pub use skia_bindings::SkRRect_Type as Type;
variant_name!(Type::Complex);

/// The radii are stored: top-left, top-right, bottom-right, bottom-left.
pub use skia_bindings::SkRRect_Corner as Corner;
variant_name!(Corner::LowerLeft);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RRect(SkRRect);

native_transmutable!(SkRRect, RRect);

impl PartialEq for RRect {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkRRect_Equals(self.native(), rhs.native()) }
    }
}

impl Default for RRect {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for RRect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RRect")
            .field("rect", &self.rect())
            .field(
                "radii",
                &[
                    self.radii(Corner::UpperLeft),
                    self.radii(Corner::UpperRight),
                    self.radii(Corner::LowerRight),
                    self.radii(Corner::LowerLeft),
                ],
            )
            .field("type", &self.get_type())
            .finish()
    }
}

impl AsRef<RRect> for RRect {
    fn as_ref(&self) -> &RRect {
        self
    }
}

impl RRect {
    /// Initializes bounds at (0, 0), the origin, with zero width and height. Initializes corner
    /// radii to (0, 0), and sets the type to [`Type::Empty`].
    pub fn new() -> Self {
        RRect::construct(|rr| unsafe { sb::C_SkRRect_Construct(rr) })
    }

    pub fn get_type(&self) -> Type {
        unsafe { sb::C_SkRRect_getType(self.native()) }
    }

    pub fn is_empty(&self) -> bool {
        self.get_type() == Type::Empty
    }

    pub fn is_rect(&self) -> bool {
        self.get_type() == Type::Rect
    }

    pub fn is_oval(&self) -> bool {
        self.get_type() == Type::Oval
    }

    pub fn is_simple(&self) -> bool {
        self.get_type() == Type::Simple
    }

    pub fn is_nine_patch(&self) -> bool {
        self.get_type() == Type::NinePatch
    }

    pub fn is_complex(&self) -> bool {
        self.get_type() == Type::Complex
    }

    /// Returns the span on the x-axis. This does not check if the result fits in a 32-bit float;
    /// the result may be infinity.
    pub fn width(&self) -> scalar {
        self.rect().width()
    }

    /// Returns the span on the y-axis. This does not check if the result fits in a 32-bit float;
    /// the result may be infinity.
    pub fn height(&self) -> scalar {
        self.rect().height()
    }

    /// Returns the top-left corner radii. If the type is [`Type::Empty`], [`Type::Rect`],
    /// [`Type::Oval`], or [`Type::Simple`], returns a value representative of all corner radii. If
    /// the type is [`Type::NinePatch`] or [`Type::Complex`], at least one of the remaining three
    /// corners has a different value.
    pub fn simple_radii(&self) -> Vector {
        self.radii(Corner::UpperLeft)
    }

    /// Sets bounds to zero width and height at (0, 0), the origin. Sets corner radii to zero and
    /// sets the type to [`Type::Empty`].
    pub fn set_empty(&mut self) {
        *self = Self::new()
    }

    /// Sets bounds to sorted `rect`, and sets corner radii to zero. If the set bounds has width and
    /// height, sets the type to [`Type::Rect`]; otherwise, sets the type to [`Type::Empty`].
    ///
    /// - `rect` bounds to set
    pub fn set_rect(&mut self, rect: impl AsRef<Rect>) {
        unsafe { sb::C_SkRRect_setRect(self.native_mut(), rect.as_ref().native()) }
    }

    /// Initializes bounds at (0, 0), the origin, with zero width and height. Initializes corner
    /// radii to (0, 0), and sets the type to [`Type::Empty`].
    pub fn new_empty() -> Self {
        Self::new()
    }

    // TODO: consider to rename all the following new_* function to from_* functions?
    //       is it possible to find a proper convention here (new_ vs from_?)?

    /// Initializes to a copy of `rect` bounds and zeroes corner radii.
    ///
    /// - `rect` bounds to copy
    pub fn new_rect(rect: impl AsRef<Rect>) -> Self {
        let mut rr = Self::default();
        rr.set_rect(rect);
        rr
    }

    /// Initializes to an oval, x-axis radii to half `oval.width()`, and all y-axis radii to half
    /// `oval.height()`. If the oval bounds is empty, sets the type to [`Type::Empty`]. Otherwise,
    /// sets the type to [`Type::Oval`].
    ///
    /// - `oval` bounds of oval
    pub fn new_oval(oval: impl AsRef<Rect>) -> Self {
        let mut rr = Self::default();
        rr.set_oval(oval);
        rr
    }

    /// Initializes to a rounded rectangle with the same radii for all four corners. If `rect` is
    /// empty, sets the type to [`Type::Empty`]. Otherwise, if `x_rad` and `y_rad` are zero, sets
    /// the type to [`Type::Rect`]. Otherwise, if `x_rad` is at least half `rect.width()` and
    /// `y_rad` is at least half `rect.height()`, sets the type to [`Type::Oval`]. Otherwise, sets
    /// the type to [`Type::Simple`].
    ///
    /// - `rect` bounds of rounded rectangle
    /// - `x_rad` x-axis radius of corners
    /// - `y_rad` y-axis radius of corners
    pub fn new_rect_xy(rect: impl AsRef<Rect>, x_rad: scalar, y_rad: scalar) -> Self {
        let mut rr = Self::default();
        rr.set_rect_xy(rect.as_ref(), x_rad, y_rad);
        rr
    }

    /// Initializes to a rounded rectangle with a radii array for individual control of all four
    /// corners.
    ///
    /// If `rect` is empty, sets the type to [`Type::Empty`]. Otherwise, if one of each corner radii
    /// are zero, sets the type to [`Type::Rect`]. Otherwise, if all x-axis radii are equal and at
    /// least half `rect.width()`, and all y-axis radii are equal at least half `rect.height()`,
    /// sets the type to [`Type::Oval`]. Otherwise, if all x-axis radii are equal, and all y-axis
    /// radii are equal, sets the type to [`Type::Simple`]. Otherwise, sets the type to
    /// [`Type::NinePatch`].
    ///
    /// - `rect` bounds of rounded rectangle
    /// - `radii` corner x-axis and y-axis radii
    pub fn new_rect_radii(rect: impl AsRef<Rect>, radii: &[Vector; 4]) -> Self {
        let mut rr = Self::default();
        rr.set_rect_radii(rect, radii);
        rr
    }

    /// Initializes bounds to `rect`. Sets radii to (`left_rad`, `top_rad`), (`right_rad`,
    /// `top_rad`), (`right_rad`, `bottom_rad`), (`left_rad`, `bottom_rad`).
    ///
    /// If `rect` is empty, sets the type to [`Type::Empty`]. Otherwise, if `left_rad` and
    /// `right_rad` are zero, sets the type to [`Type::Rect`]. Otherwise, if `top_rad` and
    /// `bottom_rad` are zero, sets the type to [`Type::Rect`]. Otherwise, if `left_rad` and
    /// `right_rad` are equal and at least half `rect.width()`, and `top_rad` and `bottom_rad` are
    /// equal at least half `rect.height()`, sets the type to [`Type::Oval`]. Otherwise, if
    /// `left_rad` and `right_rad` are equal, and `top_rad` and `bottom_rad` are equal, sets the
    /// type to [`Type::Simple`]. Otherwise, sets the type to [`Type::NinePatch`].
    ///
    /// Nine patch refers to the nine parts defined by the radii: one center rectangle, four edge
    /// patches, and four corner patches.
    ///
    /// - `rect` bounds of rounded rectangle
    /// - `left_rad` left-top and left-bottom x-axis radius
    /// - `top_rad` left-top and right-top y-axis radius
    /// - `right_rad` right-top and right-bottom x-axis radius
    /// - `bottom_rad` left-bottom and right-bottom y-axis radius
    pub fn new_nine_patch(
        rect: impl AsRef<Rect>,
        left_rad: scalar,
        top_rad: scalar,
        right_rad: scalar,
        bottom_rad: scalar,
    ) -> Self {
        let mut r = Self::default();
        r.set_nine_patch(rect, left_rad, top_rad, right_rad, bottom_rad);
        r
    }

    /// Sets bounds to `oval`, x-axis radii to half `oval.width()`, and all y-axis radii to half
    /// `oval.height()`. If the oval bounds is empty, sets the type to [`Type::Empty`]. Otherwise,
    /// sets the type to [`Type::Oval`].
    ///
    /// - `oval` bounds of oval
    pub fn set_oval(&mut self, oval: impl AsRef<Rect>) {
        unsafe { self.native_mut().setOval(oval.as_ref().native()) }
    }

    /// Sets to a rounded rectangle with the same radii for all four corners. If `rect` is empty,
    /// sets the type to [`Type::Empty`]. Otherwise, if `x_rad` or `y_rad` is zero, sets the type to
    /// [`Type::Rect`]. Otherwise, if `x_rad` is at least half `rect.width()` and `y_rad` is at
    /// least half `rect.height()`, sets the type to [`Type::Oval`]. Otherwise, sets the type to
    /// [`Type::Simple`].
    ///
    /// - `rect` bounds of rounded rectangle
    /// - `x_rad` x-axis radius of corners
    /// - `y_rad` y-axis radius of corners
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_setRectXY>
    pub fn set_rect_xy(&mut self, rect: impl AsRef<Rect>, x_rad: scalar, y_rad: scalar) {
        unsafe {
            self.native_mut()
                .setRectXY(rect.as_ref().native(), x_rad, y_rad)
        }
    }

    /// Sets bounds to `rect`. Sets radii to (`left_rad`, `top_rad`), (`right_rad`, `top_rad`),
    /// (`right_rad`, `bottom_rad`), (`left_rad`, `bottom_rad`).
    ///
    /// If `rect` is empty, sets the type to [`Type::Empty`]. Otherwise, if `left_rad` and
    /// `right_rad` are zero, sets the type to [`Type::Rect`]. Otherwise, if `top_rad` and
    /// `bottom_rad` are zero, sets the type to [`Type::Rect`]. Otherwise, if `left_rad` and
    /// `right_rad` are equal and at least half `rect.width()`, and `top_rad` and `bottom_rad` are
    /// equal at least half `rect.height()`, sets the type to [`Type::Oval`]. Otherwise, if
    /// `left_rad` and `right_rad` are equal, and `top_rad` and `bottom_rad` are equal, sets the
    /// type to [`Type::Simple`]. Otherwise, sets the type to [`Type::NinePatch`].
    ///
    /// Nine patch refers to the nine parts defined by the radii: one center rectangle, four edge
    /// patches, and four corner patches.
    ///
    /// - `rect` bounds of rounded rectangle
    /// - `left_rad` left-top and left-bottom x-axis radius
    /// - `top_rad` left-top and right-top y-axis radius
    /// - `right_rad` right-top and right-bottom x-axis radius
    /// - `bottom_rad` left-bottom and right-bottom y-axis radius
    pub fn set_nine_patch(
        &mut self,
        rect: impl AsRef<Rect>,
        left_rad: scalar,
        top_rad: scalar,
        right_rad: scalar,
        bottom_rad: scalar,
    ) {
        unsafe {
            self.native_mut().setNinePatch(
                rect.as_ref().native(),
                left_rad,
                top_rad,
                right_rad,
                bottom_rad,
            )
        }
    }

    /// Sets bounds to `rect`. Sets the radii array for individual control of all four corners.
    ///
    /// If `rect` is empty, sets the type to [`Type::Empty`]. Otherwise, if one of each corner radii
    /// are zero, sets the type to [`Type::Rect`]. Otherwise, if all x-axis radii are equal and at
    /// least half `rect.width()`, and all y-axis radii are equal at least half `rect.height()`,
    /// sets the type to [`Type::Oval`]. Otherwise, if all x-axis radii are equal, and all y-axis
    /// radii are equal, sets the type to [`Type::Simple`]. Otherwise, sets the type to
    /// [`Type::NinePatch`].
    ///
    /// - `rect` bounds of rounded rectangle
    /// - `radii` corner x-axis and y-axis radii
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_setRectRadii>
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_setRectRadii>
    pub fn set_rect_radii(&mut self, rect: impl AsRef<Rect>, radii: &[Vector; 4]) {
        unsafe {
            self.native_mut()
                .setRectRadii(rect.as_ref().native(), radii.native().as_ptr())
        }
    }

    /// Returns bounds. Bounds may have zero width or zero height. Bounds right is greater than or
    /// equal to left; bounds bottom is greater than or equal to top. Result is identical to
    /// [`Self::bounds()`].
    pub fn rect(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fRect)
    }

    /// Returns the scalar pair for the radius of the curve on the x-axis and y-axis for one corner.
    /// Both radii may be zero. If not zero, both are positive and finite.
    ///
    /// - `corner` corner to return radii for
    pub fn radii(&self, corner: Corner) -> Vector {
        Vector::from_native_c(self.native().fRadii[corner as usize])
    }

    /// Returns the corner radii for all four corners, in the same order as [`Corner`].
    pub fn radii_ref(&self) -> &[Vector; 4] {
        Vector::from_native_array_ref(&self.native().fRadii)
    }

    /// Returns bounds. Bounds may have zero width or zero height. Bounds right is greater than or
    /// equal to left; bounds bottom is greater than or equal to top. Result is identical to
    /// [`Self::rect()`].
    pub fn bounds(&self) -> &Rect {
        self.rect()
    }

    /// Insets bounds by `delta`, and adjusts radii by `delta`. `delta` may be positive, negative,
    /// or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged. Otherwise, if
    /// the adjusted radius becomes negative, pins the radius to zero. If `delta.x` exceeds half the
    /// bounds width, the bounds left and right are set to the bounds x-axis center. If `delta.y`
    /// exceeds half the bounds height, the bounds top and bottom are set to the bounds y-axis
    /// center.
    ///
    /// If `delta` causes the bounds to become infinite, the bounds is zeroed.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_inset>
    pub fn inset(&mut self, delta: impl Into<Vector>) {
        *self = self.with_inset(delta)
    }

    /// Copies this rounded rectangle to a new one, then insets the new bounds by `delta`, and
    /// adjusts the new radii by `delta`. `delta` may be positive, negative, or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged. Otherwise, if
    /// the adjusted radius becomes negative, pins the radius to zero. If `delta.x` exceeds half the
    /// new bounds width, the new bounds left and right are set to the new bounds x-axis center. If
    /// `delta.y` exceeds half the new bounds height, the new bounds top and bottom are set to the
    /// new bounds y-axis center.
    ///
    /// If `delta` causes the new bounds to become infinite, the new bounds is zeroed.
    #[must_use]
    pub fn with_inset(&self, delta: impl Into<Vector>) -> Self {
        let delta = delta.into();
        let mut r = Self::default();
        unsafe { self.native().inset(delta.x, delta.y, r.native_mut()) };
        r
    }

    /// Outsets bounds by `delta`, and adjusts radii by `delta`. `delta` may be positive, negative,
    /// or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged. Otherwise, if
    /// the adjusted radius becomes negative, pins the radius to zero. If `delta.x` exceeds half the
    /// bounds width, the bounds left and right are set to the bounds x-axis center. If `delta.y`
    /// exceeds half the bounds height, the bounds top and bottom are set to the bounds y-axis
    /// center.
    ///
    /// If `delta` causes the bounds to become infinite, the bounds is zeroed.
    pub fn outset(&mut self, delta: impl Into<Vector>) {
        *self = self.with_outset(delta)
    }

    /// Copies this rounded rectangle to a new one, then outsets the new bounds by `delta`, and
    /// adjusts the new radii by `delta`. `delta` may be positive, negative, or zero.
    ///
    /// If either corner radius is zero, the corner has no curvature and is unchanged. Otherwise, if
    /// the adjusted radius becomes negative, pins the radius to zero. If `delta.x` exceeds half the
    /// new bounds width, the new bounds left and right are set to the new bounds x-axis center. If
    /// `delta.y` exceeds half the new bounds height, the new bounds top and bottom are set to the
    /// new bounds y-axis center.
    ///
    /// If `delta` causes the new bounds to become infinite, the new bounds is zeroed.
    #[must_use]
    pub fn with_outset(&self, delta: impl Into<Vector>) -> Self {
        self.with_inset(-delta.into())
    }

    /// Translates this rounded rectangle by `delta`.
    pub fn offset(&mut self, delta: impl Into<Vector>) {
        let delta = delta.into();
        unsafe { self.native_mut().offset(delta.x, delta.y) }
    }

    /// Returns this rounded rectangle translated by `delta`, with unchanged corner radii.
    #[must_use]
    pub fn with_offset(&self, delta: impl Into<Vector>) -> Self {
        let mut copied = *self;
        copied.offset(delta);
        copied
    }

    /// Returns true if `point` is inside the bounds and corner radii, and if this rounded rectangle
    /// is not empty.
    pub fn contains_point(&self, point: impl Into<Point>) -> bool {
        let point = point.into();
        unsafe { sb::C_SkRRect_containsPoint(self.native(), point.native()) }
    }

    /// Returns true if `rect` is inside the bounds and corner radii, and if this rounded rectangle
    /// and `rect` are not empty.
    ///
    /// - `rect` area tested for containment
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_contains>
    pub fn contains(&self, rect: impl AsRef<Rect>) -> bool {
        unsafe { sb::C_SkRRect_containsRect(self.native(), rect.as_ref().native()) }
    }

    /// Returns true if the bounds and radii values are finite and describe a [`RRect`] type that
    /// matches [`Self::get_type()`]. All [`RRect`] methods construct valid types, even if the input
    /// values are not valid. Invalid [`RRect`] data can only be generated by corrupting memory.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_isValid>
    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }

    pub const SIZE_IN_MEMORY: usize = mem::size_of::<Self>();

    /// Writes this rounded rectangle to `buffer`. Writes [`Self::SIZE_IN_MEMORY`] bytes, and
    /// returns [`Self::SIZE_IN_MEMORY`], the number of bytes written.
    ///
    /// - `buffer` storage for this rounded rectangle
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_writeToMemory>
    pub fn write_to_memory(&self, buffer: &mut Vec<u8>) {
        unsafe {
            let size = self.native().writeToMemory(ptr::null_mut());
            buffer.resize(size, 0);
            let written = self.native().writeToMemory(buffer.as_mut_ptr() as _);
            debug_assert_eq!(written, size);
        }
    }

    /// Reads this rounded rectangle from `buffer`, reading [`Self::SIZE_IN_MEMORY`] bytes. Returns
    /// [`Self::SIZE_IN_MEMORY`], the bytes read, if `buffer.len()` is at least
    /// [`Self::SIZE_IN_MEMORY`]. Otherwise, returns zero.
    ///
    /// - `buffer` memory to read from
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_readFromMemory>
    pub fn read_from_memory(&mut self, buffer: &[u8]) -> usize {
        unsafe {
            self.native_mut()
                .readFromMemory(buffer.as_ptr() as _, buffer.len())
        }
    }

    /// Transforms this rounded rectangle by `matrix` and returns it if possible. If the matrix does
    /// not preserve axis-alignment (e.g. rotates, skews, etc.) then this returns `None`.
    #[must_use]
    pub fn transform(&self, matrix: &Matrix) -> Option<Self> {
        let mut r = Self::default();
        unsafe { self.native().transform1(matrix.native(), r.native_mut()) }.then_some(r)
    }

    /// Writes a text representation of this rounded rectangle to standard output. Set `as_hex` true
    /// to generate exact binary representations of floating point numbers.
    ///
    /// - `as_hex` true if scalar values are written as hexadecimal
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@RRect_dump>
    pub fn dump(&self, as_hex: impl Into<Option<bool>>) {
        unsafe { self.native().dump(as_hex.into().unwrap_or_default()) }
    }

    pub fn dump_to_string(&self, as_hex: bool) -> String {
        let mut str = interop::String::default();
        unsafe { sb::C_SkRRect_dumpToString(self.native(), as_hex, str.native_mut()) }
        str.to_string()
    }

    pub fn dump_hex(&self) {
        self.dump(true)
    }
}
