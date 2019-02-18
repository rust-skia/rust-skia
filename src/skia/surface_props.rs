use rust_skia::SkPixelGeometry;
use rust_skia::SkSurfaceProps;
use rust_skia::SkSurfaceProps_Flags;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PixelGeometry(pub(crate) SkPixelGeometry);

#[allow(non_upper_case_globals)]
impl PixelGeometry {
    pub const Unknown: PixelGeometry = PixelGeometry(SkPixelGeometry::kUnknown_SkPixelGeometry);
    pub const RGBH: PixelGeometry = PixelGeometry(SkPixelGeometry::kRGB_H_SkPixelGeometry);
    pub const BGRH: PixelGeometry = PixelGeometry(SkPixelGeometry::kBGR_H_SkPixelGeometry);
    pub const RGBV: PixelGeometry = PixelGeometry(SkPixelGeometry::kRGB_V_SkPixelGeometry);
    pub const BGRV: PixelGeometry = PixelGeometry(SkPixelGeometry::kBGR_V_SkPixelGeometry);

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

bitflags! {
    pub struct SurfacePropsFlags: u32 {
        const UseDeviceIndependentFonts =
            (SkSurfaceProps_Flags::kUseDeviceIndependentFonts_Flag) as u32;
    }
}

impl From<SkSurfaceProps_Flags> for SurfacePropsFlags {
    fn from(flags: SkSurfaceProps_Flags) -> Self {
        SurfacePropsFlags::from_bits(flags as u32).unwrap()
    }
}

#[derive(Copy)]
pub struct SurfaceProps(pub(crate) SkSurfaceProps);

impl Clone for SurfaceProps {
    fn clone(&self) -> Self {
        SurfaceProps(unsafe { SkSurfaceProps::new3(&self.0) })
    }
}

impl PartialEq for SurfaceProps {
    fn eq(&self, other: &SurfaceProps) -> bool {
        unsafe { rust_skia::C_SkSurfaceProps_Equals(&self.0, &other.0) }
    }
}

impl SurfaceProps {
    pub fn new(flags: SurfacePropsFlags, pixel_geometry: PixelGeometry) -> SurfaceProps {
        SurfaceProps(unsafe { SkSurfaceProps::new(flags.bits(), pixel_geometry.0) })
    }

    pub fn flags(&self) -> SurfacePropsFlags {
        SurfacePropsFlags::from_bits(unsafe { self.0.flags() }).unwrap()
    }

    pub fn pixel_geometry(&self) -> PixelGeometry {
        PixelGeometry(unsafe { self.0.pixelGeometry() })
    }

    pub fn is_use_device_independnet_fonts(&self) -> bool {
        unsafe { self.0.isUseDeviceIndependentFonts() }
    }
}

#[test]
fn create() {
    let props = SurfaceProps::new(SurfacePropsFlags::UseDeviceIndependentFonts, PixelGeometry::RGBH);
    assert_eq!(SurfacePropsFlags::UseDeviceIndependentFonts, props.flags());
    assert_eq!(PixelGeometry::RGBH, props.pixel_geometry());
    assert_eq!(true, props.is_use_device_independnet_fonts());
}

