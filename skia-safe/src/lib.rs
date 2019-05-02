mod prelude;
mod core;
mod docs;
mod effects;
mod interop;
mod pathops;
pub mod gpu;
#[cfg(feature = "svg")]
pub mod svg;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

/// All Sk* types are accessible via skia_safe::
pub use crate::core::*;
pub use crate::core::document::document;
pub use crate::core::contour_measure::contour_measure;
pub use crate::core::path_measure::path_measure;
pub use crate::docs::*;
pub use crate::effects::*;
pub use crate::pathops::*;

#[cfg(test)]
mod transmutation_tests {

    use crate::prelude::NativeTransmutableSliceAccess;
    use crate::core::Point;
    use skia_bindings::SkPoint;

    #[test]
    fn test_transmutation_of_fixed_size_arrays_to_slice() {
        let mut points = [Point::default(); 4];

        let points_native = points.native_mut();
        let native_point = SkPoint { fX: 10.0, fY: 11.0 };
        points_native[1] = native_point;

        assert_eq!(points[1].x, native_point.fX);
        assert_eq!(points[1].y, native_point.fY);
    }
}