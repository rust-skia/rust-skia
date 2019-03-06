use crate::prelude::*;
use rust_skia::{
    SkPixelGeometry,
    SkSurfaceProps,
    SkSurfaceProps_Flags
};

pub type PixelGeometry = EnumHandle<SkPixelGeometry>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPixelGeometry> {
    pub const Unknown: Self = Self(SkPixelGeometry::kUnknown_SkPixelGeometry);
    pub const RGBH: Self = Self(SkPixelGeometry::kRGB_H_SkPixelGeometry);
    pub const BGRH: Self = Self(SkPixelGeometry::kBGR_H_SkPixelGeometry);
    pub const RGBV: Self = Self(SkPixelGeometry::kRGB_V_SkPixelGeometry);
    pub const BGRV: Self = Self(SkPixelGeometry::kBGR_V_SkPixelGeometry);

    pub fn is_rgb(&self) -> bool {
        *self == Self::RGBH || *self == Self::RGBV
    }

    pub fn is_bgr(&self) -> bool {
        *self == Self::BGRH || *self == Self::BGRV
    }

    pub fn is_h(&self) -> bool {
        *self == Self::RGBH || *self == Self::BGRH
    }

    pub fn is_v(&self) -> bool {
        *self == Self::RGBV || *self == Self::BGRV
    }
}

impl Default for EnumHandle<SkPixelGeometry> {
    fn default() -> Self {
        PixelGeometry::Unknown
    }
}

bitflags! {
    pub struct SurfacePropsFlags: u32 {
        const UseDeviceIndependentFonts =
            (SkSurfaceProps_Flags::kUseDeviceIndependentFonts_Flag) as u32;
    }
}

impl Default for SurfacePropsFlags {
    fn default() -> Self {
        SurfacePropsFlags::empty()
    }
}

pub type SurfaceProps = ValueHandle<SkSurfaceProps>;

impl NativeClone for SkSurfaceProps {
    fn clone(&self) -> Self {
        unsafe { SkSurfaceProps::new3(self) }
    }
}

impl NativePartialEq for SkSurfaceProps {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rust_skia::C_SkSurfaceProps_Equals(self, other) }
    }
}

impl Default for ValueHandle<SkSurfaceProps> {
    fn default() -> Self {
        SurfaceProps::new(Default::default(), Default::default())
    }
}

impl ValueHandle<SkSurfaceProps> {
    pub fn new(flags: SurfacePropsFlags, pixel_geometry: PixelGeometry) -> SurfaceProps {
        SurfaceProps::from_native(unsafe {
            SkSurfaceProps::new(flags.bits(), pixel_geometry.into_native())
        })
    }

    pub fn flags(&self) -> SurfacePropsFlags {
        SurfacePropsFlags::from_bits_truncate(unsafe {
            self.native().flags()
        })
    }

    pub fn pixel_geometry(&self) -> PixelGeometry {
        PixelGeometry::from_native(unsafe {
            self.native().pixelGeometry()
        })
    }

    pub fn is_use_device_independent_fonts(&self) -> bool {
        unsafe { self.native().isUseDeviceIndependentFonts() }
    }
}

#[test]
fn create() {
    let props = SurfaceProps::new(SurfacePropsFlags::UseDeviceIndependentFonts, PixelGeometry::RGBH);
    assert_eq!(SurfacePropsFlags::UseDeviceIndependentFonts, props.flags());
    assert_eq!(PixelGeometry::RGBH, props.pixel_geometry());
    assert_eq!(true, props.is_use_device_independent_fonts());
}

