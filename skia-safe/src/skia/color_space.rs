use std::mem;
use super::{Matrix44, Data};
use crate::prelude::*;
use skia_bindings::{
    SkColorSpaceTransferFn,
    SkColorSpace,
    SkColorSpacePrimaries,
    SkGammaNamed,
    SkColorSpace_Gamut,
    SkColorSpace_RenderTargetGamma,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum GammaNamed {
    Linear = SkGammaNamed::kLinear_SkGammaNamed as _,
    SRGB = SkGammaNamed::kSRGB_SkGammaNamed as _,
    Curve2Dot2 = SkGammaNamed::k2Dot2Curve_SkGammaNamed as _,
    NonStandard = SkGammaNamed::kNonStandard_SkGammaNamed as _
}

impl NativeTransmutable<SkGammaNamed> for GammaNamed {}
#[test] fn test_gamma_named_layout() { GammaNamed::test_layout() }

#[derive(Copy, Clone, PartialEq, Debug)]
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

impl NativeTransmutable<SkColorSpacePrimaries> for ColorSpacePrimaries {}

#[test]
fn test_color_space_primaries_layout() {
    ColorSpacePrimaries::test_layout()
}

impl ColorSpacePrimaries {

    pub fn to_xyzd50(&self) -> Option<Matrix44> {
        let mut matrix = Matrix44::new();
        unsafe { self.native().toXYZD50(matrix.native_mut()) }
            .if_true_some(matrix)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorSpaceTransferFn {
    pub g: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32
}

impl NativeTransmutable<SkColorSpaceTransferFn> for ColorSpaceTransferFn {}

#[test]
fn test_color_space_transfer_fn_layout() {
    ColorSpaceTransferFn::test_layout()
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

impl NativeRefCounted for SkColorSpace {
    fn _ref(&self) {
        unsafe { skia_bindings::C_SkColorSpace_ref(self) };
    }

    fn _unref(&self) {
        unsafe { skia_bindings::C_SkColorSpace_unref(self) }
    }
}

impl NativePartialEq for SkColorSpace {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { skia_bindings::SkColorSpace_Equals(self, rhs) }
    }
}

impl ColorSpace {
    pub fn new_srgb() -> ColorSpace {
        ColorSpace::from_ptr(unsafe { skia_bindings::C_SkColorSpace_MakeSRGB() }).unwrap()
    }

    pub fn new_srgb_linear() -> ColorSpace {
        ColorSpace::from_ptr(unsafe { skia_bindings::C_SkColorSpace_MakeSRGBLinear() }).unwrap()
    }

    pub fn gamma_named(&self) -> GammaNamed {
        GammaNamed::from_native(unsafe {
            skia_bindings::C_SkColorSpace_gammaNamed(self.native())
        })
    }

    pub fn gamma_close_to_srgb(&self) -> bool {
        unsafe { self.native().gammaCloseToSRGB() }
    }

    pub fn gamma_is_linear(&self) -> bool {
        unsafe { self.native().gammaIsLinear() }
    }

    pub fn is_numerical_transfer_fn(&self) -> Option<ColorSpaceTransferFn> {
        let mut tfn : ColorSpaceTransferFn = unsafe { mem::zeroed() };
        unsafe {
            self.native().isNumericalTransferFn(tfn.native_mut())
        }.if_true_some(tfn)
    }

    pub fn to_xyzd50(&self) -> Option<Matrix44> {
        let mut matrix = Matrix44::new();
        unsafe { self.native().toXYZD50(matrix.native_mut()) }
            .if_true_some(matrix)
    }

    pub fn to_xyzd50_hash(&self) -> XYZD50Hash {
        XYZD50Hash(unsafe { self.native().toXYZD50Hash() })
    }

    #[must_use]
    pub fn with_linear_gamma(&self) -> ColorSpace {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_makeLinearGamma(self.native())
        }).unwrap()
    }

    #[must_use]
    pub fn with_srgb_gamma(&self) -> ColorSpace {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_makeSRGBGamma(self.native())
        }).unwrap()
    }

    pub fn with_color_spin(&self) -> ColorSpace {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_makeColorSpin(self.native())
        }).unwrap()
    }

    pub fn is_srgb(&self) -> bool {
        unsafe { self.native().isSRGB() }
    }

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_serialize(self.native())
        }).unwrap()
    }

    pub fn deserialize(data: Data) -> ColorSpace {
        let bytes = data.bytes();
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_Deserialize(bytes.as_ptr() as _, bytes.len())
        }).unwrap()
    }
}

pub trait NewRGB<T> {
    fn new_rgb(v: T) -> Self;
}

// TODO: should we use references for the heavier types?

type RGB1 = (ColorSpaceRenderTargetGamma, ColorSpaceGamut);
type RGB2 = (ColorSpaceRenderTargetGamma, Matrix44);
type RGB3 = (ColorSpaceTransferFn, ColorSpaceGamut);
type RGB4 = (ColorSpaceTransferFn, Matrix44);
type RGB5 = (GammaNamed, Matrix44);

impl NewRGB<RGB1> for RCHandle<SkColorSpace> {
    fn new_rgb(v: RGB1) -> Self {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_MakeRGB((v.0).into_native(), (v.1).into_native())
        }).unwrap()
    }
}

impl NewRGB<RGB2> for RCHandle<SkColorSpace> {
    fn new_rgb(v: RGB2) -> Self {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_MakeRGB2((v.0).into_native(), v.1.native())
        }).unwrap()
    }
}

impl NewRGB<RGB3> for RCHandle<SkColorSpace> {
    fn new_rgb(v: RGB3) -> Self {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_MakeRGB3(v.0.native(), v.1.into_native())
        }).unwrap()
    }
}

impl NewRGB<RGB4> for RCHandle<SkColorSpace> {
    fn new_rgb(v: RGB4) -> Self {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_MakeRGB4(v.0.native(), v.1.native())
        }).unwrap()
    }
}

impl NewRGB<RGB5> for RCHandle<SkColorSpace> {
    fn new_rgb(v: RGB5) -> Self {
        ColorSpace::from_ptr(unsafe {
            skia_bindings::C_SkColorSpace_MakeRGB5((v.0).into_native(), v.1.native())
        }).unwrap()
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ColorSpaceRenderTargetGamma {
    Linear = SkColorSpace_RenderTargetGamma::kLinear_RenderTargetGamma as _,
    SRGB = SkColorSpace_RenderTargetGamma::kSRGB_RenderTargetGamma as _
}

impl NativeTransmutable<SkColorSpace_RenderTargetGamma> for ColorSpaceRenderTargetGamma {}
#[test] fn test_color_space_render_target_gamma_layout() { ColorSpaceRenderTargetGamma::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ColorSpaceGamut {
    SRGB = SkColorSpace_Gamut::kSRGB_Gamut as _,
    AdobeRGB = SkColorSpace_Gamut::kAdobeRGB_Gamut as _,
    DCIP3D65 = SkColorSpace_Gamut::kDCIP3_D65_Gamut as _,
    Rec2020 = SkColorSpace_Gamut::kRec2020_Gamut as _
}

impl NativeTransmutable<SkColorSpace_Gamut> for ColorSpaceGamut {}
#[test] fn test_color_space_gamut_layout () { ColorSpaceGamut::test_layout() }

pub struct XYZD50Hash(pub u32);

#[cfg(test)]
impl RefCount for ColorSpace {
    fn ref_cnt(&self) -> usize {
        self.native().ref_cnt()
    }
}

#[test]
pub fn create_and_clone_colorspaces() {
    ColorSpace::new_rgb((ColorSpaceRenderTargetGamma::Linear, ColorSpaceGamut::AdobeRGB));
    let x = ColorSpace::new_rgb((ColorSpaceRenderTargetGamma::Linear, Matrix44::new_identity()));
    let _r = x.clone();
}

#[test]
pub fn serialize_and_deserialize() {
    let original = ColorSpace::new_rgb(
        (ColorSpaceRenderTargetGamma::Linear, ColorSpaceGamut::AdobeRGB)
    );
    assert_eq!(1, original.native().ref_cnt());
    let serialized = original.serialize();
    assert_eq!(1, serialized.native().ref_cnt());
    let deserialized = ColorSpace::deserialize(serialized);
    assert_eq!(1, deserialized.native().ref_cnt());

    assert!(original == deserialized);
}
