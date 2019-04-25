use super::{Matrix44, Data};
use crate::prelude::*;
use skia_bindings::{
    SkColorSpace,
    SkColorSpacePrimaries,
};

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
#[test] fn test_color_space_primaries_layout() { ColorSpacePrimaries::test_layout() }

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

pub struct NamedTransferFn {}

// TODO: Make the binding generator provide all these constants.
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

    pub fn to_xyzd50(&self) -> Option<Matrix44> {
        let mut matrix = Matrix44::default();
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

pub struct XYZD50Hash(pub u32);

#[cfg(test)]
impl RefCount for ColorSpace {
    fn ref_cnt(&self) -> usize {
        self.native().ref_cnt()
    }
}

#[test]
pub fn create_and_clone_colorspaces() {
    ColorSpace::new_srgb();
    let x = ColorSpace::new_srgb_linear();
    let _r = x.clone();
}

#[test]
pub fn serialize_and_deserialize() {
    // TODO: it seems that the deserializer deduplicates the
    // srgb colorspace, so fix this test as soon we can create
    // custom colorspaces again.
    let original = ColorSpace::new_srgb();
    assert_eq!(2, original.native().ref_cnt());
    let serialized = original.serialize();
    assert_eq!(1, serialized.native().ref_cnt());
    let deserialized = ColorSpace::deserialize(serialized);
    assert_eq!(3, deserialized.native().ref_cnt());

    assert!(original == deserialized);
}
