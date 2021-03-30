#[cfg(feature = "gpu")]
use crate::gpu;
use crate::{
    prelude::*, scalar, u8cpu, Bitmap, BlendMode, ClipOp, Color, Color4f, Data, Drawable,
    FilterMode, Font, IPoint, IRect, ISize, Image, ImageFilter, ImageInfo, Matrix, Paint, Path,
    Picture, Pixmap, Point, QuickReject, RRect, Rect, Region, SamplingOptions, Shader, Surface,
    SurfaceProps, TextBlob, TextEncoding, Vector, Vertices, M44,
};
use skia_bindings as sb;
use skia_bindings::{
    SkAutoCanvasRestore, SkCanvas, SkCanvas_SaveLayerRec, SkImageFilter, SkPaint, SkRect,
};
use std::{
    convert::TryInto,
    ffi::CString,
    fmt,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr, slice,
};

pub use lattice::Lattice;

bitflags! {
    /** \enum SkCanvas::SaveLayerFlagsSet
        SaveLayerFlags provides options that may be used in any combination in SaveLayerRec,
        defining how layer allocated by saveLayer() operates. It may be set to zero,
        kPreserveLCDText_SaveLayerFlag, kInitWithPrevious_SaveLayerFlag, or both flags.
    */
    pub struct SaveLayerFlags: u32 {
        const PRESERVE_LCD_TEXT = sb::SkCanvas_SaveLayerFlagsSet_kPreserveLCDText_SaveLayerFlag as _;
        /// initializes with previous contents
        const INIT_WITH_PREVIOUS = sb::SkCanvas_SaveLayerFlagsSet_kInitWithPrevious_SaveLayerFlag as _;
        const F16_COLOR_TYPE = sb::SkCanvas_SaveLayerFlagsSet_kF16ColorType as _;
    }
}

/** \struct SkCanvas::SaveLayerRec
    SaveLayerRec contains the state used to create the layer.
*/
#[allow(dead_code)]
pub struct SaveLayerRec<'a> {
    // We _must_ store _references_ to the native types here, because not all of them are native
    // transmutable, like ImageFilter or Image, which are represented as ref counted pointers and so
    // we would store a reference to a pointer only.
    /// hints at layer size limit
    bounds: Option<&'a SkRect>,
    /** modifies overlay */
    paint: Option<&'a SkPaint>,
    /**
     *  If not null, this triggers the same initialization behavior as setting
     *  kInitWithPrevious_SaveLayerFlag on fSaveLayerFlags: the current layer is copied into
     *  the new layer, rather than initializing the new layer with transparent-black.
     *  This is then filtered by fBackdrop (respecting the current clip).
     */
    backdrop: Option<&'a SkImageFilter>,
    /** preserves LCD text, creates with prior layer contents */
    flags: SaveLayerFlags,
}

impl<'a> NativeTransmutable<SkCanvas_SaveLayerRec> for SaveLayerRec<'a> {}

#[test]
fn test_save_layer_rec_layout() {
    SaveLayerRec::test_layout()
}

impl fmt::Debug for SaveLayerRec<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SaveLayerRec")
            .field("bounds", &self.bounds.map(Rect::from_native_ref))
            .field("paint", &self.paint.map(Paint::from_native_ref))
            .field(
                "backdrop",
                &ImageFilter::from_unshared_ptr_ref(&(self.backdrop.as_ptr_or_null() as *mut _)),
            )
            .field("flags", &self.flags)
            .finish()
    }
}

impl<'a> Default for SaveLayerRec<'a> {
    /** Sets fBounds, fPaint, and fBackdrop to nullptr. Clears fSaveLayerFlags.

        @return  empty SaveLayerRec
    */
    fn default() -> Self {
        SaveLayerRec {
            bounds: None,
            paint: None,
            backdrop: None,
            flags: SaveLayerFlags::empty(),
        }
    }
}

impl<'a> SaveLayerRec<'a> {
    pub fn bounds(self, bounds: &'a Rect) -> Self {
        Self {
            bounds: Some(bounds.native()),
            ..self
        }
    }

    pub fn paint(self, paint: &'a Paint) -> Self {
        Self {
            paint: Some(paint.native()),
            ..self
        }
    }

    pub fn backdrop(self, backdrop: &'a ImageFilter) -> Self {
        Self {
            backdrop: Some(backdrop.native()),
            ..self
        }
    }

    #[deprecated(
        since = "0.33.0",
        note = "removed without replacement, does not set clip_mask"
    )]
    pub fn clip_mask(self, _clip_mask: &'a Image) -> Self {
        self
    }

    #[deprecated(
        since = "0.33.0",
        note = "removed without replacement, does not set clip_matrix"
    )]
    pub fn clip_matrix(self, _clip_matrix: &'a Matrix) -> Self {
        self
    }

    pub fn flags(self, flags: SaveLayerFlags) -> Self {
        Self { flags, ..self }
    }
}

/** \enum SkCanvas::PointMode
    Selects if an array of points are drawn as discrete points, as lines, or as
    an open polygon.
*/
pub use sb::SkCanvas_PointMode as PointMode;

#[test]
fn test_canvas_point_mode_naming() {
    let _ = PointMode::Polygon;
}

/** \enum SkCanvas::SrcRectConstraint
    SrcRectConstraint controls the behavior at the edge of source SkRect,
    provided to drawImageRect(), trading off speed for precision.

    SkFilterQuality in SkPaint may sample multiple pixels in the image. Source SkRect
    restricts the bounds of pixels that may be read. SkFilterQuality may slow down if
    it cannot read outside the bounds, when sampling near the edge of source SkRect.
    SrcRectConstraint specifies whether an SkImageFilter is allowed to read pixels
    outside source SkRect.
*/
pub use sb::SkCanvas_SrcRectConstraint as SrcRectConstraint;

#[test]
fn test_src_rect_constraint_naming() {
    let _ = SrcRectConstraint::Fast;
}

/// Provides access to Canvas's pixels.
/// Returned by Canvas::access_top_layer_pixels()
#[derive(Debug)]
pub struct TopLayerPixels<'a> {
    pub pixels: &'a mut [u8],
    pub info: ImageInfo,
    pub row_bytes: usize,
    pub origin: IPoint,
}

///  SkCanvas provides an interface for drawing, and how the drawing is clipped and transformed.
///  SkCanvas contains a stack of SkMatrix and clip values.
///
///  SkCanvas and SkPaint together provide the state to draw into SkSurface or SkBaseDevice.
///  Each SkCanvas draw call transforms the geometry of the object by the concatenation of all
///  SkMatrix values in the stack. The transformed geometry is clipped by the intersection
///  of all of clip values in the stack. The SkCanvas draw calls use SkPaint to supply drawing
///  state such as color, SkTypeface, text size, stroke width, SkShader and so on.
///
///  To draw to a pixel-based destination, create raster surface or GPU surface.
///  Request SkCanvas from SkSurface to obtain the interface to draw.
///  SkCanvas generated by raster surface draws to memory visible to the CPU.
///  SkCanvas generated by GPU surface uses Vulkan or OpenGL to draw to the GPU.
///
///  To draw to a document, obtain SkCanvas from SVG canvas, document PDF, or SkPictureRecorder.
///  SkDocument based SkCanvas and other SkCanvas subclasses reference SkBaseDevice describing the
///  destination.
///
///  SkCanvas can be constructed to draw to SkBitmap without first creating raster surface.
///  This approach may be deprecated in the future.
#[repr(transparent)]
pub struct Canvas(SkCanvas);

impl NativeAccess<SkCanvas> for Canvas {
    fn native(&self) -> &SkCanvas {
        &self.0
    }

    fn native_mut(&mut self) -> &mut SkCanvas {
        &mut self.0
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Canvas")
            .field("image_info", &self.image_info())
            .field("props", &self.props())
            .field("base_layer_size", &self.base_layer_size())
            .field("save_count", &self.save_count())
            .field("local_clip_bounds", &self.local_clip_bounds())
            .field("device_clip_bounds", &self.device_clip_bounds())
            .field("local_to_device", &self.local_to_device())
            .finish()
    }
}

/// A type representing a canvas that is owned and dropped when it goes out of scope _and_ is bound
/// to the lifetime of some value.
///
/// Functions are resolved with the [`Deref`] trait.
#[repr(transparent)]
pub struct OwnedCanvas<'lt>(ptr::NonNull<Canvas>, PhantomData<&'lt ()>);

impl Deref for OwnedCanvas<'_> {
    type Target = Canvas;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for OwnedCanvas<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for OwnedCanvas<'_> {
    /** Draws saved layers, if any.
        Frees up resources used by SkCanvas.

        example: https://fiddle.skia.org/c/@Canvas_destructor
    */
    fn drop(&mut self) {
        unsafe { sb::C_SkCanvas_delete(self.native()) }
    }
}

impl Default for OwnedCanvas<'_> {
    /// Creates an empty SkCanvas with no backing device or pixels, with
    /// a width and height of zero.
    ///
    /// @return  empty SkCanvas
    ///
    /// example: https://fiddle.skia.org/c/@Canvas_empty_constructor
    fn default() -> Self {
        let ptr = unsafe { sb::C_SkCanvas_newEmpty() };
        Canvas::own_from_native_ptr(ptr).unwrap()
    }
}

impl fmt::Debug for OwnedCanvas<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OwnedCanvas").field(self as &Canvas).finish()
    }
}

impl AsMut<Canvas> for Canvas {
    fn as_mut(&mut self) -> &mut Canvas {
        self
    }
}

impl<'lt> AsMut<Canvas> for OwnedCanvas<'lt> {
    fn as_mut(&mut self) -> &mut Canvas {
        self.deref_mut()
    }
}

impl Canvas {
    /// Allocates raster SkCanvas that will draw directly into pixels.
    ///
    /// SkCanvas is returned if all parameters are valid.
    /// Valid parameters include:
    /// info dimensions are zero or positive;
    /// info contains SkColorType and SkAlphaType supported by raster surface;
    /// pixels is not nullptr;
    /// rowBytes is zero or large enough to contain info width pixels of SkColorType.
    ///
    /// Pass zero for rowBytes to compute rowBytes from info width and size of pixel.
    /// If rowBytes is greater than zero, it must be equal to or greater than
    /// info width times bytes required for SkColorType.
    ///
    /// Pixel buffer size should be info height times computed rowBytes.
    /// Pixels are not initialized.
    /// To access pixels after drawing, call flush() or peekPixels().
    ///
    /// @param info      width, height, SkColorType, SkAlphaType, SkColorSpace, of raster surface;
    ///  width, or height, or both, may be zero
    /// @param pixels    pointer to destination pixels buffer
    /// @param rowBytes  interval from one SkSurface row to the next, or zero
    /// @param props     LCD striping orientation and setting for device independent fonts;
    ///  may be nullptr
    /// @return          SkCanvas if all parameters are valid; otherwise, nullptr
    pub fn from_raster_direct<'pixels>(
        info: &ImageInfo,
        pixels: &'pixels mut [u8],
        row_bytes: impl Into<Option<usize>>,
        props: Option<&SurfaceProps>,
    ) -> Option<OwnedCanvas<'pixels>> {
        let row_bytes = row_bytes.into().unwrap_or_else(|| info.min_row_bytes());
        if info.valid_pixels(row_bytes, pixels) {
            let ptr = unsafe {
                sb::C_SkCanvas_MakeRasterDirect(
                    info.native(),
                    pixels.as_mut_ptr() as _,
                    row_bytes,
                    props.native_ptr_or_null(),
                )
            };
            Self::own_from_native_ptr(ptr)
        } else {
            None
        }
    }

    /// Allocates raster SkCanvas specified by inline image specification. Subsequent SkCanvas
    /// calls draw into pixels.
    /// SkColorType is set to kN32_SkColorType.
    /// SkAlphaType is set to kPremul_SkAlphaType.
    /// To access pixels after drawing, call flush() or peekPixels().
    ///
    /// SkCanvas is returned if all parameters are valid.
    /// Valid parameters include:
    /// width and height are zero or positive;
    /// pixels is not nullptr;
    /// rowBytes is zero or large enough to contain width pixels of kN32_SkColorType.
    ///
    /// Pass zero for rowBytes to compute rowBytes from width and size of pixel.
    /// If rowBytes is greater than zero, it must be equal to or greater than
    /// width times bytes required for SkColorType.
    ///
    /// Pixel buffer size should be height times rowBytes.
    ///
    /// @param width     pixel column count on raster surface created; must be zero or greater
    /// @param height    pixel row count on raster surface created; must be zero or greater
    /// @param pixels    pointer to destination pixels buffer; buffer size should be height
    ///                     times rowBytes
    /// @param rowBytes  interval from one SkSurface row to the next, or zero
    /// @return          SkCanvas if all parameters are valid; otherwise, nullptr
    pub fn from_raster_direct_n32<'pixels>(
        size: impl Into<ISize>,
        pixels: &'pixels mut [u32],
        row_bytes: impl Into<Option<usize>>,
    ) -> Option<OwnedCanvas<'pixels>> {
        let info = ImageInfo::new_n32_premul(size, None);
        let pixels_ptr: *mut u8 = pixels.as_mut_ptr() as _;
        let pixels_u8: &'pixels mut [u8] =
            unsafe { slice::from_raw_parts_mut(pixels_ptr, mem::size_of_val(pixels)) };
        Self::from_raster_direct(&info, pixels_u8, row_bytes, None)
    }

    /** Creates SkCanvas of the specified dimensions without a SkSurface.
        Used by subclasses with custom implementations for draw member functions.

        If props equals nullptr, SkSurfaceProps are created with
        SkSurfaceProps::InitType settings, which choose the pixel striping
        direction and order. Since a platform may dynamically change its direction when
        the device is rotated, and since a platform may have multiple monitors with
        different characteristics, it is best not to rely on this legacy behavior.

        @param width   zero or greater
        @param height  zero or greater
        @param props   LCD striping orientation and setting for device independent fonts;
                       may be nullptr
        @return        SkCanvas placeholder with dimensions

        example: https://fiddle.skia.org/c/@Canvas_int_int_const_SkSurfaceProps_star
    */
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'lt>(
        size: impl Into<ISize>,
        props: Option<&SurfaceProps>,
    ) -> Option<OwnedCanvas<'lt>> {
        let size = size.into();
        if size.width >= 0 && size.height >= 0 {
            let ptr = unsafe {
                sb::C_SkCanvas_newWidthHeightAndProps(
                    size.width,
                    size.height,
                    props.native_ptr_or_null(),
                )
            };
            Canvas::own_from_native_ptr(ptr)
        } else {
            None
        }
    }

    /** Constructs a canvas that draws into bitmap.
        Use props to match the device characteristics, like LCD striping.

        bitmap is copied so that subsequently editing bitmap will not affect
        constructed SkCanvas.

        @param bitmap  width, height, SkColorType, SkAlphaType,
                       and pixel storage of raster surface
        @param props   order and orientation of RGB striping; and whether to use
                       device independent fonts
        @return        SkCanvas that can be used to draw into bitmap

        example: https://fiddle.skia.org/c/@Canvas_const_SkBitmap_const_SkSurfaceProps
    */
    pub fn from_bitmap<'lt>(bitmap: &Bitmap, props: Option<&SurfaceProps>) -> OwnedCanvas<'lt> {
        let props_ptr = props.native_ptr_or_null();
        let ptr = if props_ptr.is_null() {
            unsafe { sb::C_SkCanvas_newFromBitmap(bitmap.native()) }
        } else {
            unsafe { sb::C_SkCanvas_newFromBitmapAndProps(bitmap.native(), props_ptr) }
        };
        Canvas::own_from_native_ptr(ptr).unwrap()
    }

    /** Returns SkImageInfo for SkCanvas. If SkCanvas is not associated with raster surface or
        GPU surface, returned SkColorType is set to kUnknown_SkColorType.

        @return  dimensions and SkColorType of SkCanvas

        example: https://fiddle.skia.org/c/@Canvas_imageInfo
    */
    pub fn image_info(&self) -> ImageInfo {
        let mut ii = ImageInfo::default();
        unsafe { sb::C_SkCanvas_imageInfo(self.native(), ii.native_mut()) };
        ii
    }

    /** Copies SkSurfaceProps, if SkCanvas is associated with raster surface or
        GPU surface, and returns true. Otherwise, returns false and leave props unchanged.

        @param props  storage for writable SkSurfaceProps
        @return       true if SkSurfaceProps was copied

        example: https://fiddle.skia.org/c/@Canvas_getProps
    */
    pub fn props(&self) -> Option<SurfaceProps> {
        let mut sp = SurfaceProps::default();
        unsafe { self.native().getProps(sp.native_mut()) }.if_true_some(sp)
    }

    /** Triggers the immediate execution of all pending draw operations.
        If SkCanvas is associated with GPU surface, resolves all pending GPU operations.
        If SkCanvas is associated with raster surface, has no effect; raster draw
        operations are never deferred.

        DEPRECATED: Replace usage with GrDirectContext::flush()
    */
    #[deprecated(since = "0.38.0", note = "Replace usage with DirectContext::flush()")]
    pub fn flush(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().flush();
        }
        self
    }

    /** Gets the size of the base or root layer in global canvas coordinates. The
        origin of the base layer is always (0,0). The area available for drawing may be
        smaller (due to clipping or saveLayer).

        @return  integral width and height of base layer

        example: https://fiddle.skia.org/c/@Canvas_getBaseLayerSize
    */
    pub fn base_layer_size(&self) -> ISize {
        let mut size = ISize::default();
        unsafe { sb::C_SkCanvas_getBaseLayerSize(self.native(), size.native_mut()) }
        size
    }

    /** Creates SkSurface matching info and props, and associates it with SkCanvas.
        Returns nullptr if no match found.

        If props is nullptr, matches SkSurfaceProps in SkCanvas. If props is nullptr and SkCanvas
        does not have SkSurfaceProps, creates SkSurface with default SkSurfaceProps.

        @param info   width, height, SkColorType, SkAlphaType, and SkColorSpace
        @param props  SkSurfaceProps to match; may be nullptr to match SkCanvas
        @return       SkSurface matching info and props, or nullptr if no match is available

        example: https://fiddle.skia.org/c/@Canvas_makeSurface
    */
    pub fn new_surface(
        &mut self,
        info: &ImageInfo,
        props: Option<&SurfaceProps>,
    ) -> Option<Surface> {
        Surface::from_ptr(unsafe {
            sb::C_SkCanvas_makeSurface(self.native_mut(), info.native(), props.native_ptr_or_null())
        })
    }

    /** Returns GPU context of the GPU surface associated with SkCanvas.

        @return  GPU context, if available; nullptr otherwise

        example: https://fiddle.skia.org/c/@Canvas_recordingContext
    */
    #[cfg(feature = "gpu")]
    pub fn recording_context(&mut self) -> Option<gpu::RecordingContext> {
        gpu::RecordingContext::from_unshared_ptr(unsafe {
            sb::C_SkCanvas_recordingContext(self.native_mut())
        })
    }

    /** Sometimes a canvas is owned by a surface. If it is, getSurface() will return a bare
     *  pointer to that surface, else this will return nullptr.
     */
    /// # Safety
    /// This function is unsafe because it is not clear how exactly the lifetime of the canvas
    /// relates to surface returned.
    /// TODO: It might be possible to make this safe by returning a _kind of_ reference to the
    ///       Surface that can not be cloned and stays bound to the lifetime of canvas.
    ///       But even then, the Surface might exist twice then, which is confusing, but
    ///       probably safe, because the first instance is borrowed by the canvas.
    /// See also `OwnedCanvas`, `Surface::canvas()`.
    pub unsafe fn surface(&mut self) -> Option<Surface> {
        Surface::from_unshared_ptr(self.native_mut().getSurface())
    }

    /** Returns the pixel base address, SkImageInfo, rowBytes, and origin if the pixels
        can be read directly. The returned address is only valid
        while SkCanvas is in scope and unchanged. Any SkCanvas call or SkSurface call
        may invalidate the returned address and other returned values.

        If pixels are inaccessible, info, rowBytes, and origin are unchanged.

        @param info      storage for writable pixels' SkImageInfo; may be nullptr
        @param rowBytes  storage for writable pixels' row bytes; may be nullptr
        @param origin    storage for SkCanvas top layer origin, its top-left corner;
                         may be nullptr
        @return          address of pixels, or nullptr if inaccessible

        example: https://fiddle.skia.org/c/@Canvas_accessTopLayerPixels_a
        example: https://fiddle.skia.org/c/@Canvas_accessTopLayerPixels_b
    */
    pub fn access_top_layer_pixels(&mut self) -> Option<TopLayerPixels> {
        let mut info = ImageInfo::default();
        let mut row_bytes = 0;
        let mut origin = IPoint::default();
        let ptr = unsafe {
            self.native_mut().accessTopLayerPixels(
                info.native_mut(),
                &mut row_bytes,
                origin.native_mut(),
            )
        };
        if !ptr.is_null() {
            let size = info.compute_byte_size(row_bytes);
            let pixels = unsafe { slice::from_raw_parts_mut(ptr as _, size) };
            Some(TopLayerPixels {
                pixels,
                info,
                row_bytes,
                origin,
            })
        } else {
            None
        }
    }

    // TODO: accessTopRasterHandle()

    /** Returns true if SkCanvas has direct access to its pixels.

        Pixels are readable when SkBaseDevice is raster. Pixels are not readable when SkCanvas
        is returned from GPU surface, returned by SkDocument::beginPage, returned by
        SkPictureRecorder::beginRecording, or SkCanvas is the base of a utility class
        like DebugCanvas.

        pixmap is valid only while SkCanvas is in scope and unchanged. Any
        SkCanvas or SkSurface call may invalidate the pixmap values.

        @param pixmap  storage for pixel state if pixels are readable; otherwise, ignored
        @return        true if SkCanvas has direct access to pixels

        example: https://fiddle.skia.org/c/@Canvas_peekPixels
    */
    pub fn peek_pixels(&mut self) -> Option<Borrows<Pixmap>> {
        let mut pixmap = Pixmap::default();
        unsafe { self.native_mut().peekPixels(pixmap.native_mut()) }
            .if_true_then_some(move || pixmap.borrows(self))
    }

    /** Copies SkRect of pixels from SkCanvas into dstPixels. SkMatrix and clip are
        ignored.

        Source SkRect corners are (srcX, srcY) and (imageInfo().width(), imageInfo().height()).
        Destination SkRect corners are (0, 0) and (dstInfo.width(), dstInfo.height()).
        Copies each readable pixel intersecting both rectangles, without scaling,
        converting to dstInfo.colorType() and dstInfo.alphaType() if required.

        Pixels are readable when SkBaseDevice is raster, or backed by a GPU.
        Pixels are not readable when SkCanvas is returned by SkDocument::beginPage,
        returned by SkPictureRecorder::beginRecording, or SkCanvas is the base of a utility
        class like DebugCanvas.

        The destination pixel storage must be allocated by the caller.

        Pixel values are converted only if SkColorType and SkAlphaType
        do not match. Only pixels within both source and destination rectangles
        are copied. dstPixels contents outside SkRect intersection are unchanged.

        Pass negative values for srcX or srcY to offset pixels across or down destination.

        Does not copy, and returns false if:
        - Source and destination rectangles do not intersect.
        - SkCanvas pixels could not be converted to dstInfo.colorType() or dstInfo.alphaType().
        - SkCanvas pixels are not readable; for instance, SkCanvas is document-based.
        - dstRowBytes is too small to contain one row of pixels.

        @param dstInfo      width, height, SkColorType, and SkAlphaType of dstPixels
        @param dstPixels    storage for pixels; dstInfo.height() times dstRowBytes, or larger
        @param dstRowBytes  size of one destination row; dstInfo.width() times pixel size, or larger
        @param srcX         offset into readable pixels on x-axis; may be negative
        @param srcY         offset into readable pixels on y-axis; may be negative
        @return             true if pixels were copied
    */
    #[must_use]
    pub fn read_pixels(
        &mut self,
        info: &ImageInfo,
        dst_pixels: &mut [u8],
        dst_row_bytes: usize,
        src_point: impl Into<IPoint>,
    ) -> bool {
        let src_point = src_point.into();
        let required_size = info.compute_byte_size(dst_row_bytes);
        (dst_pixels.len() >= required_size)
            && unsafe {
                self.native_mut().readPixels(
                    info.native(),
                    dst_pixels.as_mut_ptr() as _,
                    dst_row_bytes,
                    src_point.x,
                    src_point.y,
                )
            }
    }

    /** Copies SkRect of pixels from SkCanvas into pixmap. SkMatrix and clip are
        ignored.

        Source SkRect corners are (srcX, srcY) and (imageInfo().width(), imageInfo().height()).
        Destination SkRect corners are (0, 0) and (pixmap.width(), pixmap.height()).
        Copies each readable pixel intersecting both rectangles, without scaling,
        converting to pixmap.colorType() and pixmap.alphaType() if required.

        Pixels are readable when SkBaseDevice is raster, or backed by a GPU.
        Pixels are not readable when SkCanvas is returned by SkDocument::beginPage,
        returned by SkPictureRecorder::beginRecording, or SkCanvas is the base of a utility
        class like DebugCanvas.

        Caller must allocate pixel storage in pixmap if needed.

        Pixel values are converted only if SkColorType and SkAlphaType
        do not match. Only pixels within both source and destination SkRect
        are copied. pixmap pixels contents outside SkRect intersection are unchanged.

        Pass negative values for srcX or srcY to offset pixels across or down pixmap.

        Does not copy, and returns false if:
        - Source and destination rectangles do not intersect.
        - SkCanvas pixels could not be converted to pixmap.colorType() or pixmap.alphaType().
        - SkCanvas pixels are not readable; for instance, SkCanvas is document-based.
        - SkPixmap pixels could not be allocated.
        - pixmap.rowBytes() is too small to contain one row of pixels.

        @param pixmap  storage for pixels copied from SkCanvas
        @param srcX    offset into readable pixels on x-axis; may be negative
        @param srcY    offset into readable pixels on y-axis; may be negative
        @return        true if pixels were copied

        example: https://fiddle.skia.org/c/@Canvas_readPixels_2
    */
    #[must_use]
    pub fn read_pixels_to_pixmap(&mut self, pixmap: &mut Pixmap, src: impl Into<IPoint>) -> bool {
        let src = src.into();
        unsafe { self.native_mut().readPixels1(pixmap.native(), src.x, src.y) }
    }

    /** Copies SkRect of pixels from SkCanvas into bitmap. SkMatrix and clip are
        ignored.

        Source SkRect corners are (srcX, srcY) and (imageInfo().width(), imageInfo().height()).
        Destination SkRect corners are (0, 0) and (bitmap.width(), bitmap.height()).
        Copies each readable pixel intersecting both rectangles, without scaling,
        converting to bitmap.colorType() and bitmap.alphaType() if required.

        Pixels are readable when SkBaseDevice is raster, or backed by a GPU.
        Pixels are not readable when SkCanvas is returned by SkDocument::beginPage,
        returned by SkPictureRecorder::beginRecording, or SkCanvas is the base of a utility
        class like DebugCanvas.

        Caller must allocate pixel storage in bitmap if needed.

        SkBitmap values are converted only if SkColorType and SkAlphaType
        do not match. Only pixels within both source and destination rectangles
        are copied. SkBitmap pixels outside SkRect intersection are unchanged.

        Pass negative values for srcX or srcY to offset pixels across or down bitmap.

        Does not copy, and returns false if:
        - Source and destination rectangles do not intersect.
        - SkCanvas pixels could not be converted to bitmap.colorType() or bitmap.alphaType().
        - SkCanvas pixels are not readable; for instance, SkCanvas is document-based.
        - bitmap pixels could not be allocated.
        - bitmap.rowBytes() is too small to contain one row of pixels.

        @param bitmap  storage for pixels copied from SkCanvas
        @param srcX    offset into readable pixels on x-axis; may be negative
        @param srcY    offset into readable pixels on y-axis; may be negative
        @return        true if pixels were copied

        example: https://fiddle.skia.org/c/@Canvas_readPixels_3
    */
    #[must_use]
    pub fn read_pixels_to_bitmap(&mut self, bitmap: &mut Bitmap, src: impl Into<IPoint>) -> bool {
        let src = src.into();
        unsafe { self.native_mut().readPixels2(bitmap.native(), src.x, src.y) }
    }

    /** Copies SkRect from pixels to SkCanvas. SkMatrix and clip are ignored.
        Source SkRect corners are (0, 0) and (info.width(), info.height()).
        Destination SkRect corners are (x, y) and
        (imageInfo().width(), imageInfo().height()).

        Copies each readable pixel intersecting both rectangles, without scaling,
        converting to imageInfo().colorType() and imageInfo().alphaType() if required.

        Pixels are writable when SkBaseDevice is raster, or backed by a GPU.
        Pixels are not writable when SkCanvas is returned by SkDocument::beginPage,
        returned by SkPictureRecorder::beginRecording, or SkCanvas is the base of a utility
        class like DebugCanvas.

        Pixel values are converted only if SkColorType and SkAlphaType
        do not match. Only pixels within both source and destination rectangles
        are copied. SkCanvas pixels outside SkRect intersection are unchanged.

        Pass negative values for x or y to offset pixels to the left or
        above SkCanvas pixels.

        Does not copy, and returns false if:
        - Source and destination rectangles do not intersect.
        - pixels could not be converted to SkCanvas imageInfo().colorType() or
        imageInfo().alphaType().
        - SkCanvas pixels are not writable; for instance, SkCanvas is document-based.
        - rowBytes is too small to contain one row of pixels.

        @param info      width, height, SkColorType, and SkAlphaType of pixels
        @param pixels    pixels to copy, of size info.height() times rowBytes, or larger
        @param rowBytes  size of one row of pixels; info.width() times pixel size, or larger
        @param x         offset into SkCanvas writable pixels on x-axis; may be negative
        @param y         offset into SkCanvas writable pixels on y-axis; may be negative
        @return          true if pixels were written to SkCanvas

        example: https://fiddle.skia.org/c/@Canvas_writePixels
    */
    #[must_use]
    pub fn write_pixels(
        &mut self,
        info: &ImageInfo,
        pixels: &[u8],
        row_bytes: usize,
        offset: impl Into<IPoint>,
    ) -> bool {
        let offset = offset.into();
        let required_size = info.compute_byte_size(row_bytes);
        (pixels.len() >= required_size)
            && unsafe {
                self.native_mut().writePixels(
                    info.native(),
                    pixels.as_ptr() as _,
                    row_bytes,
                    offset.x,
                    offset.y,
                )
            }
    }

    /** Copies SkRect from pixels to SkCanvas. SkMatrix and clip are ignored.
        Source SkRect corners are (0, 0) and (bitmap.width(), bitmap.height()).

        Destination SkRect corners are (x, y) and
        (imageInfo().width(), imageInfo().height()).

        Copies each readable pixel intersecting both rectangles, without scaling,
        converting to imageInfo().colorType() and imageInfo().alphaType() if required.

        Pixels are writable when SkBaseDevice is raster, or backed by a GPU.
        Pixels are not writable when SkCanvas is returned by SkDocument::beginPage,
        returned by SkPictureRecorder::beginRecording, or SkCanvas is the base of a utility
        class like DebugCanvas.

        Pixel values are converted only if SkColorType and SkAlphaType
        do not match. Only pixels within both source and destination rectangles
        are copied. SkCanvas pixels outside SkRect intersection are unchanged.

        Pass negative values for x or y to offset pixels to the left or
        above SkCanvas pixels.

        Does not copy, and returns false if:
        - Source and destination rectangles do not intersect.
        - bitmap does not have allocated pixels.
        - bitmap pixels could not be converted to SkCanvas imageInfo().colorType() or
        imageInfo().alphaType().
        - SkCanvas pixels are not writable; for instance, SkCanvas is document based.
        - bitmap pixels are inaccessible; for instance, bitmap wraps a texture.

        @param bitmap  contains pixels copied to SkCanvas
        @param x       offset into SkCanvas writable pixels on x-axis; may be negative
        @param y       offset into SkCanvas writable pixels on y-axis; may be negative
        @return        true if pixels were written to SkCanvas

        example: https://fiddle.skia.org/c/@Canvas_writePixels_2
        example: https://fiddle.skia.org/c/@State_Stack_a
        example: https://fiddle.skia.org/c/@State_Stack_b
    */
    #[must_use]
    pub fn write_pixels_from_bitmap(&mut self, bitmap: &Bitmap, offset: impl Into<IPoint>) -> bool {
        let offset = offset.into();
        unsafe {
            self.native_mut()
                .writePixels1(bitmap.native(), offset.x, offset.y)
        }
    }

    /** Saves SkMatrix and clip.
        Calling restore() discards changes to SkMatrix and clip,
        restoring the SkMatrix and clip to their state when save() was called.

        SkMatrix may be changed by translate(), scale(), rotate(), skew(), concat(), setMatrix(),
        and resetMatrix(). Clip may be changed by clipRect(), clipRRect(), clipPath(), clipRegion().

        Saved SkCanvas state is put on a stack; multiple calls to save() should be balance
        by an equal number of calls to restore().

        Call restoreToCount() with result to restore this and subsequent saves.

        @return  depth of saved stack

        example: https://fiddle.skia.org/c/@Canvas_save
    */
    pub fn save(&mut self) -> usize {
        unsafe { self.native_mut().save().try_into().unwrap() }
    }

    // The save_layer(bounds, paint) variants have been replaced by SaveLayerRec.

    /** Saves SkMatrix and clip, and allocates SkBitmap for subsequent drawing.

        Calling restore() discards changes to SkMatrix and clip,
        and blends layer with alpha opacity onto prior layer.

        SkMatrix may be changed by translate(), scale(), rotate(), skew(), concat(),
        setMatrix(), and resetMatrix(). Clip may be changed by clipRect(), clipRRect(),
        clipPath(), clipRegion().

        SkRect bounds suggests but does not define layer size. To clip drawing to
        a specific rectangle, use clipRect().

        alpha of zero is fully transparent, 255 is fully opaque.

        Call restoreToCount() with returned value to restore this and subsequent saves.

        @param bounds  hint to limit the size of layer; may be nullptr
        @param alpha   opacity of layer
        @return        depth of saved stack

        example: https://fiddle.skia.org/c/@Canvas_saveLayerAlpha
    */
    pub fn save_layer_alpha(&mut self, bounds: impl Into<Option<Rect>>, alpha: u8cpu) -> usize {
        unsafe {
            self.native_mut()
                .saveLayerAlpha(bounds.into().native().as_ptr_or_null(), alpha)
        }
        .try_into()
        .unwrap()
    }

    /** Saves SkMatrix and clip, and allocates SkBitmap for subsequent drawing.

        Calling restore() discards changes to SkMatrix and clip,
        and blends SkBitmap with alpha opacity onto the prior layer.

        SkMatrix may be changed by translate(), scale(), rotate(), skew(), concat(),
        setMatrix(), and resetMatrix(). Clip may be changed by clipRect(), clipRRect(),
        clipPath(), clipRegion().

        SaveLayerRec contains the state used to create the layer.

        Call restoreToCount() with returned value to restore this and subsequent saves.

        @param layerRec  layer state
        @return          depth of save state stack before this call was made.

        example: https://fiddle.skia.org/c/@Canvas_saveLayer_3
    */
    pub fn save_layer(&mut self, layer_rec: &SaveLayerRec) -> usize {
        unsafe { self.native_mut().saveLayer1(layer_rec.native()) }
            .try_into()
            .unwrap()
    }

    /** Removes changes to SkMatrix and clip since SkCanvas state was
        last saved. The state is removed from the stack.

        Does nothing if the stack is empty.

        example: https://fiddle.skia.org/c/@AutoCanvasRestore_restore

        example: https://fiddle.skia.org/c/@Canvas_restore
    */
    pub fn restore(&mut self) -> &mut Self {
        unsafe { self.native_mut().restore() };
        self
    }

    /** Returns the number of saved states, each containing: SkMatrix and clip.
        Equals the number of save() calls less the number of restore() calls plus one.
        The save count of a new canvas is one.

        @return  depth of save state stack

        example: https://fiddle.skia.org/c/@Canvas_getSaveCount
    */
    pub fn save_count(&self) -> usize {
        unsafe { self.native().getSaveCount() }.try_into().unwrap()
    }

    /** Restores state to SkMatrix and clip values when save(), saveLayer(),
        saveLayerPreserveLCDTextRequests(), or saveLayerAlpha() returned saveCount.

        Does nothing if saveCount is greater than state stack count.
        Restores state to initial values if saveCount is less than or equal to one.

        @param saveCount  depth of state stack to restore

        example: https://fiddle.skia.org/c/@Canvas_restoreToCount
    */
    pub fn restore_to_count(&mut self, count: usize) -> &mut Self {
        unsafe { self.native_mut().restoreToCount(count.try_into().unwrap()) }
        self
    }

    /** Translates SkMatrix by dx along the x-axis and dy along the y-axis.

        Mathematically, replaces SkMatrix with a translation matrix
        premultiplied with SkMatrix.

        This has the effect of moving the drawing by (dx, dy) before transforming
        the result with SkMatrix.

        @param dx  distance to translate on x-axis
        @param dy  distance to translate on y-axis

        example: https://fiddle.skia.org/c/@Canvas_translate
    */
    pub fn translate(&mut self, d: impl Into<Vector>) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().translate(d.x, d.y) }
        self
    }

    /** Scales SkMatrix by sx on the x-axis and sy on the y-axis.

        Mathematically, replaces SkMatrix with a scale matrix
        premultiplied with SkMatrix.

        This has the effect of scaling the drawing by (sx, sy) before transforming
        the result with SkMatrix.

        @param sx  amount to scale on x-axis
        @param sy  amount to scale on y-axis

        example: https://fiddle.skia.org/c/@Canvas_scale
    */
    pub fn scale(&mut self, (sx, sy): (scalar, scalar)) -> &mut Self {
        unsafe { self.native_mut().scale(sx, sy) }
        self
    }

    /** Rotates SkMatrix by degrees about a point at (px, py). Positive degrees rotates
        clockwise.

        Mathematically, constructs a rotation matrix; premultiplies the rotation matrix by
        a translation matrix; then replaces SkMatrix with the resulting matrix
        premultiplied with SkMatrix.

        This has the effect of rotating the drawing about a given point before
        transforming the result with SkMatrix.

        @param degrees  amount to rotate, in degrees
        @param px       x-axis value of the point to rotate about
        @param py       y-axis value of the point to rotate about

        example: https://fiddle.skia.org/c/@Canvas_rotate_2
    */
    pub fn rotate(&mut self, degrees: scalar, point: Option<Point>) -> &mut Self {
        match point {
            Some(point) => unsafe { self.native_mut().rotate1(degrees, point.x, point.y) },
            None => unsafe { self.native_mut().rotate(degrees) },
        }
        self
    }

    /** Skews SkMatrix by sx on the x-axis and sy on the y-axis. A positive value of sx
        skews the drawing right as y-axis values increase; a positive value of sy skews
        the drawing down as x-axis values increase.

        Mathematically, replaces SkMatrix with a skew matrix premultiplied with SkMatrix.

        This has the effect of skewing the drawing by (sx, sy) before transforming
        the result with SkMatrix.

        @param sx  amount to skew on x-axis
        @param sy  amount to skew on y-axis

        example: https://fiddle.skia.org/c/@Canvas_skew
    */
    pub fn skew(&mut self, (sx, sy): (scalar, scalar)) -> &mut Self {
        unsafe { self.native_mut().skew(sx, sy) }
        self
    }

    /** Replaces SkMatrix with matrix premultiplied with existing SkMatrix.

        This has the effect of transforming the drawn geometry by matrix, before
        transforming the result with existing SkMatrix.

        @param matrix  matrix to premultiply with existing SkMatrix

        example: https://fiddle.skia.org/c/@Canvas_concat
    */
    pub fn concat(&mut self, matrix: &Matrix) -> &mut Self {
        unsafe { self.native_mut().concat(matrix.native()) }
        self
    }

    // TODO: markCTM
    // TODO: findMarkedCTM

    pub fn concat_44(&mut self, m: &M44) -> &mut Self {
        unsafe { self.native_mut().concat1(m.native()) }
        self
    }

    /** Replaces SkMatrix with matrix.
        Unlike concat(), any prior matrix state is overwritten.

        @param matrix  matrix to copy, replacing existing SkMatrix

        example: https://fiddle.skia.org/c/@Canvas_setMatrix
    */
    pub fn set_matrix(&mut self, matrix: &M44) -> &mut Self {
        unsafe { self.native_mut().setMatrix(matrix.native()) }
        self
    }

    /** Sets SkMatrix to the identity matrix.
        Any prior matrix state is overwritten.

        example: https://fiddle.skia.org/c/@Canvas_resetMatrix
    */
    pub fn reset_matrix(&mut self) -> &mut Self {
        unsafe { self.native_mut().resetMatrix() }
        self
    }

    /** Replaces clip with the intersection or difference of clip and rect,
        with an aliased or anti-aliased clip edge. rect is transformed by SkMatrix
        before it is combined with clip.

        @param rect         SkRect to combine with clip
        @param op           SkClipOp to apply to clip
        @param doAntiAlias  true if clip is to be anti-aliased

        example: https://fiddle.skia.org/c/@Canvas_clipRect
    */
    pub fn clip_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        op: impl Into<Option<ClipOp>>,
        do_anti_alias: impl Into<Option<bool>>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().clipRect(
                rect.as_ref().native(),
                op.into().unwrap_or_default(),
                do_anti_alias.into().unwrap_or_default(),
            )
        }
        self
    }

    pub fn clip_irect(
        &mut self,
        irect: impl AsRef<IRect>,
        op: impl Into<Option<ClipOp>>,
    ) -> &mut Self {
        let r = Rect::from(*irect.as_ref());
        self.clip_rect(r, op, false)
    }

    /** Replaces clip with the intersection or difference of clip and rrect,
        with an aliased or anti-aliased clip edge.
        rrect is transformed by SkMatrix
        before it is combined with clip.

        @param rrect        SkRRect to combine with clip
        @param op           SkClipOp to apply to clip
        @param doAntiAlias  true if clip is to be anti-aliased

        example: https://fiddle.skia.org/c/@Canvas_clipRRect
    */
    pub fn clip_rrect(
        &mut self,
        rrect: impl AsRef<RRect>,
        op: impl Into<Option<ClipOp>>,
        do_anti_alias: impl Into<Option<bool>>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().clipRRect(
                rrect.as_ref().native(),
                op.into().unwrap_or_default(),
                do_anti_alias.into().unwrap_or_default(),
            )
        }
        self
    }

    /** Replaces clip with the intersection or difference of clip and path,
        with an aliased or anti-aliased clip edge. SkPath::FillType determines if path
        describes the area inside or outside its contours; and if path contour overlaps
        itself or another path contour, whether the overlaps form part of the area.
        path is transformed by SkMatrix before it is combined with clip.

        @param path         SkPath to combine with clip
        @param op           SkClipOp to apply to clip
        @param doAntiAlias  true if clip is to be anti-aliased

        example: https://fiddle.skia.org/c/@Canvas_clipPath
    */
    pub fn clip_path(
        &mut self,
        path: &Path,
        op: impl Into<Option<ClipOp>>,
        do_anti_alias: impl Into<Option<bool>>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().clipPath(
                path.native(),
                op.into().unwrap_or_default(),
                do_anti_alias.into().unwrap_or_default(),
            )
        }
        self
    }

    pub fn clip_shader(
        &mut self,
        shader: impl Into<Shader>,
        op: impl Into<Option<ClipOp>>,
    ) -> &mut Self {
        unsafe {
            sb::C_SkCanvas_clipShader(
                self.native_mut(),
                shader.into().into_ptr(),
                op.into().unwrap_or(ClipOp::Intersect),
            )
        }
        self
    }

    /** Replaces clip with the intersection or difference of clip and SkRegion deviceRgn.
        Resulting clip is aliased; pixels are fully contained by the clip.
        deviceRgn is unaffected by SkMatrix.

        @param deviceRgn  SkRegion to combine with clip
        @param op         SkClipOp to apply to clip

        example: https://fiddle.skia.org/c/@Canvas_clipRegion
    */
    pub fn clip_region(&mut self, device_rgn: &Region, op: impl Into<Option<ClipOp>>) -> &mut Self {
        unsafe {
            self.native_mut()
                .clipRegion(device_rgn.native(), op.into().unwrap_or_default())
        }
        self
    }

    // quickReject() functions are implemented as a trait.

    /** Returns bounds of clip, transformed by inverse of SkMatrix. If clip is empty,
        return SkRect::MakeEmpty, where all SkRect sides equal zero.

        SkRect returned is outset by one to account for partial pixel coverage if clip
        is anti-aliased.

        @return  bounds of clip in local coordinates

        example: https://fiddle.skia.org/c/@Canvas_getLocalClipBounds
    */
    pub fn local_clip_bounds(&self) -> Option<Rect> {
        let r = Rect::from_native_c(unsafe { sb::C_SkCanvas_getLocalClipBounds(self.native()) });
        r.is_empty().if_false_some(r)
    }

    /** Returns SkIRect bounds of clip, unaffected by SkMatrix. If clip is empty,
        return SkRect::MakeEmpty, where all SkRect sides equal zero.

        Unlike getLocalClipBounds(), returned SkIRect is not outset.

        @return  bounds of clip in SkBaseDevice coordinates

        example: https://fiddle.skia.org/c/@Canvas_getDeviceClipBounds
    */
    pub fn device_clip_bounds(&self) -> Option<IRect> {
        let r = IRect::from_native_c(unsafe { sb::C_SkCanvas_getDeviceClipBounds(self.native()) });
        r.is_empty().if_false_some(r)
    }

    /** Fills clip with color color.
        mode determines how ARGB is combined with destination.

        @param color  SkColor4f representing unpremultiplied color.
        @param mode   SkBlendMode used to combine source color and destination
    */
    pub fn draw_color(
        &mut self,
        color: impl Into<Color4f>,
        mode: impl Into<Option<BlendMode>>,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawColor(&color.into().into_native(), mode.into().unwrap_or_default())
        }
        self
    }

    /** Fills clip with color color using SkBlendMode::kSrc.
        This has the effect of replacing all pixels contained by clip with color.

        @param color  SkColor4f representing unpremultiplied color.
    */
    pub fn clear(&mut self, color: impl Into<Color4f>) -> &mut Self {
        self.draw_color(color, BlendMode::Src)
    }

    /** Makes SkCanvas contents undefined. Subsequent calls that read SkCanvas pixels,
        such as drawing with SkBlendMode, return undefined results. discard() does
        not change clip or SkMatrix.

        discard() may do nothing, depending on the implementation of SkSurface or SkBaseDevice
        that created SkCanvas.

        discard() allows optimized performance on subsequent draws by removing
        cached data associated with SkSurface or SkBaseDevice.
        It is not necessary to call discard() once done with SkCanvas;
        any cached data is deleted when owning SkSurface or SkBaseDevice is deleted.
    */
    pub fn discard(&mut self) -> &mut Self {
        unsafe { sb::C_SkCanvas_discard(self.native_mut()) }
        self
    }

    /** Fills clip with SkPaint paint. SkPaint components, SkShader,
        SkColorFilter, SkImageFilter, and SkBlendMode affect drawing;
        SkMaskFilter and SkPathEffect in paint are ignored.

        @param paint  graphics state used to fill SkCanvas

        example: https://fiddle.skia.org/c/@Canvas_drawPaint
    */
    pub fn draw_paint(&mut self, paint: &Paint) -> &mut Self {
        unsafe { self.native_mut().drawPaint(paint.native()) }
        self
    }

    /** Draws pts using clip, SkMatrix and SkPaint paint.
        count is the number of points; if count is less than one, has no effect.
        mode may be one of: kPoints_PointMode, kLines_PointMode, or kPolygon_PointMode.

        If mode is kPoints_PointMode, the shape of point drawn depends on paint
        SkPaint::Cap. If paint is set to SkPaint::kRound_Cap, each point draws a
        circle of diameter SkPaint stroke width. If paint is set to SkPaint::kSquare_Cap
        or SkPaint::kButt_Cap, each point draws a square of width and height
        SkPaint stroke width.

        If mode is kLines_PointMode, each pair of points draws a line segment.
        One line is drawn for every two points; each point is used once. If count is odd,
        the final point is ignored.

        If mode is kPolygon_PointMode, each adjacent pair of points draws a line segment.
        count minus one lines are drawn; the first and last point are used once.

        Each line segment respects paint SkPaint::Cap and SkPaint stroke width.
        SkPaint::Style is ignored, as if were set to SkPaint::kStroke_Style.

        Always draws each element one at a time; is not affected by
        SkPaint::Join, and unlike drawPath(), does not create a mask from all points
        and lines before drawing.

        @param mode   whether pts draws points or lines
        @param count  number of points in the array
        @param pts    array of points to draw
        @param paint  stroke, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawPoints
    */
    pub fn draw_points(&mut self, mode: PointMode, pts: &[Point], paint: &Paint) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawPoints(mode, pts.len(), pts.native().as_ptr(), paint.native())
        }
        self
    }

    /** Draws point p using clip, SkMatrix and SkPaint paint.

        The shape of point drawn depends on paint SkPaint::Cap.
        If paint is set to SkPaint::kRound_Cap, draw a circle of diameter
        SkPaint stroke width. If paint is set to SkPaint::kSquare_Cap or SkPaint::kButt_Cap,
        draw a square of width and height SkPaint stroke width.
        SkPaint::Style is ignored, as if were set to SkPaint::kStroke_Style.

        @param p      top-left edge of circle or square
        @param paint  stroke, blend, color, and so on, used to draw
    */
    pub fn draw_point(&mut self, p: impl Into<Point>, paint: &Paint) -> &mut Self {
        let p = p.into();
        unsafe { self.native_mut().drawPoint(p.x, p.y, paint.native()) }
        self
    }

    /** Draws line segment from p0 to p1 using clip, SkMatrix, and SkPaint paint.
        In paint: SkPaint stroke width describes the line thickness;
        SkPaint::Cap draws the end rounded or square;
        SkPaint::Style is ignored, as if were set to SkPaint::kStroke_Style.

        @param p0     start of line segment
        @param p1     end of line segment
        @param paint  stroke, blend, color, and so on, used to draw
    */
    pub fn draw_line(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        paint: &Paint,
    ) -> &mut Self {
        let (p1, p2) = (p1.into(), p2.into());
        unsafe {
            self.native_mut()
                .drawLine(p1.x, p1.y, p2.x, p2.y, paint.native())
        }
        self
    }

    /** Draws SkRect rect using clip, SkMatrix, and SkPaint paint.
        In paint: SkPaint::Style determines if rectangle is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness, and
        SkPaint::Join draws the corners rounded or square.

        @param rect   rectangle to draw
        @param paint  stroke or fill, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawRect
    */
    pub fn draw_rect(&mut self, rect: impl AsRef<Rect>, paint: &Paint) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawRect(rect.as_ref().native(), paint.native())
        }
        self
    }

    /** Draws SkIRect rect using clip, SkMatrix, and SkPaint paint.
        In paint: SkPaint::Style determines if rectangle is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness, and
        SkPaint::Join draws the corners rounded or square.

        @param rect   rectangle to draw
        @param paint  stroke or fill, blend, color, and so on, used to draw
    */
    pub fn draw_irect(&mut self, rect: impl AsRef<IRect>, paint: &Paint) -> &mut Self {
        self.draw_rect(Rect::from(*rect.as_ref()), paint)
    }

    /** Draws SkRegion region using clip, SkMatrix, and SkPaint paint.
        In paint: SkPaint::Style determines if rectangle is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness, and
        SkPaint::Join draws the corners rounded or square.

        @param region  region to draw
        @param paint   SkPaint stroke or fill, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawRegion
    */
    pub fn draw_region(&mut self, region: &Region, paint: &Paint) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawRegion(region.native(), paint.native())
        }
        self
    }

    /** Draws oval oval using clip, SkMatrix, and SkPaint.
        In paint: SkPaint::Style determines if oval is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness.

        @param oval   SkRect bounds of oval
        @param paint  SkPaint stroke or fill, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawOval
    */
    pub fn draw_oval(&mut self, oval: impl AsRef<Rect>, paint: &Paint) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawOval(oval.as_ref().native(), paint.native())
        }
        self
    }

    /** Draws SkRRect rrect using clip, SkMatrix, and SkPaint paint.
        In paint: SkPaint::Style determines if rrect is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness.

        rrect may represent a rectangle, circle, oval, uniformly rounded rectangle, or
        may have any combination of positive non-square radii for the four corners.

        @param rrect  SkRRect with up to eight corner radii to draw
        @param paint  SkPaint stroke or fill, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawRRect
    */
    pub fn draw_rrect(&mut self, rrect: impl AsRef<RRect>, paint: &Paint) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawRRect(rrect.as_ref().native(), paint.native())
        }
        self
    }

    /** Draws SkRRect outer and inner
        using clip, SkMatrix, and SkPaint paint.
        outer must contain inner or the drawing is undefined.
        In paint: SkPaint::Style determines if SkRRect is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness.
        If stroked and SkRRect corner has zero length radii, SkPaint::Join can
        draw corners rounded or square.

        GPU-backed platforms optimize drawing when both outer and inner are
        concave and outer contains inner. These platforms may not be able to draw
        SkPath built with identical data as fast.

        @param outer  SkRRect outer bounds to draw
        @param inner  SkRRect inner bounds to draw
        @param paint  SkPaint stroke or fill, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawDRRect_a
        example: https://fiddle.skia.org/c/@Canvas_drawDRRect_b
    */
    pub fn draw_drrect(
        &mut self,
        outer: impl AsRef<RRect>,
        inner: impl AsRef<RRect>,
        paint: &Paint,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawDRRect(
                outer.as_ref().native(),
                inner.as_ref().native(),
                paint.native(),
            )
        }
        self
    }

    /** Draws circle at center with radius using clip, SkMatrix, and SkPaint paint.
        If radius is zero or less, nothing is drawn.
        In paint: SkPaint::Style determines if circle is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness.

        @param center  circle center
        @param radius  half the diameter of circle
        @param paint   SkPaint stroke or fill, blend, color, and so on, used to draw
    */
    pub fn draw_circle(
        &mut self,
        center: impl Into<Point>,
        radius: scalar,
        paint: &Paint,
    ) -> &mut Self {
        let center = center.into();
        unsafe {
            self.native_mut()
                .drawCircle(center.x, center.y, radius, paint.native())
        }
        self
    }

    /** Draws arc using clip, SkMatrix, and SkPaint paint.

        Arc is part of oval bounded by oval, sweeping from startAngle to startAngle plus
        sweepAngle. startAngle and sweepAngle are in degrees.

        startAngle of zero places start point at the right middle edge of oval.
        A positive sweepAngle places arc end point clockwise from start point;
        a negative sweepAngle places arc end point counterclockwise from start point.
        sweepAngle may exceed 360 degrees, a full circle.
        If useCenter is true, draw a wedge that includes lines from oval
        center to arc end points. If useCenter is false, draw arc between end points.

        If SkRect oval is empty or sweepAngle is zero, nothing is drawn.

        @param oval        SkRect bounds of oval containing arc to draw
        @param startAngle  angle in degrees where arc begins
        @param sweepAngle  sweep angle in degrees; positive is clockwise
        @param useCenter   if true, include the center of the oval
        @param paint       SkPaint stroke or fill, blend, color, and so on, used to draw
    */
    pub fn draw_arc(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle: scalar,
        sweep_angle: scalar,
        use_center: bool,
        paint: &Paint,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawArc(
                oval.as_ref().native(),
                start_angle,
                sweep_angle,
                use_center,
                paint.native(),
            )
        }
        self
    }

    /** Draws SkRRect bounded by SkRect rect, with corner radii (rx, ry) using clip,
        SkMatrix, and SkPaint paint.

        In paint: SkPaint::Style determines if SkRRect is stroked or filled;
        if stroked, SkPaint stroke width describes the line thickness.
        If rx or ry are less than zero, they are treated as if they are zero.
        If rx plus ry exceeds rect width or rect height, radii are scaled down to fit.
        If rx and ry are zero, SkRRect is drawn as SkRect and if stroked is affected by
        SkPaint::Join.

        @param rect   SkRect bounds of SkRRect to draw
        @param rx     axis length on x-axis of oval describing rounded corners
        @param ry     axis length on y-axis of oval describing rounded corners
        @param paint  stroke, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawRoundRect
    */
    pub fn draw_round_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        rx: scalar,
        ry: scalar,
        paint: &Paint,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .drawRoundRect(rect.as_ref().native(), rx, ry, paint.native())
        }
        self
    }

    /** Draws SkPath path using clip, SkMatrix, and SkPaint paint.
        SkPath contains an array of path contour, each of which may be open or closed.

        In paint: SkPaint::Style determines if SkRRect is stroked or filled:
        if filled, SkPath::FillType determines whether path contour describes inside or
        outside of fill; if stroked, SkPaint stroke width describes the line thickness,
        SkPaint::Cap describes line ends, and SkPaint::Join describes how
        corners are drawn.

        @param path   SkPath to draw
        @param paint  stroke, blend, color, and so on, used to draw

        example: https://fiddle.skia.org/c/@Canvas_drawPath
    */
    pub fn draw_path(&mut self, path: &Path, paint: &Paint) -> &mut Self {
        unsafe { self.native_mut().drawPath(path.native(), paint.native()) }
        self
    }

    pub fn draw_image(
        &mut self,
        image: impl AsRef<Image>,
        left_top: impl Into<Point>,
        paint: Option<&Paint>,
    ) -> &mut Self {
        let left_top = left_top.into();
        self.draw_image_with_sampling_options(image, left_top, SamplingOptions::default(), paint)
    }

    pub fn draw_image_rect(
        &mut self,
        image: impl AsRef<Image>,
        src: Option<(&Rect, SrcRectConstraint)>,
        dst: impl AsRef<Rect>,
        paint: &Paint,
    ) -> &mut Self {
        self.draw_image_rect_with_sampling_options(
            image,
            src,
            dst,
            SamplingOptions::default(),
            paint,
        )
    }

    pub fn draw_image_with_sampling_options(
        &mut self,
        image: impl AsRef<Image>,
        left_top: impl Into<Point>,
        sampling: impl Into<SamplingOptions>,
        paint: Option<&Paint>,
    ) -> &mut Self {
        let left_top = left_top.into();
        unsafe {
            self.native_mut().drawImage(
                image.as_ref().native(),
                left_top.x,
                left_top.y,
                sampling.into().native(),
                paint.native_ptr_or_null(),
            )
        }
        self
    }

    pub fn draw_image_rect_with_sampling_options(
        &mut self,
        image: impl AsRef<Image>,
        src: Option<(&Rect, SrcRectConstraint)>,
        dst: impl AsRef<Rect>,
        sampling: impl Into<SamplingOptions>,
        paint: &Paint,
    ) -> &mut Self {
        let sampling = sampling.into();
        match src {
            Some((src, constraint)) => unsafe {
                self.native_mut().drawImageRect(
                    image.as_ref().native(),
                    src.native(),
                    dst.as_ref().native(),
                    sampling.native(),
                    paint.native(),
                    constraint,
                )
            },
            None => unsafe {
                self.native_mut().drawImageRect1(
                    image.as_ref().native(),
                    dst.as_ref().native(),
                    sampling.native(),
                    paint.native(),
                )
            },
        }
        self
    }

    /** Draws SkImage image stretched proportionally to fit into SkRect dst.
        SkIRect center divides the image into nine sections: four sides, four corners, and
        the center. Corners are unmodified or scaled down proportionately if their sides
        are larger than dst; center and four sides are scaled to fit remaining space, if any.

        Additionally transform draw using clip, SkMatrix, and optional SkPaint paint.

        If SkPaint paint is supplied, apply SkColorFilter, alpha, SkImageFilter, and
        SkBlendMode. If image is kAlpha_8_SkColorType, apply SkShader.
        If paint contains SkMaskFilter, generate mask from image bounds.
        Any SkMaskFilter on paint is ignored as is paint anti-aliasing state.

        If generated mask extends beyond image bounds, replicate image edge colors, just
        as SkShader made from SkImage::makeShader with SkShader::kClamp_TileMode set
        replicates the image edge color when it samples outside of its bounds.

        @param image   SkImage containing pixels, dimensions, and format
        @param center  SkIRect edge of image corners and sides
        @param dst     destination SkRect of image to draw to
        @param filter  what technique to use when sampling the image
        @param paint   SkPaint containing SkBlendMode, SkColorFilter, SkImageFilter,
                       and so on; or nullptr
    */
    pub fn draw_image_nine(
        &mut self,
        image: impl AsRef<Image>,
        center: impl AsRef<IRect>,
        dst: impl AsRef<Rect>,
        filter_mode: FilterMode,
        paint: Option<&Paint>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawImageNine(
                image.as_ref().native(),
                center.as_ref().native(),
                dst.as_ref().native(),
                filter_mode,
                paint.native_ptr_or_null(),
            )
        }
        self
    }

    /** Draws SkImage image stretched proportionally to fit into SkRect dst.

        SkCanvas::Lattice lattice divides image into a rectangular grid.
        Each intersection of an even-numbered row and column is fixed;
        fixed lattice elements never scale larger than their initial
        size and shrink proportionately when all fixed elements exceed the bitmap
        dimension. All other grid elements scale to fill the available space, if any.

        Additionally transform draw using clip, SkMatrix, and optional SkPaint paint.

        If SkPaint paint is supplied, apply SkColorFilter, alpha, SkImageFilter, and
        SkBlendMode. If image is kAlpha_8_SkColorType, apply SkShader.
        If paint contains SkMaskFilter, generate mask from image bounds.
        Any SkMaskFilter on paint is ignored as is paint anti-aliasing state.

        If generated mask extends beyond bitmap bounds, replicate bitmap edge colors,
        just as SkShader made from SkShader::MakeBitmapShader with
        SkShader::kClamp_TileMode set replicates the bitmap edge color when it samples
        outside of its bounds.

        @param image    SkImage containing pixels, dimensions, and format
        @param lattice  division of bitmap into fixed and variable rectangles
        @param dst      destination SkRect of image to draw to
        @param filter   what technique to use when sampling the image
        @param paint    SkPaint containing SkBlendMode, SkColorFilter, SkImageFilter,
                        and so on; or nullptr
    */
    pub fn draw_image_lattice(
        &mut self,
        image: impl AsRef<Image>,
        lattice: &Lattice,
        filter: FilterMode,
        dst: impl AsRef<Rect>,
        paint: Option<&Paint>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawImageLattice(
                image.as_ref().native(),
                &lattice.native().native,
                dst.as_ref().native(),
                filter,
                paint.native_ptr_or_null(),
            )
        }
        self
    }

    // TODO: drawSimpleText?

    /** Draws SkString, with origin at (x, y), using clip, SkMatrix, SkFont font,
        and SkPaint paint.

        This function uses the default character-to-glyph mapping from the
        SkTypeface in font.  It does not perform typeface fallback for
        characters not found in the SkTypeface.  It does not perform kerning;
        glyphs are positioned based on their default advances.

        SkString str is encoded as UTF-8.

        Text size is affected by SkMatrix and SkFont text size. Default text
        size is 12 point.

        All elements of paint: SkPathEffect, SkMaskFilter, SkShader,
        SkColorFilter, and SkImageFilter; apply to text. By
        default, draws filled black glyphs.

        @param str     character code points drawn,
                       ending with a char value of zero
        @param x       start of string on x-axis
        @param y       start of string on y-axis
        @param font    typeface, text size and so, used to describe the text
        @param paint   blend, color, and so on, used to draw
    */
    pub fn draw_str(
        &mut self,
        str: impl AsRef<str>,
        origin: impl Into<Point>,
        font: &Font,
        paint: &Paint,
    ) -> &mut Self {
        // rust specific, based on drawSimpleText with fixed UTF8 encoding,
        // implementation is similar to Font's *_str methods.
        let origin = origin.into();
        let bytes = str.as_ref().as_bytes();
        unsafe {
            self.native_mut().drawSimpleText(
                bytes.as_ptr() as _,
                bytes.len(),
                TextEncoding::UTF8.into_native(),
                origin.x,
                origin.y,
                font.native(),
                paint.native(),
            )
        }
        self
    }

    /** Draws SkTextBlob blob at (x, y), using clip, SkMatrix, and SkPaint paint.

        blob contains glyphs, their positions, and paint attributes specific to text:
        SkTypeface, SkPaint text size, SkPaint text scale x,
        SkPaint text skew x, SkPaint::Align, SkPaint::Hinting, anti-alias, SkPaint fake bold,
        SkPaint font embedded bitmaps, SkPaint full hinting spacing, LCD text, SkPaint linear text,
        and SkPaint subpixel text.

        SkTextEncoding must be set to SkTextEncoding::kGlyphID.

        Elements of paint: SkPathEffect, SkMaskFilter, SkShader, SkColorFilter,
        and SkImageFilter; apply to blob.

        @param blob   glyphs, positions, and their paints' text size, typeface, and so on
        @param x      horizontal offset applied to blob
        @param y      vertical offset applied to blob
        @param paint  blend, color, stroking, and so on, used to draw
    */
    pub fn draw_text_blob(
        &mut self,
        blob: impl AsRef<TextBlob>,
        origin: impl Into<Point>,
        paint: &Paint,
    ) -> &mut Self {
        let origin = origin.into();
        unsafe {
            self.native_mut().drawTextBlob(
                blob.as_ref().native(),
                origin.x,
                origin.y,
                paint.native(),
            )
        }
        self
    }

    /** Draws SkPicture picture, using clip and SkMatrix; transforming picture with
        SkMatrix matrix, if provided; and use SkPaint paint alpha, SkColorFilter,
        SkImageFilter, and SkBlendMode, if provided.

        If paint is non-null, then the picture is always drawn into a temporary layer before
        actually landing on the canvas. Note that drawing into a layer can also change its
        appearance if there are any non-associative blendModes inside any of the pictures elements.

        @param picture  recorded drawing commands to play
        @param matrix   SkMatrix to rotate, scale, translate, and so on; may be nullptr
        @param paint    SkPaint to apply transparency, filtering, and so on; may be nullptr
    */
    pub fn draw_picture(
        &mut self,
        picture: impl AsRef<Picture>,
        matrix: Option<&Matrix>,
        paint: Option<&Paint>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawPicture(
                picture.as_ref().native(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
            )
        }
        self
    }

    /** Draws SkVertices vertices, a triangle mesh, using clip and SkMatrix.
        If paint contains an SkShader and vertices does not contain texCoords, the shader
        is mapped using the vertices' positions.

        If vertices colors are defined in vertices, and SkPaint paint contains SkShader,
        SkBlendMode mode combines vertices colors with SkShader.

        @param vertices  triangle mesh to draw
        @param mode      combines vertices colors with SkShader, if both are present
        @param paint     specifies the SkShader, used as SkVertices texture, may be nullptr

        example: https://fiddle.skia.org/c/@Canvas_drawVertices_2
    */
    pub fn draw_vertices(
        &mut self,
        vertices: &Vertices,
        mode: impl Into<Option<BlendMode>>,
        paint: &Paint,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawVertices(
                vertices.native(),
                mode.into().unwrap_or(BlendMode::Modulate),
                paint.native(),
            )
        }
        self
    }

    /** Draws a Coons patch: the interpolation of four cubics with shared corners,
        associating a color, and optionally a texture SkPoint, with each corner.

        Coons patch uses clip and SkMatrix, paint SkShader, SkColorFilter,
        alpha, SkImageFilter, and SkBlendMode. If SkShader is provided it is treated
        as Coons patch texture; SkBlendMode mode combines color colors and SkShader if
        both are provided.

        SkPoint array cubics specifies four SkPath cubic starting at the top-left corner,
        in clockwise order, sharing every fourth point. The last SkPath cubic ends at the
        first point.

        Color array color associates colors with corners in top-left, top-right,
        bottom-right, bottom-left order.

        If paint contains SkShader, SkPoint array texCoords maps SkShader as texture to
        corners in top-left, top-right, bottom-right, bottom-left order. If texCoords is
        nullptr, SkShader is mapped using positions (derived from cubics).

        @param cubics     SkPath cubic array, sharing common points
        @param colors     color array, one for each corner
        @param texCoords  SkPoint array of texture coordinates, mapping SkShader to corners;
                          may be nullptr
        @param mode       SkBlendMode for colors, and for SkShader if paint has one
        @param paint      SkShader, SkColorFilter, SkBlendMode, used to draw
    */
    pub fn draw_patch(
        &mut self,
        cubics: &[Point; 12],
        colors: &[Color; 4],
        tex_coords: &[Point; 4],
        mode: impl Into<Option<BlendMode>>,
        paint: &Paint,
    ) -> &mut Self {
        unsafe {
            self.native_mut().drawPatch(
                cubics.native().as_ptr(),
                colors.native().as_ptr(),
                tex_coords.native().as_ptr(),
                mode.into().unwrap_or(BlendMode::Modulate),
                paint.native(),
            )
        }
        self
    }

    // TODO: drawAtlas

    /** Draws SkDrawable drawable using clip and SkMatrix, concatenated with
        optional matrix.

        If SkCanvas has an asynchronous implementation, as is the case
        when it is recording into SkPicture, then drawable will be referenced,
        so that SkDrawable::draw() can be called when the operation is finalized. To force
        immediate drawing, call SkDrawable::draw() instead.

        @param drawable  custom struct encapsulating drawing commands
        @param matrix    transformation applied to drawing; may be nullptr

        example: https://fiddle.skia.org/c/@Canvas_drawDrawable
    */
    pub fn draw_drawable(&mut self, drawable: &mut Drawable, matrix: Option<&Matrix>) {
        unsafe {
            self.native_mut()
                .drawDrawable(drawable.native_mut(), matrix.native_ptr_or_null())
        }
    }

    /** Draws SkDrawable drawable using clip and SkMatrix, offset by (x, y).

        If SkCanvas has an asynchronous implementation, as is the case
        when it is recording into SkPicture, then drawable will be referenced,
        so that SkDrawable::draw() can be called when the operation is finalized. To force
        immediate drawing, call SkDrawable::draw() instead.

        @param drawable  custom struct encapsulating drawing commands
        @param x         offset into SkCanvas writable pixels on x-axis
        @param y         offset into SkCanvas writable pixels on y-axis

        example: https://fiddle.skia.org/c/@Canvas_drawDrawable_2
    */
    pub fn draw_drawable_at(&mut self, drawable: &mut Drawable, offset: impl Into<Point>) {
        let offset = offset.into();
        unsafe {
            self.native_mut()
                .drawDrawable1(drawable.native_mut(), offset.x, offset.y)
        }
    }

    /** Associates SkRect on SkCanvas when an annotation; a key-value pair, where the key is
        a null-terminated UTF-8 string, and optional value is stored as SkData.

        Only some canvas implementations, such as recording to SkPicture, or drawing to
        document PDF, use annotations.

        @param rect   SkRect extent of canvas to annotate
        @param key    string used for lookup
        @param value  data holding value stored in annotation
    */
    pub fn draw_annotation(
        &mut self,
        rect: impl AsRef<Rect>,
        key: &str,
        value: &Data,
    ) -> &mut Self {
        let key = CString::new(key).unwrap();
        unsafe {
            self.native_mut().drawAnnotation(
                rect.as_ref().native(),
                key.as_ptr(),
                value.native_mut_force(),
            )
        }
        self
    }

    /** Returns true if clip is empty; that is, nothing will draw.

        May do work when called; it should not be called
        more often than needed. However, once called, subsequent calls perform no
        work until clip changes.

        @return  true if clip is empty

        example: https://fiddle.skia.org/c/@Canvas_isClipEmpty
    */
    pub fn is_clip_empty(&self) -> bool {
        unsafe { sb::C_SkCanvas_isClipEmpty(self.native()) }
    }

    /** Returns true if clip is SkRect and not empty.
        Returns false if the clip is empty, or if it is not SkRect.

        @return  true if clip is SkRect and not empty

        example: https://fiddle.skia.org/c/@Canvas_isClipRect
    */
    pub fn is_clip_rect(&self) -> bool {
        unsafe { sb::C_SkCanvas_isClipRect(self.native()) }
    }

    /** Returns the current transform from local coordinates to the 'device', which for most
     *  purposes means pixels.
     *
     *  @return transformation from local coordinates to device / pixels.
     */
    pub fn local_to_device(&self) -> M44 {
        M44::construct(|m| unsafe { sb::C_SkCanvas_getLocalToDevice(self.native(), m) })
    }

    /**
     *  Throws away the 3rd row and column in the matrix, so be warned.
     */
    pub fn local_to_device_as_3x3(&self) -> Matrix {
        self.local_to_device().to_m33()
    }

    /** DEPRECATED
     *  Legacy version of getLocalToDevice(), which strips away any Z information, and
     *  just returns a 3x3 version.
     *
     *  @return 3x3 version of getLocalToDevice()
     *
     *  example: https://fiddle.skia.org/c/@Canvas_getTotalMatrix
     *  example: https://fiddle.skia.org/c/@Clip
     */
    #[deprecated(
        since = "0.38.0",
        note = "use local_to_device() or local_to_device_as_3x3() instead"
    )]
    pub fn total_matrix(&self) -> Matrix {
        let mut matrix = Matrix::default();
        unsafe { sb::C_SkCanvas_getTotalMatrix(self.native(), matrix.native_mut()) };
        matrix
    }

    //
    // internal helper
    //

    pub(crate) fn own_from_native_ptr<'lt>(native: *mut SkCanvas) -> Option<OwnedCanvas<'lt>> {
        if !native.is_null() {
            Some(OwnedCanvas::<'lt>(
                ptr::NonNull::new(Self::borrow_from_native_mut(unsafe { &mut *native })).unwrap(),
                PhantomData,
            ))
        } else {
            None
        }
    }

    pub(crate) fn borrow_from_native(native: &SkCanvas) -> &Self {
        unsafe { transmute_ref(native) }
    }

    pub(crate) fn borrow_from_native_mut(native: &mut SkCanvas) -> &mut Self {
        unsafe { transmute_ref_mut(native) }
    }
}

impl QuickReject<Rect> for Canvas {
    /** Returns true if SkRect rect, transformed by SkMatrix, can be quickly determined to be
        outside of clip. May return false even though rect is outside of clip.

        Use to check if an area to be drawn is clipped out, to skip subsequent draw calls.

        @param rect  SkRect to compare with clip
        @return      true if rect, transformed by SkMatrix, does not intersect clip

        example: https://fiddle.skia.org/c/@Canvas_quickReject
    */
    fn quick_reject(&self, other: &Rect) -> bool {
        unsafe { self.native().quickReject(other.native()) }
    }
}

impl QuickReject<Path> for Canvas {
    /** Returns true if path, transformed by SkMatrix, can be quickly determined to be
        outside of clip. May return false even though path is outside of clip.

        Use to check if an area to be drawn is clipped out, to skip subsequent draw calls.

        @param path  SkPath to compare with clip
        @return      true if path, transformed by SkMatrix, does not intersect clip

        example: https://fiddle.skia.org/c/@Canvas_quickReject_2
    */
    fn quick_reject(&self, other: &Path) -> bool {
        unsafe { self.native().quickReject1(other.native()) }
    }
}

pub trait SetMatrix {
    /// DEPRECATED -- use SkM44 version
    #[deprecated(since = "0.38.0", note = "Use M44 version")]
    fn set_matrix(&mut self, matrix: &Matrix) -> &mut Self;
}

impl SetMatrix for Canvas {
    /// DEPRECATED -- use SkM44 version
    fn set_matrix(&mut self, matrix: &Matrix) -> &mut Self {
        unsafe { self.native_mut().setMatrix1(matrix.native()) }
        self
    }
}

//
// Lattice
//

pub mod lattice {
    use crate::prelude::*;
    use crate::{Color, IRect};
    use skia_bindings as sb;
    use skia_bindings::SkCanvas_Lattice;
    use std::marker::PhantomData;

    /** \struct SkCanvas::Lattice
        SkCanvas::Lattice divides SkBitmap or SkImage into a rectangular grid.
        Grid entries on even columns and even rows are fixed; these entries are
        always drawn at their original size if the destination is large enough.
        If the destination side is too small to hold the fixed entries, all fixed
        entries are proportionately scaled down to fit.
        The grid entries not on even columns and rows are scaled to fit the
        remaining space, if any.
    */
    #[derive(Debug)]
    pub struct Lattice<'a> {
        /// x-axis values dividing bitmap
        pub x_divs: &'a [i32],
        /// y-axis values dividing bitmap
        pub y_divs: &'a [i32],
        /// array of fill types
        pub rect_types: Option<&'a [RectType]>,
        /// source bounds to draw from
        pub bounds: Option<IRect>,
        /// array of colors
        pub colors: Option<&'a [Color]>,
    }

    #[derive(Debug)]
    pub(crate) struct Ref<'a> {
        pub native: SkCanvas_Lattice,
        pd: PhantomData<&'a Lattice<'a>>,
    }

    impl<'a> Lattice<'a> {
        pub(crate) fn native(&self) -> Ref {
            if let Some(rect_types) = self.rect_types {
                let rect_count = (self.x_divs.len() + 1) * (self.y_divs.len() + 1);
                assert_eq!(rect_count, rect_types.len());
                // even though rect types may not include any FixedColor refs,
                // we expect the colors slice with a proper size here, this
                // saves us for going over the types array and looking for FixedColor
                // entries.
                assert_eq!(rect_count, self.colors.unwrap().len());
            }

            let native = SkCanvas_Lattice {
                fXDivs: self.x_divs.as_ptr(),
                fYDivs: self.y_divs.as_ptr(),
                fRectTypes: self.rect_types.as_ptr_or_null(),
                fXCount: self.x_divs.len().try_into().unwrap(),
                fYCount: self.y_divs.len().try_into().unwrap(),
                fBounds: self.bounds.native().as_ptr_or_null(),
                fColors: self.colors.native().as_ptr_or_null(),
            };
            Ref {
                native,
                pd: PhantomData,
            }
        }
    }

    /** \enum SkCanvas::Lattice::RectType
        Optional setting per rectangular grid entry to make it transparent,
        or to fill the grid entry with a color.
    */
    pub use sb::SkCanvas_Lattice_RectType as RectType;

    #[test]
    fn test_lattice_rect_type_naming() {
        let _ = RectType::FixedColor;
    }
}

/** \class SkAutoCanvasRestore
    Stack helper class calls SkCanvas::restoreToCount when SkAutoCanvasRestore
    goes out of scope. Use this to guarantee that the canvas is restored to a known
    state.
*/
#[derive(Debug)]
pub struct AutoRestoredCanvas<'a> {
    canvas: &'a mut Canvas,
    restore: SkAutoCanvasRestore,
}

impl<'a> Deref for AutoRestoredCanvas<'a> {
    type Target = Canvas;
    fn deref(&self) -> &Self::Target {
        self.canvas
    }
}

impl<'a> DerefMut for AutoRestoredCanvas<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.canvas
    }
}

impl<'a> NativeAccess<SkAutoCanvasRestore> for AutoRestoredCanvas<'a> {
    fn native(&self) -> &SkAutoCanvasRestore {
        &self.restore
    }

    fn native_mut(&mut self) -> &mut SkAutoCanvasRestore {
        &mut self.restore
    }
}

impl<'a> Drop for AutoRestoredCanvas<'a> {
    /** Restores SkCanvas to saved state. Destructor is called when container goes out of
        scope.
    */
    fn drop(&mut self) {
        unsafe { sb::C_SkAutoCanvasRestore_destruct(self.native_mut()) }
    }
}

impl<'a> AutoRestoredCanvas<'a> {
    /** Restores SkCanvas to saved state immediately. Subsequent calls and
        ~SkAutoCanvasRestore() have no effect.
    */
    pub fn restore(&mut self) {
        unsafe { sb::C_SkAutoCanvasRestore_restore(self.native_mut()) }
    }
}

pub enum AutoCanvasRestore {}

impl AutoCanvasRestore {
    // TODO: rename to save(), add a method to Canvas, perhaps named auto_restored()?
    /** Preserves SkCanvas::save() count. Optionally saves SkCanvas clip and SkCanvas matrix.

        @param canvas  SkCanvas to guard
        @param doSave  call SkCanvas::save()
        @return        utility to restore SkCanvas state on destructor
    */
    pub fn guard(canvas: &mut Canvas, do_save: bool) -> AutoRestoredCanvas {
        let restore = construct(|acr| unsafe {
            sb::C_SkAutoCanvasRestore_Construct(acr, canvas.native_mut(), do_save)
        });

        AutoRestoredCanvas { canvas, restore }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        canvas::SaveLayerFlags, canvas::SaveLayerRec, AlphaType, Canvas, ClipOp, Color, ColorType,
        ImageInfo, OwnedCanvas, Rect,
    };

    #[test]
    fn test_raster_direct_creation_and_clear_in_memory() {
        let info = ImageInfo::new((2, 2), ColorType::RGBA8888, AlphaType::Unpremul, None);
        assert_eq!(8, info.min_row_bytes());
        let mut bytes: [u8; 8 * 2] = Default::default();
        {
            let mut canvas = Canvas::from_raster_direct(&info, bytes.as_mut(), None, None).unwrap();
            canvas.clear(Color::RED);
        }

        assert_eq!(0xff, bytes[0]);
        assert_eq!(0x00, bytes[1]);
        assert_eq!(0x00, bytes[2]);
        assert_eq!(0xff, bytes[3]);
    }

    #[test]
    fn test_raster_direct_n32_creation_and_clear_in_memory() {
        let mut pixels: [u32; 4] = Default::default();
        {
            let mut canvas = Canvas::from_raster_direct_n32((2, 2), pixels.as_mut(), None).unwrap();
            canvas.clear(Color::RED);
        }

        // TODO: equals to 0xff0000ff on macOS, but why? Endianness should be the same.
        // assert_eq!(0xffff0000, pixels[0]);
    }

    #[test]
    fn test_empty_canvas_creation() {
        let canvas = OwnedCanvas::default();
        drop(canvas)
    }

    #[test]
    fn test_save_layer_rec_lifetimes() {
        let rect = Rect::default();
        {
            let _rec = SaveLayerRec::default()
                .flags(SaveLayerFlags::PRESERVE_LCD_TEXT)
                .bounds(&rect);
        }
    }

    #[test]
    fn test_make_surface() {
        let mut pixels: [u32; 4] = Default::default();
        let mut canvas = Canvas::from_raster_direct_n32((2, 2), pixels.as_mut(), None).unwrap();
        let ii = canvas.image_info();
        let mut surface = canvas.new_surface(&ii, None).unwrap();
        dbg!(&mut canvas as *mut _);
        drop(canvas);

        let canvas = surface.canvas();
        dbg!(canvas as *mut _);
        canvas.clear(Color::RED);
    }

    #[test]
    fn clip_options_overloads() {
        let mut c = OwnedCanvas::default();
        // do_anti_alias
        c.clip_rect(Rect::default(), None, true);
        // clip_op
        c.clip_rect(Rect::default(), ClipOp::Difference, None);
        // both
        c.clip_rect(Rect::default(), ClipOp::Difference, true);
    }

    /// Regression test for: https://github.com/rust-skia/rust-skia/issues/427
    #[test]
    fn test_local_and_device_clip_bounds() {
        let mut surface = crate::Surface::new_raster_n32_premul((100, 100)).unwrap();
        let _ = surface.canvas().device_clip_bounds();
        let _ = surface.canvas().local_clip_bounds();
        let _ = surface.canvas().local_to_device();
    }
}
