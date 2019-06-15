use crate::prelude::*;
use crate::{
    AlphaType, Color, Color4f, ColorSpace, ColorType, FilterQuality, IPoint, IRect, ImageInfo,
};
use skia_bindings::{C_SkPixmap_destruct, C_SkPixmap_setColorSpace, SkPixmap};
use std::convert::TryInto;
use std::ffi::c_void;

pub type Pixmap = Handle<SkPixmap>;

impl NativeDrop for SkPixmap {
    fn drop(&mut self) {
        unsafe { C_SkPixmap_destruct(self) }
    }
}

impl Default for Handle<SkPixmap> {
    fn default() -> Self {
        Pixmap::from_native(unsafe { SkPixmap::new() })
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
        let pm = Pixmap::from_native(unsafe {
            SkPixmap::new1(info.native(), pixels.as_ptr() as _, row_bytes)
        });
        pm.borrows(pixels)
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    // TODO: Add reset function that borrows pixels?

    pub fn set_color_space<'a>(
        &mut self,
        color_space: impl Into<Option<&'a ColorSpace>>,
    ) -> &mut Self {
        unsafe { C_SkPixmap_setColorSpace(self.native_mut(), color_space.into().shared_ptr()) }
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
        ImageInfo::from_native_ref(unsafe { &*self.native().info() })
    }

    pub fn row_bytes(&self) -> usize {
        unsafe { self.native().rowBytes() }
    }

    pub unsafe fn addr(&self) -> *const c_void {
        self.native().addr()
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
        ColorSpace::from_unshared_ptr(unsafe { self.native().colorSpace() })
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { self.native().isOpaque() }
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_native(unsafe { self.native().bounds() })
    }

    pub fn row_bytes_as_pixels(&self) -> usize {
        unsafe { self.native().rowBytesAsPixels().try_into().unwrap() }
    }

    pub fn shift_per_pixel(&self) -> usize {
        unsafe { self.native().shiftPerPixel().try_into().unwrap() }
    }

    pub fn compute_byte_size(&self) -> usize {
        unsafe { self.native().computeByteSize().try_into().unwrap() }
    }

    pub fn compute_is_opaque(&self) -> bool {
        unsafe { self.native().computeIsOpaque() }
    }

    pub fn get_color(&self, p: impl Into<IPoint>) -> Color {
        let p = p.into();
        Color::from_native(unsafe { self.native().getColor(p.x, p.y) })
    }

    pub fn get_alpha_f(&self, p: impl Into<IPoint>) -> f32 {
        let p = p.into();
        unsafe { self.native().getAlphaf(p.x, p.y) }
    }

    pub unsafe fn addr_at(&self, p: impl Into<IPoint>) -> *const c_void {
        let p = p.into();
        self.native().addr1(p.x, p.y)
    }

    // TODO: addr8(), addr16(), addr32(), addr64(), addrF16(),
    //       addr8_at(), addr16_at(), addr32_at(), addr64_at(), addrF16_at()

    pub unsafe fn writable_addr(&self) -> *mut c_void {
        self.native().writable_addr()
    }

    pub unsafe fn writable_addr_at(&self, p: impl Into<IPoint>) -> *mut c_void {
        let p = p.into();
        self.native().writable_addr1(p.x, p.y)
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
            self.native().readPixels1(
                dst_info.native(),
                pixels.as_mut_ptr() as _,
                dst_row_bytes,
                src.x,
                src.y,
            )
        }
    }

    pub fn read_pixels_to_pixmap(&self, dst: &Pixmap, src: impl Into<IPoint>) -> bool {
        let src = src.into();
        unsafe { self.native().readPixels2(dst.native(), src.x, src.y) }
    }

    pub fn scale_pixels(&self, dst: &Pixmap, filter_quality: FilterQuality) -> bool {
        unsafe {
            self.native()
                .scalePixels(dst.native(), filter_quality.into_native())
        }
    }

    pub fn erase(&self, color: impl Into<Color>, subset: Option<&IRect>) -> bool {
        let color = color.into().into_native();
        unsafe {
            match subset {
                Some(subset) => self.native().erase(color, subset.native()),
                None => self.native().erase1(color),
            }
        }
    }

    pub fn erase_4f(&self, color: impl AsRef<Color4f>, subset: Option<&IRect>) -> bool {
        let color = color.as_ref();
        unsafe {
            self.native()
                .erase2(color.native(), subset.native_ptr_or_null())
        }
    }
}
