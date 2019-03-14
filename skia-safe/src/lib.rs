pub mod graphics;
pub mod skia;
pub mod effects;
mod prelude;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;


#[cfg(test)]
mod transmutation_tests {

    use crate::prelude::NativeTransmutableSliceAccess;
    use crate::skia::Point;
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