#[allow(non_camel_case_types)]
pub type scalar = skia_bindings::SkScalar;

// TODO: wrap more core/SkScalar.h functions / macros.

pub trait Scalar: Copy {
    const NEARLY_ZERO: Self;
    const ONE: Self;
    const HALF: Self;

    fn nearly_equal(x: scalar, y: scalar, tolerance: impl Into<Option<scalar>>) -> bool;
    fn nearly_zero(&self, tolerance: impl Into<Option<scalar>>) -> bool;
}

impl Scalar for scalar {
    const NEARLY_ZERO: Self = 1.0 / ((1 << 12) as Self);
    const ONE: Self = 1.0;
    const HALF: Self = 0.5;

    fn nearly_equal(x: scalar, y: scalar, tolerance: impl Into<Option<scalar>>) -> bool {
        let tolerance = tolerance.into().unwrap_or(Self::NEARLY_ZERO);
        debug_assert!(tolerance >= 0.0);
        (x - y).abs() <= tolerance
    }

    fn nearly_zero(&self, tolerance: impl Into<Option<scalar>>) -> bool {
        let tolerance = tolerance.into().unwrap_or(Self::NEARLY_ZERO);
        debug_assert!(tolerance >= 0.0);
        self.abs() <= tolerance
    }
}
