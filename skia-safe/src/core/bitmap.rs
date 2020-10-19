use crate::prelude::*;
use crate::{
    AlphaType, Color, ColorSpace, ColorType, IPoint, IRect, ISize, ImageInfo, Paint, PixelRef,
    Pixmap,
};
use crate::{Matrix, Shader, TileMode};
use skia_bindings as sb;
use skia_bindings::SkBitmap;
use std::{ffi, ptr};

pub type Bitmap = Handle<SkBitmap>;

impl NativeDrop for SkBitmap {
    fn drop(&mut self) {
        unsafe { sb::C_SkBitmap_destruct(self) }
    }
}

impl NativeClone for SkBitmap {
    fn clone(&self) -> Self {
        unsafe { SkBitmap::new1(self) }
    }
}

// TODO: implement Default?

impl Handle<SkBitmap> {
    pub fn new() -> Self {
        Self::construct(|bitmap| unsafe { sb::C_SkBitmap_Construct(bitmap) })
    }

    pub fn swap(&mut self, other: &mut Self) {
        unsafe { self.native_mut().swap(other.native_mut()) }
    }

    pub fn pixmap(&self) -> &Pixmap {
        Pixmap::from_native_ref(&self.native().fPixmap)
    }

    pub fn info(&self) -> &ImageInfo {
        self.pixmap().info()
    }

    pub fn width(&self) -> i32 {
        self.pixmap().width()
    }

    pub fn height(&self) -> i32 {
        self.pixmap().height()
    }

    pub fn color_type(&self) -> ColorType {
        self.pixmap().color_type()
    }

    pub fn alpha_type(&self) -> AlphaType {
        self.pixmap().alpha_type()
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.pixmap().color_space()
    }

    pub fn bytes_per_pixel(&self) -> usize {
        self.info().bytes_per_pixel()
    }

    pub fn row_bytes_as_pixels(&self) -> usize {
        self.pixmap().row_bytes_as_pixels()
    }

    pub fn shift_per_pixel(&self) -> usize {
        self.pixmap().shift_per_pixel()
    }

    pub fn is_empty(&self) -> bool {
        self.info().is_empty()
    }

    pub fn is_null(&self) -> bool {
        self.native().fPixelRef.fPtr.is_null()
    }

    pub fn draws_nothing(&self) -> bool {
        self.is_empty() || self.is_null()
    }

    pub fn row_bytes(&self) -> usize {
        self.pixmap().row_bytes()
    }

    pub fn set_alpha_type(&mut self, alpha_type: AlphaType) -> bool {
        unsafe { self.native_mut().setAlphaType(alpha_type) }
    }

    pub unsafe fn pixels(&mut self) -> *mut ffi::c_void {
        self.pixmap().writable_addr()
    }

    pub fn compute_byte_size(&self) -> usize {
        self.pixmap().compute_byte_size()
    }

    pub fn is_immutable(&self) -> bool {
        unsafe { self.native().isImmutable() }
    }

    pub fn set_immutable(&mut self) {
        unsafe { self.native_mut().setImmutable() }
    }

    pub fn is_opaque(&self) -> bool {
        self.pixmap().is_opaque()
    }

    #[deprecated(since = "0.35.0", note = "Removed without replacement")]
    pub fn is_volatile(&self) -> ! {
        panic!("Removed without replacement")
    }

    #[deprecated(since = "0.35.0", note = "Removed without replacement")]
    pub fn set_is_volatile(&mut self, _is_volatile: bool) {
        panic!("Removed without replacement");
    }

    pub fn reset(&mut self) {
        unsafe { self.native_mut().reset() }
    }

    pub fn compute_is_opaque(bm: &Self) -> bool {
        unsafe { sb::C_SkBitmap_ComputeIsOpaque(bm.native()) }
    }

    pub fn bounds(&self) -> IRect {
        self.info().bounds()
    }

    pub fn dimensions(&self) -> ISize {
        self.info().dimensions()
    }

    pub fn get_subset(&self) -> IRect {
        let origin = self.pixel_ref_origin();
        IRect::from_xywh(origin.x, origin.y, self.width(), self.height())
    }

    #[must_use]
    pub fn set_info(
        &mut self,
        image_info: &ImageInfo,
        row_bytes: impl Into<Option<usize>>,
    ) -> bool {
        unsafe {
            self.native_mut()
                .setInfo(image_info.native(), row_bytes.into().unwrap_or(0))
        }
    }

    #[must_use]
    pub fn try_alloc_pixels_flags(&mut self, image_info: &ImageInfo) -> bool {
        unsafe {
            self.native_mut().tryAllocPixelsFlags(
                image_info.native(),
                sb::SkBitmap_AllocFlags_kZeroPixels_AllocFlag as _,
            )
        }
    }

    pub fn alloc_pixels_flags(&mut self, image_info: &ImageInfo) {
        self.try_alloc_pixels_flags(image_info)
            .into_option()
            .expect("Bitmap::alloc_pixels_flags failed");
    }

    #[must_use]
    pub fn try_alloc_pixels_info(
        &mut self,
        image_info: &ImageInfo,
        row_bytes: impl Into<Option<usize>>,
    ) -> bool {
        let row_bytes = row_bytes
            .into()
            .unwrap_or_else(|| image_info.min_row_bytes());
        unsafe {
            self.native_mut()
                .tryAllocPixels(image_info.native(), row_bytes)
        }
    }

    pub fn alloc_pixels_info(
        &mut self,
        image_info: &ImageInfo,
        row_bytes: impl Into<Option<usize>>,
    ) {
        self.try_alloc_pixels_info(image_info, row_bytes.into())
            .into_option()
            .expect("Bitmap::alloc_pixels_info failed");
    }

    #[must_use]
    pub fn try_alloc_n32_pixels(
        &mut self,
        (width, height): (i32, i32),
        is_opaque: impl Into<Option<bool>>,
    ) -> bool {
        unsafe {
            sb::C_SkBitmap_tryAllocN32Pixels(
                self.native_mut(),
                width,
                height,
                is_opaque.into().unwrap_or(false),
            )
        }
    }

    pub fn alloc_n32_pixels(
        &mut self,
        (width, height): (i32, i32),
        is_opaque: impl Into<Option<bool>>,
    ) {
        self.try_alloc_n32_pixels((width, height), is_opaque.into().unwrap_or(false))
            .into_option()
            .expect("Bitmap::alloc_n32_pixels_failed")
    }

    pub unsafe fn install_pixels(
        &mut self,
        image_info: &ImageInfo,
        pixels: *mut ffi::c_void,
        row_bytes: usize,
    ) -> bool {
        self.native_mut().installPixels(
            image_info.native(),
            pixels,
            row_bytes,
            None,
            ptr::null_mut(),
        )
    }

    // TODO: setPixels()?

    #[must_use]
    pub fn try_alloc_pixels(&mut self) -> bool {
        unsafe { sb::C_SkBitmap_tryAllocPixels(self.native_mut()) }
    }

    pub fn alloc_pixels(&mut self) {
        self.try_alloc_pixels()
            .into_option()
            .expect("Bitmap::alloc_pixels failed")
    }

    // TODO: allocPixels(Allocator*)

    // TODO: find a way to return pixel ref without increasing the ref count here?
    pub fn pixel_ref(&self) -> Option<PixelRef> {
        PixelRef::from_unshared_ptr(self.native().fPixelRef.fPtr)
    }

    pub fn pixel_ref_origin(&self) -> IPoint {
        IPoint::from_native_c(unsafe { sb::C_SkBitmap_pixelRefOrigin(self.native()) })
    }

    pub fn set_pixel_ref(
        &mut self,
        pixel_ref: impl Into<Option<PixelRef>>,
        offset: impl Into<IPoint>,
    ) {
        let offset = offset.into();
        unsafe {
            sb::C_SkBitmap_setPixelRef(
                self.native_mut(),
                pixel_ref.into().into_ptr_or_null(),
                offset.x,
                offset.y,
            )
        }
    }

    pub fn is_ready_to_draw(&self) -> bool {
        unsafe { sb::C_SkBitmap_readyToDraw(self.native()) }
    }

    pub fn generation_id(&self) -> u32 {
        unsafe { self.native().getGenerationID() }
    }

    pub fn notify_pixels_changed(&self) {
        unsafe { self.native().notifyPixelsChanged() }
    }

    pub fn erase_color(&self, c: impl Into<Color>) {
        unsafe { self.native().eraseColor(c.into().into_native()) }
    }

    pub fn erase_argb(&self, a: u8, r: u8, g: u8, b: u8) {
        unsafe { sb::C_SkBitmap_eraseARGB(self.native(), a.into(), r.into(), g.into(), b.into()) }
    }

    pub fn erase(&self, c: impl Into<Color>, area: impl AsRef<IRect>) {
        unsafe {
            self.native()
                .erase(c.into().into_native(), area.as_ref().native())
        }
    }

    pub fn get_color(&self, p: impl Into<IPoint>) -> Color {
        self.pixmap().get_color(p)
    }

    pub fn get_alpha_f(&self, p: impl Into<IPoint>) -> f32 {
        self.pixmap().get_alpha_f(p)
    }

    pub unsafe fn get_addr(&self, p: impl Into<IPoint>) -> *const ffi::c_void {
        let p = p.into();
        self.native().getAddr(p.x, p.y)
    }

    // TODO: get_addr_32()?
    // TODO: get_addr_16()?

    pub fn extract_subset(&self, dst: &mut Self, subset: impl AsRef<IRect>) -> bool {
        unsafe {
            self.native()
                .extractSubset(dst.native_mut(), subset.as_ref().native())
        }
    }

    pub unsafe fn read_pixels(
        &self,
        dst_info: &ImageInfo,
        dst_pixels: *mut ffi::c_void,
        dst_row_bytes: usize,
        src_x: i32,
        src_y: i32,
    ) -> bool {
        self.native()
            .readPixels(dst_info.native(), dst_pixels, dst_row_bytes, src_x, src_y)
    }

    // TOOD: read_pixels(Pixmap)
    // TOOD: write_pixels(Pixmap)

    pub fn extract_alpha(&self, dst: &mut Self, paint: Option<&Paint>) -> Option<IPoint> {
        let mut offset = IPoint::default();
        unsafe {
            sb::C_SkBitmap_extractAlpha(
                self.native(),
                dst.native_mut(),
                paint.native_ptr_or_null(),
                offset.native_mut(),
            )
        }
        .if_true_some(offset)
    }

    pub fn peek_pixels(&self) -> Option<Borrows<Pixmap>> {
        let mut pixmap = Pixmap::default();
        unsafe { self.native().peekPixels(pixmap.native_mut()) }
            .if_true_then_some(|| pixmap.borrows(self))
    }

    pub fn to_shader<'a>(
        &self,
        tile_modes: impl Into<Option<(TileMode, TileMode)>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Shader {
        let tile_modes = tile_modes.into();
        let local_matrix = local_matrix.into();
        Shader::from_ptr(unsafe {
            let tmx = tile_modes.map(|tm| tm.0).unwrap_or_default();
            let tmy = tile_modes.map(|tm| tm.1).unwrap_or_default();
            sb::C_SkBitmap_makeShader(self.native(), tmx, tmy, local_matrix.native_ptr_or_null())
        })
        .unwrap()
    }
}

#[test]
fn create_clone_and_drop() {
    let bm = Bitmap::new();
    #[allow(clippy::redundant_clone)]
    let _bm2 = bm.clone();
}

#[test]
fn get_info() {
    let bm = Bitmap::new();
    let _info = bm.info();
}

#[test]
fn empty_bitmap_shader() {
    let bm = Bitmap::new();
    let _shader = bm.to_shader(None, None);
}

#[test]
fn shader_with_tilemode() {
    let bm = Bitmap::new();
    let _shader = bm.to_shader((TileMode::Decal, TileMode::Mirror), None);
}

#[test]
fn test_get_subset() {
    let bm = Bitmap::new();
    let _ = bm.get_subset();
}

#[test]
fn test_pixel_ref_origin() {
    let bm = Bitmap::new();
    let _ = bm.pixel_ref_origin();
}
