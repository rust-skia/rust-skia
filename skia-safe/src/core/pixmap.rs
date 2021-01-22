use crate::{
    prelude::*, AlphaType, Color, Color4f, ColorSpace, ColorType, IPoint, IRect, ISize, ImageInfo,
    SamplingOptions,
};
use skia_bindings as sb;
use skia_bindings::SkPixmap;
use std::convert::TryInto;
use std::ffi::c_void;
use std::os::raw;
use std::{ptr, slice};

pub type Pixmap = Handle<SkPixmap>;
unsafe impl Send for Pixmap {}
unsafe impl Sync for Pixmap {}

impl NativeDrop for SkPixmap {
    fn drop(&mut self) {
        unsafe { sb::C_SkPixmap_destruct(self) }
    }
}

impl Default for Handle<SkPixmap> {
    fn default() -> Self {
        Pixmap::from_native_c(SkPixmap {
            fPixels: ptr::null(),
            fRowBytes: 0,
            fInfo: construct(|ii| unsafe { sb::C_SkImageInfo_Construct(ii) }),
        })
    }
}

impl Handle<SkPixmap> {
    pub fn new<'pixels>(
        info: &ImageInfo,
        pixels: &'pixels [u8],
        row_bytes: usize,
    ) -> Borrows<'pixels, Self> {
        let width: usize = info.width().try_into().unwrap();
        let height: usize = info.height().try_into().unwrap();

        assert!(row_bytes >= width * info.bytes_per_pixel());
        assert!(pixels.len() >= height * row_bytes);

        let pm = Pixmap::from_native_c(SkPixmap {
            fPixels: pixels.as_ptr() as _,
            fRowBytes: row_bytes,
            fInfo: info.native().clone(),
        });
        pm.borrows(pixels)
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    // TODO: Add reset function that borrows pixels?

    pub fn set_color_space(&mut self, color_space: impl Into<Option<ColorSpace>>) -> &mut Self {
        unsafe {
            sb::C_SkPixmap_setColorSpace(self.native_mut(), color_space.into().into_ptr_or_null())
        }
        self
    }

    pub fn extract_subset(&self, area: impl AsRef<IRect>) -> Option<Pixmap> {
        let mut pixmap = Pixmap::default();
        unsafe {
            self.native()
                .extractSubset(pixmap.native_mut(), area.as_ref().native())
        }
        .if_true_some(pixmap)
    }

    pub fn info(&self) -> &ImageInfo {
        ImageInfo::from_native_ref(&self.native().fInfo)
    }

    pub fn row_bytes(&self) -> usize {
        self.native().fRowBytes
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn addr(&self) -> *const c_void {
        self.native().fPixels
    }

    pub fn width(&self) -> i32 {
        self.info().width()
    }

    pub fn height(&self) -> i32 {
        self.info().height()
    }

    pub fn dimensions(&self) -> ISize {
        self.info().dimensions()
    }

    pub fn color_type(&self) -> ColorType {
        self.info().color_type()
    }

    pub fn alpha_type(&self) -> AlphaType {
        self.info().alpha_type()
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.info().color_space()
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha_type().is_opaque()
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_wh(self.width(), self.height())
    }

    pub fn row_bytes_as_pixels(&self) -> usize {
        self.row_bytes() >> self.shift_per_pixel()
    }

    pub fn shift_per_pixel(&self) -> usize {
        self.info().shift_per_pixel()
    }

    pub fn compute_byte_size(&self) -> usize {
        self.info().compute_byte_size(self.row_bytes())
    }

    pub fn compute_is_opaque(&self) -> bool {
        unsafe { self.native().computeIsOpaque() }
    }

    pub fn get_color(&self, p: impl Into<IPoint>) -> Color {
        let p = p.into();
        self.assert_pixel_exists(p);
        Color::from_native_c(unsafe { self.native().getColor(p.x, p.y) })
    }

    pub fn get_alpha_f(&self, p: impl Into<IPoint>) -> f32 {
        let p = p.into();
        self.assert_pixel_exists(p);
        unsafe { self.native().getAlphaf(p.x, p.y) }
    }

    // Helper to test if the pixel does exist physically in memory.
    fn assert_pixel_exists(&self, p: impl Into<IPoint>) {
        let p = p.into();
        assert!(!unsafe { self.addr() }.is_null());
        assert!(p.x >= 0 && p.x < self.width());
        assert!(p.y >= 0 && p.y < self.height());
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn addr_at(&self, p: impl Into<IPoint>) -> *const c_void {
        let p = p.into();
        (self.addr() as *const raw::c_char).add(self.info().compute_offset(p, self.row_bytes()))
            as _
    }

    // TODO: addr8(), addr16(), addr32(), addr64(), addrF16(),
    //       addr8_at(), addr16_at(), addr32_at(), addr64_at(), addrF16_at()

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn writable_addr(&self) -> *mut c_void {
        self.addr() as _
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn writable_addr_at(&self, p: impl Into<IPoint>) -> *mut c_void {
        self.addr_at(p) as _
    }

    // TODO: writable_addr8
    // TODO: writable_addr16
    // TODO: writable_addr32
    // TODO: writable_addr64
    // TODO: writable_addrF16

    pub fn read_pixels<P>(
        &self,
        dst_info: &ImageInfo,
        pixels: &mut [P],
        dst_row_bytes: usize,
        src: impl Into<IPoint>,
    ) -> bool {
        if pixels.elements_size_of()
            != (usize::try_from(dst_info.height()).unwrap() * dst_row_bytes)
        {
            return false;
        }

        let src = src.into();

        unsafe {
            self.native().readPixels(
                dst_info.native(),
                pixels.as_mut_ptr() as _,
                dst_row_bytes,
                src.x,
                src.y,
            )
        }
    }

    pub fn read_pixels_to_pixmap(&self, dst: &Pixmap, src: impl Into<IPoint>) -> bool {
        let row_bytes = dst.row_bytes();
        let len = usize::try_from(dst.height()).unwrap() * row_bytes;
        unsafe {
            let addr = dst.writable_addr() as *mut raw::c_char;
            self.read_pixels(
                dst.info(),
                slice::from_raw_parts_mut(addr, len),
                row_bytes,
                src,
            )
        }
    }

    pub fn scale_pixels(&self, dst: &Pixmap, sampling: impl Into<SamplingOptions>) -> bool {
        let sampling = sampling.into();
        unsafe { self.native().scalePixels(dst.native(), sampling.native()) }
    }

    pub fn erase(&self, color: impl Into<Color>, subset: Option<&IRect>) -> bool {
        let color = color.into().into_native();
        unsafe {
            match subset {
                Some(subset) => self.native().erase(color, subset.native()),
                None => self.native().erase(color, self.bounds().native()),
            }
        }
    }

    pub fn erase_4f(&self, color: impl AsRef<Color4f>, subset: Option<&IRect>) -> bool {
        self.erase_with_colorspace(color, None, subset)
    }

    pub fn erase_with_colorspace(
        &self,
        color: impl AsRef<Color4f>,
        cs: Option<&ColorSpace>,
        subset: Option<&IRect>,
    ) -> bool {
        let color = color.as_ref();
        unsafe {
            self.native().erase1(
                color.native(),
                cs.native_ptr_or_null_mut_force(),
                subset.native_ptr_or_null(),
            )
        }
    }
}
