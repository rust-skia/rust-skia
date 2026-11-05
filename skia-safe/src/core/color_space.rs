use super::Data;
use crate::prelude::*;
use sb::SkNVRefCnt;
use skia_bindings::{
    self as sb, SkColorSpace, SkColorSpacePrimaries, skcms_Matrix3x3, skcms_TransferFunction,
};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct ColorSpacePrimaries {
    pub rx: f32,
    pub ry: f32,
    pub gx: f32,
    pub gy: f32,
    pub bx: f32,
    pub by: f32,
    pub wx: f32,
    pub wy: f32,
}

native_transmutable!(SkColorSpacePrimaries, ColorSpacePrimaries);

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct ColorSpaceTransferFn {
    pub g: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

native_transmutable!(skcms_TransferFunction, ColorSpaceTransferFn);

/// A row-major 3x3 matrix that converts linear RGB into the XYZ D50 color space, and so defines
/// the gamut of a [`ColorSpace`].
///
/// Skia names this type `skcms_Matrix3x3`. The Rust name follows [`ColorSpacePrimaries`] and
/// [`ColorSpaceTransferFn`] and names what the matrix is for.
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct ColorSpaceXYZ {
    pub vals: [[f32; 3]; 3],
}

native_transmutable!(skcms_Matrix3x3, ColorSpaceXYZ);

/// Color primaries defined by ITU-T H.273, table 2. Names are given by the first
/// specification referenced in the value's row.
pub mod named_primaries {
    use super::ColorSpacePrimaries;
    use skia_bindings::SkNamedPrimaries_CicpId;

    /// Rec. ITU-R BT.709-6, value 1.
    pub const REC709: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.64,
        ry: 0.33,
        gx: 0.3,
        gy: 0.6,
        bx: 0.15,
        by: 0.06,
        wx: 0.3127,
        wy: 0.329,
    };

    /// Rec. ITU-R BT.470-6 System M (historical), value 4.
    pub const REC470_SYSTEM_M: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.67,
        ry: 0.33,
        gx: 0.21,
        gy: 0.71,
        bx: 0.14,
        by: 0.08,
        wx: 0.31,
        wy: 0.316,
    };

    /// Rec. ITU-R BT.470-6 System B, G (historical), value 5.
    pub const REC470_SYSTEM_BG: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.64,
        ry: 0.33,
        gx: 0.29,
        gy: 0.60,
        bx: 0.15,
        by: 0.06,
        wx: 0.3127,
        wy: 0.3290,
    };

    /// Rec. ITU-R BT.601-7 525, value 6.
    pub const REC601: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.630,
        ry: 0.340,
        gx: 0.310,
        gy: 0.595,
        bx: 0.155,
        by: 0.070,
        wx: 0.3127,
        wy: 0.3290,
    };

    /// SMPTE ST 240, value 7 (functionally the same as value 6).
    pub const SMPTE_ST_240: ColorSpacePrimaries = REC601;

    /// Generic film (colour filters using Illuminant C), value 8.
    pub const GENERIC_FILM: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.681,
        ry: 0.319,
        gx: 0.243,
        gy: 0.692,
        bx: 0.145,
        by: 0.049,
        wx: 0.310,
        wy: 0.316,
    };

    /// Rec. ITU-R BT.2020-2, value 9.
    pub const REC2020: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.708,
        ry: 0.292,
        gx: 0.170,
        gy: 0.797,
        bx: 0.131,
        by: 0.046,
        wx: 0.3127,
        wy: 0.3290,
    };

    /// SMPTE ST 428-1, value 10.
    pub const SMPTE_ST_428_1: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 1.0,
        ry: 0.0,
        gx: 0.0,
        gy: 1.0,
        bx: 0.0,
        by: 0.0,
        wx: 1.0 / 3.0,
        wy: 1.0 / 3.0,
    };

    /// SMPTE RP 431-2, value 11.
    pub const SMPTE_RP_431_2: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.680,
        ry: 0.320,
        gx: 0.265,
        gy: 0.690,
        bx: 0.150,
        by: 0.060,
        wx: 0.314,
        wy: 0.351,
    };

    /// SMPTE EG 432-1, value 12.
    pub const SMPTE_EG_432_1: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.680,
        ry: 0.320,
        gx: 0.265,
        gy: 0.690,
        bx: 0.150,
        by: 0.060,
        wx: 0.3127,
        wy: 0.3290,
    };

    /// No corresponding industry specification identified, value 22.
    /// This is sometimes referred to as EBU 3213-E, but that document doesn't
    /// specify these values.
    pub const ITU_T_H273_VALUE22: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.630,
        ry: 0.340,
        gx: 0.295,
        gy: 0.605,
        bx: 0.155,
        by: 0.077,
        wx: 0.3127,
        wy: 0.3290,
    };

    /// Mapping between names of color primaries and the number of the corresponding
    /// row in ITU-T H.273, table 2.  As above, the constants are named based on the
    /// first specification referenced in the value's row.
    pub type CicpId = SkNamedPrimaries_CicpId;
    variant_name!(CicpId::GenericFilm);

    /// <https://www.w3.org/TR/css-color-4/#predefined-prophoto-rgb>
    pub const PRO_PHOTO_RGB: ColorSpacePrimaries = ColorSpacePrimaries {
        rx: 0.7347,
        ry: 0.2653,
        gx: 0.1596,
        gy: 0.8404,
        bx: 0.0366,
        by: 0.0001,
        wx: 0.34567,
        wy: 0.35850,
    };
}

// TODO: Make the binding generator provide all these constants.
pub mod named_transfer_fn {
    //! Named [`crate::ColorSpaceTransferFn`] constants for common transfer functions.
    use crate::ColorSpaceTransferFn;
    use skia_bindings::SkNamedTransferFn_CicpId;

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

    // Transfer function defined by ITU-T H.273, table 3. Names are given by the
    // first specification referenced in the value's row. The equations in table 3
    // "either indicates the reference `[OETF]` ... or indicates the inverse of the
    // reference EOTF". The transfer functions provided are reference EOTFs.

    /// Rec. ITU-R BT.709-6, value 1. This follows note 1, which reads: "In the cases
    /// of [...] TransferCharacteristics equal to 1, 6, 14 or 15 [...], although the
    /// value is defined in terms of a reference `[OETF]`, a suggested corresponding
    /// reference `[EOTF]` has been specified in Rec. ITU-R BT.1886-0."
    pub const REC709: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.4,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// Rec. ITU-R BT.470-6 System M (historical) assumed display gamma 2.2, value 4.
    pub const REC470_SYSTEM_M: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.2,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// Rec. ITU-R BT.470-6 System B, G (historical) assumed display gamma 2.8, value 5.
    pub const REC470_SYSTEM_BG: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.8,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// Rec. ITU-R BT.601-7, same as kRec709, value 6.
    pub const REC601: ColorSpaceTransferFn = REC709;

    /// SMPTE ST 240, value 7.
    #[allow(clippy::excessive_precision)]
    pub const SMPTE_ST_240: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.222222222222,
        a: 0.899626676224,
        b: 0.100373323776,
        c: 0.25,
        d: 0.091286342118,
        e: 0.0,
        f: 0.0,
    };

    /// Linear, value 8
    pub const LINEAR: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 1.0,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// IEC 61966-2-4, value 11, same as REC709 (but is explicitly extended).
    pub const IEC61966_2_4: ColorSpaceTransferFn = REC709;

    /// IEC 61966-2-1 sRGB, value 13.
    pub const IEC61966_2_1: ColorSpaceTransferFn = SRGB;

    /// Rec. ITU-R BT.2020-2 (10-bit system), value 14.
    pub const REC2020_10BIT: ColorSpaceTransferFn = REC709;

    /// Rec. ITU-R BT.2020-2 (12-bit system), value 15.
    pub const REC2020_12BIT: ColorSpaceTransferFn = REC709;

    /// Rec. ITU-R BT.2100-2 perceptual quantization (PQ) system, value 16.
    pub const PQ: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: -5.0,
        a: 203.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// SMPTE ST 428-1, value 17.
    #[allow(clippy::excessive_precision)]
    pub const SMPTE_ST_428_1: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 2.6,
        a: 1.034080527699,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// Rec. ITU-R BT.2100-2 hybrid log-gamma (HLG) system, value 18.
    pub const HLG: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: -6.0,
        a: 203.0,
        b: 1000.0,
        c: 1.2,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// Mapping between transfer function names and the number of the corresponding
    /// row in ITU-T H.273, table 3.  As above, the constants are named based on the
    /// first specification referenced in the value's row.
    pub type CicpId = SkNamedTransferFn_CicpId;
    variant_name!(CicpId::Linear);

    /// <https://w3.org/TR/css-color-4/#valdef-color-prophoto-rgb>
    /// "The transfer curve is a gamma function with a value of 1/1.8"
    pub const PRO_PHOTO_RGB: ColorSpaceTransferFn = ColorSpaceTransferFn {
        g: 1.8,
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };

    /// <https://www.w3.org/TR/css-color-4/#predefined-a98-rgb>
    pub const A98_RGB: ColorSpaceTransferFn = DOT22;
}

pub mod named_gamut {
    //! Named [`crate::ColorSpaceXYZ`] constants for common gamuts.
    use crate::ColorSpaceXYZ;

    // ICC fixed-point (16.16) representation, taken from skcms. Please keep them exactly in sync.
    pub const SRGB: ColorSpaceXYZ = ColorSpaceXYZ {
        vals: [
            [fixed(0x6FA2), fixed(0x6299), fixed(0x24A0)],
            [fixed(0x38F5), fixed(0xB785), fixed(0x0F84)],
            [fixed(0x0390), fixed(0x18DA), fixed(0xB6CF)],
        ],
    };

    // ICC fixed-point (16.16) representation.
    pub const ADOBE_RGB: ColorSpaceXYZ = ColorSpaceXYZ {
        vals: [
            [fixed(0x9c18), fixed(0x348d), fixed(0x2631)],
            [fixed(0x4fa5), fixed(0xa02c), fixed(0x102f)],
            [fixed(0x04fc), fixed(0x0f95), fixed(0xbe9c)],
        ],
    };

    pub const DISPLAY_P3: ColorSpaceXYZ = ColorSpaceXYZ {
        vals: [
            [0.515102, 0.291965, 0.157153],
            [0.241182, 0.692236, 0.0665819],
            [-0.00104941, 0.0418818, 0.784378],
        ],
    };

    pub const REC2020: ColorSpaceXYZ = ColorSpaceXYZ {
        vals: [
            [0.673459, 0.165661, 0.125100],
            [0.279033, 0.675338, 0.0456288],
            [-0.00193139, 0.0299794, 0.797162],
        ],
    };

    pub const XYZ: ColorSpaceXYZ = ColorSpaceXYZ {
        vals: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    /// Converts a 16.16 fixed-point value to a float, like Skia's `SkFixedToFloat`, so the
    /// constants above keep the exact values of Skia's header.
    const fn fixed(value: i32) -> f32 {
        value as f32 * (1.0 / 65536.0)
    }
}

pub type ColorSpace = RCHandle<SkColorSpace>;
unsafe_send_sync!(ColorSpace);
require_base_type!(SkColorSpace, SkNVRefCnt);

impl NativeRefCounted for SkColorSpace {
    unsafe fn _ref(ptr: *const Self) {
        unsafe { sb::C_SkColorSpace_ref(ptr) };
    }

    unsafe fn _unref(ptr: *const Self) {
        unsafe { sb::C_SkColorSpace_unref(ptr) }
    }

    unsafe fn unique(ptr: *const Self) -> bool {
        unsafe { sb::C_SkColorSpace_unique(ptr) }
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
    /// Creates the sRGB color space.
    pub fn new_srgb() -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeSRGB() }).unwrap()
    }

    /// Creates a color space with the sRGB primaries, but a linear (1.0) gamma.
    pub fn new_srgb_linear() -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeSRGBLinear() }).unwrap()
    }

    /// Creates a [`ColorSpace`] from a parsed (skcms) ICC profile. Returns `None` if `data` is not
    /// a valid ICC profile.
    ///
    /// - `data` the ICC profile bytes
    pub fn new_icc(data: &[u8]) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeICC(data.as_ptr() as _, data.len()) })
    }

    /// Create a [`ColorSpace`] from a transfer function and a row-major 3x3 transformation to XYZ.
    ///
    /// Returns `None` if the transfer function is invalid.
    pub fn new_rgb(transfer_fn: &ColorSpaceTransferFn, to_xyz: &ColorSpaceXYZ) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeRGB(transfer_fn.native(), to_xyz.native()) })
    }

    /// Creates a [`ColorSpace`] from code points specified in Rec. ITU-T H.273. Returns `None` for
    /// an invalid or unsupported combination of code points.
    ///
    /// - `primaries` identifies an entry in Rec. ITU-T H.273, Table 2
    /// - `transfer_characteristics` identifies an entry in Rec. ITU-T H.273, Table 3
    ///
    /// [`ColorSpace`] (and the underlying `skcms_ICCProfile`) only supports RGB color spaces and
    /// therefore this function does not take a `matrix_coefficients` parameter; the caller is
    /// expected to verify that `matrix_coefficients` is `0`.
    ///
    /// Narrow range images are extremely rare, so this function does not take a
    /// `video_full_range_flag`; the caller is expected to verify that it is `1` (indicating a full
    /// range image).
    pub fn new_cicp(
        primaries: named_primaries::CicpId,
        transfer_characteristics: named_transfer_fn::CicpId,
    ) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_MakeCICP(primaries, transfer_characteristics) })
    }

    /// Returns a hash of the gamut transformation to XYZ D50. Allows for fast equality checking of
    /// gamuts, at the (very small) risk of collision.
    pub fn to_xyzd50_hash(&self) -> XYZD50Hash {
        XYZD50Hash(self.native().fToXYZD50Hash)
    }

    /// Returns a color space with the same gamut as this one, but with a linear gamma.
    #[must_use]
    pub fn with_linear_gamma(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_makeLinearGamma(self.native()) }).unwrap()
    }

    /// Returns a color space with the same gamut as this one, but with the sRGB transfer function.
    #[must_use]
    pub fn with_srgb_gamma(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_makeSRGBGamma(self.native()) }).unwrap()
    }

    /// Returns a color space with the same transfer function as this one, but with the primary
    /// colors rotated. In other words, this produces a new color space that maps RGB to GBR (when
    /// applied to a source), and maps RGB to BRG (when applied to a destination).
    ///
    /// This is used for testing, to construct color spaces that have severe and testable behavior.
    #[must_use]
    pub fn with_color_spin(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkColorSpace_makeColorSpin(self.native()) }).unwrap()
    }

    /// Returns true if the color space is sRGB. Returns false otherwise.
    ///
    /// This allows a little bit of tolerance, given that we might see small numerical error in
    /// some cases: converting ICC fixed point to float, converting white point to D50, rounding
    /// decisions on transfer function and matrix.
    ///
    /// This does not consider a 2.2f exponential transfer function to be sRGB. While these
    /// functions are similar (and it is sometimes useful to consider them together), this function
    /// checks for logical equality.
    pub fn is_srgb(&self) -> bool {
        unsafe { self.native().isSRGB() }
    }

    /// Returns a serialized representation of this color space.
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

    pub fn transfer_fn(&self) -> ColorSpaceTransferFn {
        let mut transfer_fn = ColorSpaceTransferFn {
            g: 0.0,
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 0.0,
        };
        unsafe { self.native().transferFn1(transfer_fn.native_mut()) };
        transfer_fn
    }

    pub fn inv_transfer_fn(&self) -> ColorSpaceTransferFn {
        let mut transfer_fn = ColorSpaceTransferFn {
            g: 0.0,
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 0.0,
        };
        unsafe { self.native().invTransferFn(transfer_fn.native_mut()) };
        transfer_fn
    }

    // TODO: gamutTransformTo()

    pub fn transfer_fn_hash(&self) -> u32 {
        unsafe { sb::C_SkColorSpace_transferFnHash(self.native()) }
    }

    pub fn hash(&self) -> u64 {
        unsafe { sb::C_SkColorSpace_hash(self.native()) }
    }
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
    // The deserializer deduplicates the sRGB colorspace against the global
    // cache, so `deserialized` must share the same underlying instance as
    // `original`, even though `serialize()` produces an independent `Data` blob.
    let original = ColorSpace::new_srgb();
    let serialized = original.serialize();
    let deserialized = ColorSpace::deserialize(serialized);

    // deserialize() deduplicates back to the cached sRGB instance.
    assert!(std::ptr::eq(
        original.native() as *const _,
        deserialized.native() as *const _,
    ));
    assert!(original == deserialized);
}

/// The fixed-point conversion of `named_gamut::SRGB` keeps the exact values of Skia's header.
/// `ColorSpace::new_rgb()` tolerates small differences, so the test reads Skia's sRGB gamut back.
#[test]
fn named_srgb_gamut_equals_skias_srgb_gamut() {
    let mut to_xyz = ColorSpaceXYZ {
        vals: [[0.0; 3]; 3],
    };
    assert!(unsafe {
        sb::SkColorSpace_toXYZD50(ColorSpace::new_srgb().native(), to_xyz.native_mut())
    });
    assert_eq!(to_xyz, named_gamut::SRGB);
}
