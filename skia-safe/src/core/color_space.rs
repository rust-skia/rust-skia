use super::Data;
use crate::prelude::*;
use sb::SkNVRefCnt;
use skia_bindings::{self as sb, SkColorSpace, SkColorSpacePrimaries};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct ColorSpacePrimaries {
    rx: f32,
    ry: f32,
    gx: f32,
    gy: f32,
    bx: f32,
    by: f32,
    wx: f32,
    wy: f32,
}

native_transmutable!(
    SkColorSpacePrimaries,
    ColorSpacePrimaries,
    color_space_primaries_layout
);

#[derive(Clone, PartialEq, Debug)]
pub struct ColorSpaceTransferFn {
    pub g: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

// TODO: Make the binding generator provide all these constants.
pub mod named_transfer_fn {
    use crate::ColorSpaceTransferFn;

    pub const SRGB: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.4,
        a: 1.0 / 1.055,
        b: 0.055 / 1.055,
        c: 1.0 / 12.92,
        d: 0.04045,
        e: 0.0,
        f: 0.0,
    };

    pub const DOT22: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.2,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    pub const LINEAR: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 1.0,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    pub const REC2020: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.22222,
        a: 0.909_672,
        b: 0.090_327_6,
        c: 0.222_222,
        d: 0.081_242_9,
        e: 0.0,
        f: 0.0,
    };

    pub const PQ: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: -2.0,
        a: -107.0 / 128.0,
        b: 1.0,
        c: 32.0 / 2523.0,
        d: 2413.0 / 128.0,
        e: -2392.0 / 128.0,
        f: 8192.0 / 1305.0,
    };

    #[allow(clippy::excessive_precision)]
    pub const HLG: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: -3.0,
        a: 2.0,
        b: 2.0,
        c: 1.0 / 0.178_832_77,
        d: 0.284_668_92,
        e: 0.559_910_73,
        f: 0.0,
    };
}

pub type ColorSpace = RCHandle<SkColorSpace>;
unsafe_send_sync!(ColorSpace);
require_base_type!(SkColorSpace, SkNVRefCnt);

impl NativeRefCounted for SkColorSpace {
    fn _ref(&self) {
        unsafe { sb::C_SkColorSpace_ref(self) };
    }

    fn _unref(&self) {
        unsafe { sb::C_SkColorSpace_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkColorSpace_unique(self) }
    }
}

impl NativePartialEq for SkColorSpace {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { skia_bindings::SkColorSpace_Equals(self, rhs) }
    }
}

impl fmt::Debug for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColorSpace").finish()
    }
}

impl ColorSpace {
    pub fn new_srgb() -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeSRGB() }).unwrap()
    }

    pub fn new_srgb_linear() -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeSRGBLinear() }).unwrap()
    }

    pub fn to_xyzd50_hash(&self) -> XYZD50Hash {
        XYZD50Hash(self.native().fToXYZD50Hash)
    }

    #[must_use]
    pub fn with_linear_gamma(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_makeLinearGamma(self.native()) }).unwrap()
    }

    #[must_use]
    pub fn with_srgb_gamma(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_makeSRGBGamma(self.native()) }).unwrap()
    }

    #[must_use]
    pub fn with_color_spin(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_makeColorSpin(self.native()) }).unwrap()
    }

    pub fn is_srgb(&self) -> bool {
        unsafe { self.native().isSRGB() }
    }

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkColorSpace_serialize(self.native()) }).unwrap()
    }

    // TODO: writeToMemory()?

    pub fn deserialize(data: impl Into<Data>) -> Self {
        let data = data.into();
        let bytes = data.as_bytes();
        Self::from_ptr(unsafe { sb::C_SkColorSpace_Deserialize(bytes.as_ptr() as _, bytes.len()) })
            .unwrap()
    }

    // TODO: transferFn()
    // TODO: invTransferFn()
    // TODO: gamutTransformTo()
    // TODO: transferFnHash()?
    // TODO: hash()?
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct XYZD50Hash(pub u32);

#[cfg(test)]
impl RefCount for SkColorSpace {
    fn ref_cnt(&self) -> usize {
        self._base.ref_cnt()
    }
}

#[test]
#[serial_test::serial]
pub fn create_and_clone_colorspaces() {
    ColorSpace::new_srgb();
    let x = ColorSpace::new_srgb_linear();
    #[allow(clippy::redundant_clone)]
    let _r = x.clone();
}

#[test]
#[serial_test::serial]
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
