use std::{
    ffi::c_void,
    mem::uninitialized,
    ptr
};
use crate::{
    prelude::*,
    skia::{
        Color,
        ColorType,
        AlphaType,
        ColorSpace,
        IRect,
        ImageInfo,
        ISize,
        IPoint,
        u8cpu
    }
};
use crate::skia::Paint;
use rust_skia::{
    SkPaint,
    SkIPoint,
    C_SkImageInfo_Copy,
    C_SkBitmap_Destruct,
    SkBitmap,
    C_SkBitmap_Copy,
    C_SkBitmap_Construct,
    C_SkBitmap_readyToDraw,
    SkBitmap_ComputeIsOpaque,
    SkBitmap_AllocFlags,
    C_SkBitmap_tryAllocN32Pixels,
    C_SkBitmap_tryAllocPixels,
    C_SkBitmap_eraseARGB,
    C_SkBitmap_extractAlpha
};

bitflags! {
    pub struct BitmapAllocFlags: u32 {
        const ZeroPixels = SkBitmap_AllocFlags::kZeroPixels_AllocFlag as u32;
    }
}

impl From<SkBitmap_AllocFlags> for BitmapAllocFlags {
    fn from(flags: SkBitmap_AllocFlags) -> BitmapAllocFlags {
        BitmapAllocFlags::from_bits(flags as u32).unwrap()
    }
}

pub type Bitmap = Handle<SkBitmap>;

impl NativeDrop for SkBitmap {
    fn drop(&mut self) {
        unsafe { C_SkBitmap_Destruct(self) }
    }
}

impl NativeClone for SkBitmap {
    fn clone(&self) -> Self {
        return unsafe { SkBitmap::new1(self) };
    }
}

impl Bitmap {
    pub fn new() -> Bitmap {
        let mut bitmap : Bitmap = unsafe { uninitialized() };
        unsafe { C_SkBitmap_Construct(bitmap.native_mut()) }
        bitmap
    }

    pub fn swap(&mut self, other: &mut Self) {
        unsafe { self.native_mut().swap(other.native_mut()) }
    }

    pub fn info(&self) -> ImageInfo {
        // we contain ImageInfo in a struct, so we have to copy it.
        let ptr = unsafe { self.native().info() };
        let mut image_info = ImageInfo::new_empty();
        unsafe { C_SkImageInfo_Copy(ptr, image_info.native_mut()) }
        image_info
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn color_type(&self) -> ColorType {
        ColorType(unsafe { self.native().colorType() })
    }

    pub fn alpha_type(&self) -> AlphaType {
        AlphaType(unsafe { self.native().alphaType() })
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkBitmap_colorSpace(self.native()) })
    }

    pub fn bytes_per_pixel(&self) -> usize {
        unsafe { self.native().bytesPerPixel() as usize }
    }

    pub fn row_bytes_as_pixels(&self) -> usize {
        unsafe { self.native().rowBytesAsPixels() as usize }
    }

    pub fn shift_per_pixel(&self) -> usize {
        unsafe { self.native().shiftPerPixel() as usize }
    }

    pub fn empty(&self) -> bool {
        unsafe { self.native().empty() }
    }

    pub fn is_null(&self) -> bool {
        unsafe { self.native().isNull() }
    }

    pub fn draws_nothing(&self) -> bool {
        unsafe { self.native().drawsNothing() }
    }

    pub fn row_bytes(&self) -> usize {
        unsafe { self.native().rowBytes() }
    }

    pub fn set_alpha_type(&mut self, alpha_type: AlphaType) -> bool {
        unsafe { self.native_mut().setAlphaType(alpha_type.0) }
    }

    pub unsafe fn get_pixels(&mut self) -> *mut c_void {
        self.native_mut().getPixels()
    }

    pub fn compute_byte_size(&self) -> usize {
        unsafe { self.native().computeByteSize() }
    }

    pub fn is_immutable(&self) -> bool {
        unsafe { self.native().isImmutable() }
    }

    pub fn set_immutable(&mut self) {
        unsafe { self.native_mut().setImmutable() }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { self.native().isOpaque() }
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { self.native().isVolatile() }
    }

    pub fn set_is_volatile(&mut self, is_volatile: bool) {
        unsafe { self.native_mut().setIsVolatile(is_volatile) }
    }

    pub fn reset(&mut self) {
        unsafe { self.native_mut().reset() }
    }

    pub fn compute_is_opaque(bm: &Self) -> bool {
        // well, the binding's version causes a linker error.
        unsafe { rust_skia::C_SkBitmap_ComputeIsOpaque(bm.native()) }
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_native(unsafe { self.native().bounds() })
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native(unsafe { self.native().dimensions() })
    }

    pub fn get_subset(&self) -> IRect {
        IRect::from_native(unsafe { self.native().getSubset() })
    }

    pub fn set_info(&mut self, image_info: &ImageInfo, row_bytes: Option<usize>) -> bool {
        unsafe { self.native_mut().setInfo(image_info.native(), row_bytes.unwrap_or(0)) }
    }

    #[must_use]
    pub fn try_alloc_pixels_flags(&mut self, image_info: &ImageInfo, flags: BitmapAllocFlags) -> bool {
        unsafe { self.native_mut().tryAllocPixelsFlags(image_info.native(), flags.bits()) }
    }

    #[must_use]
    pub fn try_alloc_pixels_info(&mut self, image_info: &ImageInfo, row_bytes: Option<usize>) -> bool {
        match row_bytes {
            Some(row_bytes) =>
                unsafe { self.native_mut().tryAllocPixels(image_info.native(), row_bytes) },
            None =>
                unsafe { self.native_mut().tryAllocPixels1(image_info.native()) },
        }
    }

    #[must_use]
    pub fn try_alloc_n32_pixels(&mut self, width: i32, height: i32, is_opaque: bool) -> bool {
        // accessing the instance method causes a linker error.
        unsafe { C_SkBitmap_tryAllocN32Pixels(self.native_mut(), width, height, is_opaque) }
    }

    pub unsafe fn install_pixels(&mut self, image_info: &ImageInfo, pixels: *mut c_void, row_bytes: usize) -> bool {
        self.native_mut().installPixels1(image_info.native(), pixels, row_bytes)
    }

    #[must_use]
    pub fn try_alloc_pixels(&mut self) -> bool {
        // linker errr.
        unsafe { C_SkBitmap_tryAllocPixels(self.native_mut()) }
    }

    pub fn pixel_ref_origin(&self) -> IPoint {
        IPoint::from_native(unsafe { self.native().pixelRefOrigin() })
    }

    pub fn ready_to_draw(&self) -> bool {
        unsafe { C_SkBitmap_readyToDraw(self.native()) }
    }

    pub fn get_generation_id(&self) -> u32 {
        unsafe { self.native().getGenerationID() }
    }

    pub fn notify_pixels_changed(&self) {
        unsafe { self.native().notifyPixelsChanged() }
    }

    pub fn erase_color(&self, c: Color) {
        unsafe { self.native().eraseColor(c.0) }
    }

    pub fn erase_argb(&self, a: u8cpu, r: u8cpu, g: u8cpu, b: u8cpu) {
        unsafe { C_SkBitmap_eraseARGB(self.native(), a, r, g, b) }
    }

    pub fn erase(&self, c: Color, area: &IRect) {
        unsafe { self.native().erase(c.0, &area.to_native()) }
    }

    #[inline]
    pub fn get_color(&self, x: i32, y: i32) -> Color {
        Color(unsafe { self.native().getColor(x, y) })
    }

    #[inline]
    pub fn get_alpha_f(&self, x: i32, y: i32) -> f32 {
        unsafe { self.native().getAlphaf(x, y) }
    }

    #[inline]
    pub unsafe fn get_addr(&self, x: i32, y: i32) -> *const c_void {
        self.native().getAddr(x, y)
    }

    pub fn extract_subset(&self, dst: &mut Bitmap, subset: &IRect) -> bool {
        unsafe { self.native().extractSubset(dst.native_mut(), &subset.to_native() ) }
    }

    pub unsafe fn read_pixels(&self, dst_info: &ImageInfo, dst_pixels: *mut c_void, dst_row_bytes: usize, src_x: i32, src_y: i32) -> bool {
        self.native().readPixels(dst_info.native(), dst_pixels, dst_row_bytes, src_x, src_y)
    }

    pub fn extract_alpha(&self, dst: &mut Bitmap, paint: Option<&Paint>) -> Option<IPoint> {
        let paint_ptr =
            paint
                .map(|p| p.native() as *const SkPaint)
                .unwrap_or(ptr::null());

        let mut offset : SkIPoint = unsafe { uninitialized() };
        if unsafe { C_SkBitmap_extractAlpha(self.native(), dst.native_mut(), paint_ptr, &mut offset) } {
            Some(IPoint::from_native(offset))
        } else {
            None
        }
    }
}

#[test]
fn create_clone_and_drop() {
    let bm = Bitmap::new();
    let bm2 = bm.clone();
}

#[test]
fn get_info() {
    let bm = Bitmap::new();
    let info = bm.info();
}