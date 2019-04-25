use crate::prelude::*;
use std::{ffi, mem, ptr};
use crate::core::{
    Paint,
    Color,
    ColorType,
    AlphaType,
    ColorSpace,
    IRect,
    ImageInfo,
    ISize,
    IPoint,
};
use skia_bindings::{SkPaint, SkIPoint, C_SkImageInfo_Copy, C_SkBitmap_destruct, SkBitmap, C_SkBitmap_Construct, C_SkBitmap_readyToDraw, C_SkBitmap_tryAllocN32Pixels, C_SkBitmap_tryAllocPixels, C_SkBitmap_eraseARGB, C_SkBitmap_extractAlpha, SkBitmap_AllocFlags_kZeroPixels_AllocFlag, C_SkBitmap_makeShader};
use crate::{Matrix, Shader, TileMode};

#[deprecated(since="0.6.0", note="AllocFlags is obsolete.  We always zero pixel memory when allocated.")]
bitflags! {
    pub struct BitmapAllocFlags: u32 {
        const ZERO_PIXELS = SkBitmap_AllocFlags_kZeroPixels_AllocFlag as u32;
    }
}

pub type Bitmap = Handle<SkBitmap>;

impl NativeDrop for SkBitmap {
    fn drop(&mut self) {
        unsafe { C_SkBitmap_destruct(self) }
    }
}

impl NativeClone for SkBitmap {
    fn clone(&self) -> Self {
        unsafe { SkBitmap::new1(self) }
    }
}

impl Handle<SkBitmap> {
    pub fn new() -> Self {
        Self::construct_c(C_SkBitmap_Construct)
    }

    pub fn swap(&mut self, other: &mut Self) {
        unsafe { self.native_mut().swap(other.native_mut()) }
    }

    pub fn info(&self) -> ImageInfo {
        // we contain ImageInfo in a struct, so we have to copy it.
        let ptr = unsafe { self.native().info() };
        let mut image_info = ImageInfo::default();
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
        ColorType::from_native(unsafe { self.native().colorType() })
    }

    pub fn alpha_type(&self) -> AlphaType {
        AlphaType::from_native(unsafe { self.native().alphaType() })
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_ptr(unsafe { skia_bindings::C_SkBitmap_colorSpace(self.native()) })
    }

    pub fn bytes_per_pixel(&self) -> usize {
        unsafe { self.native().bytesPerPixel().try_into().unwrap() }
    }

    pub fn row_bytes_as_pixels(&self) -> usize {
        unsafe { self.native().rowBytesAsPixels().try_into().unwrap() }
    }

    pub fn shift_per_pixel(&self) -> usize {
        unsafe { self.native().shiftPerPixel().try_into().unwrap() }
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
        unsafe { self.native_mut().setAlphaType(alpha_type.into_native()) }
    }

    pub unsafe fn pixels(&mut self) -> *mut ffi::c_void {
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
        unsafe { skia_bindings::C_SkBitmap_ComputeIsOpaque(bm.native()) }
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

    #[must_use]
    pub fn set_info(&mut self, image_info: &ImageInfo, row_bytes: Option<usize>) -> bool {
        unsafe { self.native_mut().setInfo(image_info.native(), row_bytes.unwrap_or(0)) }
    }

    #[must_use]
    pub fn try_alloc_pixels_flags(&mut self, image_info: &ImageInfo, flags: BitmapAllocFlags) -> bool {
        unsafe { self.native_mut().tryAllocPixelsFlags(image_info.native(), flags.bits()) }
    }

    pub fn alloc_pixels_flags(&mut self, image_info: &ImageInfo, flags: BitmapAllocFlags) {
        self.try_alloc_pixels_flags(image_info, flags)
            .to_option()
            .expect("Bitmap::alloc_pixels_flags failed");
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

    pub fn alloc_pixels_info(&mut self, image_info: &ImageInfo, row_bytes: Option<usize>) {
        self.try_alloc_pixels_info(image_info, row_bytes)
            .to_option()
            .expect("Bitmap::alloc_pixels_info failed");
    }

    #[must_use]
    pub fn try_alloc_n32_pixels(&mut self, (width, height): (i32, i32), is_opaque: bool) -> bool {
        // accessing the instance method causes a linker error.
        unsafe { C_SkBitmap_tryAllocN32Pixels(self.native_mut(), width, height, is_opaque) }
    }

    pub fn alloc_n32_pixels(&mut self, (width, height): (i32, i32), is_opaque: bool) {
        self.try_alloc_n32_pixels((width, height), is_opaque)
            .to_option()
            .expect("Bitmap::alloc_n32_pixels_failed")
    }

    pub unsafe fn install_pixels(&mut self, image_info: &ImageInfo, pixels: *mut ffi::c_void, row_bytes: usize) -> bool {
        self.native_mut().installPixels1(image_info.native(), pixels, row_bytes)
    }

    #[must_use]
    pub fn try_alloc_pixels(&mut self) -> bool {
        // linker errr.
        unsafe { C_SkBitmap_tryAllocPixels(self.native_mut()) }
    }

    pub fn alloc_pixels(&mut self) {
        self.try_alloc_pixels()
            .to_option()
            .expect("Bitmap::alloc_pixels failed")
    }

    pub fn pixel_ref_origin(&self) -> IPoint {
        IPoint::from_native(unsafe { self.native().pixelRefOrigin() })
    }

    pub fn ready_to_draw(&self) -> bool {
        unsafe { C_SkBitmap_readyToDraw(self.native()) }
    }

    pub fn generation_id(&self) -> u32 {
        unsafe { self.native().getGenerationID() }
    }

    pub fn notify_pixels_changed(&self) {
        unsafe { self.native().notifyPixelsChanged() }
    }

    pub fn erase_color<C: Into<Color>>(&self, c: C) {
        unsafe { self.native().eraseColor(c.into().into_native()) }
    }

    pub fn erase_argb(&self, a: u8, r: u8, g: u8, b: u8) {
        unsafe { C_SkBitmap_eraseARGB(self.native(), a.into(), r.into(), g.into(), b.into()) }
    }

    pub fn erase<C: Into<Color>, IR: AsRef<IRect>>(&self, c: C, area: IR) {
        unsafe { self.native().erase(c.into().into_native(), area.as_ref().native()) }
    }

    pub fn get_color(&self, p: IPoint) -> Color {
        Color::from_native(unsafe { self.native().getColor(p.x, p.y) })
    }

    pub fn get_alpha_f(&self, p: IPoint) -> f32 {
        unsafe { self.native().getAlphaf(p.x, p.y) }
    }

    #[inline]
    pub unsafe fn get_addr(&self, p: IPoint) -> *const ffi::c_void {
        self.native().getAddr(p.x, p.y)
    }

    pub fn extract_subset<IR: AsRef<IRect>>(&self, dst: &mut Self, subset: IR) -> bool {
        unsafe { self.native().extractSubset(dst.native_mut(), subset.as_ref().native() ) }
    }

    pub unsafe fn read_pixels(&self, dst_info: &ImageInfo, dst_pixels: *mut ffi::c_void, dst_row_bytes: usize, src_x: i32, src_y: i32) -> bool {
        self.native().readPixels(dst_info.native(), dst_pixels, dst_row_bytes, src_x, src_y)
    }

    pub fn extract_alpha(&self, dst: &mut Self, paint: Option<&Paint>) -> Option<IPoint> {
        let paint_ptr =
            paint
                .map(|p| p.native() as *const SkPaint)
                .unwrap_or(ptr::null());

        let mut offset : SkIPoint = unsafe { mem::uninitialized() };
        unsafe { C_SkBitmap_extractAlpha(self.native(), dst.native_mut(), paint_ptr, &mut offset) }
            .if_true_some(IPoint::from_native(offset))
    }

    pub fn new_shader(&self, tile_modes: Option<(TileMode, TileMode)>, local_matrix: Option<&Matrix>) -> Shader {
        Shader::from_ptr(unsafe {
            let tmx = tile_modes.map(|tm| tm.0).unwrap_or(TileMode::default());
            let tmy = tile_modes.map(|tm| tm.1).unwrap_or(TileMode::default());
            C_SkBitmap_makeShader(self.native(), tmx.into_native(), tmy.into_native(), local_matrix.native_ptr_or_null())
        }).unwrap()
    }
}

#[test]
fn create_clone_and_drop() {
    let bm = Bitmap::new();
    let _bm2 = bm.clone();
}

#[test]
fn get_info() {
    let bm = Bitmap::new();
    let _info = bm.info();
}
