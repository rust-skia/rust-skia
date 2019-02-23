use std::mem;
use super::{Matrix44, Data};
use crate::prelude::*;
use rust_skia::{
    SkColorSpaceTransferFn,
    SkColorSpace,
    SkColorSpacePrimaries,
    SkGammaNamed,
    SkColorSpace_Gamut,
    SkColorSpace_RenderTargetGamma,
};

pub struct GammaNamed(pub (crate) SkGammaNamed);

#[allow(non_upper_case_globals)]
impl GammaNamed {
    pub const Linear: GammaNamed = GammaNamed(SkGammaNamed::kLinear_SkGammaNamed);
    pub const SRGB: GammaNamed = GammaNamed(SkGammaNamed::kSRGB_SkGammaNamed);
    pub const Curve2Dot2: GammaNamed = GammaNamed(SkGammaNamed::k2Dot2Curve_SkGammaNamed);
    pub const NonStandard: GammaNamed = GammaNamed(SkGammaNamed::kNonStandard_SkGammaNamed);
}

pub struct ColorSpacePrimaries {
    rx: f32,
    ry: f32,
    gx: f32,
    gy: f32,
    bx: f32,
    by: f32,
    wx: f32,
    wy: f32
}

impl Into<SkColorSpacePrimaries> for ColorSpacePrimaries {
    fn into(self) -> SkColorSpacePrimaries {
        SkColorSpacePrimaries {
            fRX: self.rx,
            fRY: self.ry,
            fGX: self.gx,
            fGY: self.gy,
            fBX: self.bx,
            fBY: self.by,
            fWX: self.wx,
            fWY: self.wy
        }
    }
}

impl From<SkColorSpacePrimaries> for ColorSpacePrimaries {
    fn from(v: SkColorSpacePrimaries) -> ColorSpacePrimaries {
        ColorSpacePrimaries {
            rx: v.fRX,
            ry: v.fRY,
            gx: v.fGX,
            gy: v.fGY,
            bx: v.fBX,
            by: v.fBY,
            wx: v.fWX,
            wy: v.fWY
        }
    }
}

impl Into<Option<Matrix44>> for ColorSpacePrimaries {
    fn into(self) -> Option<Matrix44> {
        let mut matrix = Matrix44::new();
        let primaries : SkColorSpacePrimaries = self.into();
        if unsafe { primaries.toXYZD50(&mut matrix.0) } {
            Some(matrix)
        } else {
            None
        }
    }
}

pub struct ColorSpaceTransferFn {
    pub g: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32
}

impl Into<SkColorSpaceTransferFn> for ColorSpaceTransferFn {
    fn into(self) -> SkColorSpaceTransferFn {
        SkColorSpaceTransferFn {
            fG: self.g,
            fA: self.a,
            fB: self.b,
            fC: self.c,
            fD: self.d,
            fE: self.e,
            fF: self.f
        }
    }
}

impl From<SkColorSpaceTransferFn> for ColorSpaceTransferFn {
    fn from(csfn: SkColorSpaceTransferFn) -> Self {
        ColorSpaceTransferFn {
            g: csfn.fG,
            a: csfn.fA,
            b: csfn.fB,
            c: csfn.fC,
            d: csfn.fD,
            e: csfn.fE,
            f: csfn.fF
        }
    }
}

pub struct NamedTransferFn {}

#[allow(non_upper_case_globals)]
impl NamedTransferFn {
    pub const SRGB: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.4,
        a: 1.0 / 1.055,
        b: 0.055 / 1.055,
        c: 1.0 / 12.92,
        d: 0.04045,
        e: 0.0,
        f: 0.0
    };

    pub const Dot22: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.2,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0
    };

    pub const Linear: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 1.0,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0
    };
}

pub type ColorSpace = RCHandle<SkColorSpace>;

impl RefCounted for SkColorSpace {
    fn _ref(&self) {
        unsafe { rust_skia::C_SkColorSpace_ref(self) };
    }

    fn _unref(&self) {
        unsafe { rust_skia::C_SkColorSpace_unref(self) }
    }
}

impl PartialEq for ColorSpace {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { rust_skia::SkColorSpace_Equals(self.native(), rhs.native()) }
    }
}

impl ColorSpace {
    pub fn new_srgb() -> ColorSpace {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeSRGB() }).unwrap()
    }

    pub fn new_srgb_linear() -> ColorSpace {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeSRGBLinear() }).unwrap()
    }

    pub fn gamma_named(&self) -> GammaNamed {
        GammaNamed(unsafe { rust_skia::C_SkColorSpace_gammaNamed(self.native()) })
    }

    pub fn gamma_close_to_srgb(&self) -> bool {
        unsafe { self.native().gammaCloseToSRGB() }
    }

    pub fn gamma_is_linear(&self) -> bool {
        unsafe { self.native().gammaIsLinear() }
    }

    pub fn is_numerical_transfer_fn(&self) -> Option<ColorSpaceTransferFn> {
        let mut tfn : SkColorSpaceTransferFn = unsafe { mem::zeroed() };
        if unsafe { self.native().isNumericalTransferFn(&mut tfn) } {
            Some (tfn.into())
        } else {
            None
        }
    }

    pub fn to_xyzd50_hash(&self) -> XYZD50Hash {
        XYZD50Hash(unsafe { self.native().toXYZD50Hash() })
    }

    pub fn with_linear_gamma(&self) -> ColorSpace {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_makeLinearGamma(self.native()) }).unwrap()
    }

    pub fn with_srgb_gamma(&self) -> ColorSpace {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_makeSRGBGamma(self.native()) }).unwrap()
    }

    pub fn with_color_spin(&self) -> ColorSpace {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_makeColorSpin(self.native()) }).unwrap()
    }

    pub fn is_srgb(&self) -> bool {
        unsafe { self.native().isSRGB() }
    }

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe { rust_skia::C_SkColorSpace_serialize(self.native())}).unwrap()
    }

    pub fn deserialize(data: Data) -> ColorSpace {
        let bytes = data.bytes();
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_Deserialize(bytes.as_ptr() as _, bytes.len()) }).unwrap()
    }
}

pub trait NewRGB<T> {
    fn new_rgb(v: T) -> Self;
}

type RGB1 = (ColorSpaceRenderTargetGamma, ColorSpaceGamut);
type RGB2 = (ColorSpaceRenderTargetGamma, Matrix44);
type RGB3 = (ColorSpaceTransferFn, ColorSpaceGamut);
type RGB4 = (ColorSpaceTransferFn, Matrix44);
type RGB5 = (GammaNamed, Matrix44);

impl NewRGB<RGB1> for ColorSpace {
    fn new_rgb(v: RGB1) -> Self {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeRGB((v.0).0, (v.1).0) }).unwrap()
    }
}

impl NewRGB<RGB2> for ColorSpace {
    fn new_rgb(v: RGB2) -> Self {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeRGB2((v.0).0, &(v.1).0) }).unwrap()
    }
}

impl NewRGB<RGB3> for ColorSpace {
    fn new_rgb(v: RGB3) -> Self {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeRGB3(&v.0.into(), (v.1).0) }).unwrap()
    }
}

impl NewRGB<RGB4> for ColorSpace {
    fn new_rgb(v: RGB4) -> Self {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeRGB4(&v.0.into(), &(v.1).0) }).unwrap()
    }
}

impl NewRGB<RGB5> for ColorSpace {
    fn new_rgb(v: RGB5) -> Self {
        ColorSpace::from_ptr(unsafe { rust_skia::C_SkColorSpace_MakeRGB5((v.0).0, &(v.1).0) }).unwrap()
    }
}

pub struct ColorSpaceRenderTargetGamma(pub(crate) SkColorSpace_RenderTargetGamma);

#[allow(non_upper_case_globals)]
impl ColorSpaceRenderTargetGamma {
    pub const Linear: ColorSpaceRenderTargetGamma = ColorSpaceRenderTargetGamma(SkColorSpace_RenderTargetGamma::kLinear_RenderTargetGamma);
    pub const SRGB: ColorSpaceRenderTargetGamma = ColorSpaceRenderTargetGamma(SkColorSpace_RenderTargetGamma::kSRGB_RenderTargetGamma);
}

pub struct ColorSpaceGamut(pub(crate) SkColorSpace_Gamut);

#[allow(non_upper_case_globals)]
impl ColorSpaceGamut {
    pub const SRGB: ColorSpaceGamut = ColorSpaceGamut(SkColorSpace_Gamut::kSRGB_Gamut);
    pub const AdobeRGB: ColorSpaceGamut = ColorSpaceGamut(SkColorSpace_Gamut::kAdobeRGB_Gamut);
    pub const DCIP3_D65: ColorSpaceGamut = ColorSpaceGamut(SkColorSpace_Gamut::kDCIP3_D65_Gamut);
    pub const Rec2020: ColorSpaceGamut = ColorSpaceGamut(SkColorSpace_Gamut::kRec2020_Gamut);
}

pub struct XYZD50Hash(pub u32);

#[cfg(test)]
impl RefCount for ColorSpace {
    fn ref_cnt(&self) -> i32 {
        unsafe { self.native().ref_cnt() }
    }
}

#[test]
pub fn create_and_clone_colorspaces() {
    ColorSpace::new_rgb((ColorSpaceRenderTargetGamma::Linear, ColorSpaceGamut::AdobeRGB));
    let x = ColorSpace::new_rgb((ColorSpaceRenderTargetGamma::Linear, Matrix44::new_identity()));
    x.clone();
}

#[test]
pub fn serialize_and_deserialize() {
    let original = ColorSpace::new_rgb((ColorSpaceRenderTargetGamma::Linear, ColorSpaceGamut::AdobeRGB));
    unsafe { assert_eq!(1, original.native().ref_cnt()) };
    let serialized = original.serialize();
    unsafe { assert_eq!(1, serialized.native().ref_cnt()) };
    let deserialized = ColorSpace::deserialize(serialized);
    unsafe { assert_eq!(1, deserialized.native().ref_cnt()) };
    assert!(original == deserialized);
}
