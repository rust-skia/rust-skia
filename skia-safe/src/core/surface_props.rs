use crate::prelude::*;
use skia_bindings::{
    SkPixelGeometry,
    SkSurfaceProps,
    SkSurfaceProps_Flags_kUseDeviceIndependentFonts_Flag
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PixelGeometry {
    Unknown = SkPixelGeometry::kUnknown_SkPixelGeometry as _,
    RGBH = SkPixelGeometry::kRGB_H_SkPixelGeometry as _,
    BGRH = SkPixelGeometry::kBGR_H_SkPixelGeometry as _,
    RGBV = SkPixelGeometry::kRGB_V_SkPixelGeometry as _,
    BGRV = SkPixelGeometry::kBGR_V_SkPixelGeometry as _
}

impl NativeTransmutable<SkPixelGeometry> for PixelGeometry {}
#[test] fn test_pixel_geometry_layout() { PixelGeometry::test_layout() }

impl PixelGeometry {
    pub fn is_rgb(self) -> bool {
        self == PixelGeometry::RGBH || self == PixelGeometry::RGBV
    }

    pub fn is_bgr(self) -> bool {
        self == PixelGeometry::BGRH || self == PixelGeometry::BGRV
    }

    pub fn is_h(self) -> bool {
        self == PixelGeometry::RGBH || self == PixelGeometry::BGRH
    }

    pub fn is_v(self) -> bool {
        self == PixelGeometry::RGBV || self == PixelGeometry::BGRV
    }
}

impl Default for PixelGeometry {
    fn default() -> Self {
        PixelGeometry::Unknown
    }
}

bitflags! {
    pub struct SurfacePropsFlags: u32 {
        const USE_DEVICE_INDEPENDENT_FONTS =
            SkSurfaceProps_Flags_kUseDeviceIndependentFonts_Flag as u32;
    }
}

impl Default for SurfacePropsFlags {
    fn default() -> Self {
        SurfacePropsFlags::empty()
    }
}

#[derive(Copy)]
#[repr(transparent)]
pub struct SurfaceProps(SkSurfaceProps);

impl NativeTransmutable<SkSurfaceProps> for SurfaceProps {}

impl Clone for SurfaceProps {
    fn clone(&self) -> Self {
        Self::from_native(unsafe {
            SkSurfaceProps::new3(self.native())
        })
    }
}

impl PartialEq for SurfaceProps {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            skia_bindings::C_SkSurfaceProps_Equals(self.native(), other.native())
        }
    }
}

impl Default for SurfaceProps {
    fn default() -> Self {
        SurfaceProps::new(Default::default(), Default::default())
    }
}

impl SurfaceProps  {
    pub fn new(flags: SurfacePropsFlags, pixel_geometry: PixelGeometry) -> SurfaceProps {
        Self::from_native(unsafe {
            SkSurfaceProps::new(flags.bits(), pixel_geometry.into_native())
        })
    }

    pub fn flags(self) -> SurfacePropsFlags {
        SurfacePropsFlags::from_bits_truncate(unsafe {
            self.native().flags()
        })
    }

    pub fn pixel_geometry(self) -> PixelGeometry {
        PixelGeometry::from_native(unsafe {
            self.native().pixelGeometry()
        })
    }

    pub fn is_use_device_independent_fonts(self) -> bool {
        unsafe { self.native().isUseDeviceIndependentFonts() }
    }
}

#[test]
fn create() {
    let props = SurfaceProps::new(SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS, PixelGeometry::RGBH);
    assert_eq!(SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS, props.flags());
    assert_eq!(PixelGeometry::RGBH, props.pixel_geometry());
    assert_eq!(true, props.is_use_device_independent_fonts());
}

