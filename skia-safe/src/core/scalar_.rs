#[allow(non_camel_case_types)]
pub type scalar = skia_bindings::SkScalar;

// TODO: wrap more core/SkScalar.h functions / macros.

pub trait Scalar: Copy {
    const NEARLY_ZERO: Self;
    const ONE: Self;
    const HALF: Self;
}

impl Scalar for scalar {
    const NEARLY_ZERO: Self = 1.0 / ((1 << 12) as Self);
    const ONE: Self = 1.0;
    const HALF: Self = 0.5;
}
