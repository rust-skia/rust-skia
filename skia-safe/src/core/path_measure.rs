use crate::prelude::*;
use crate::{scalar, Matrix, Path, Point, Vector};
use skia_bindings::{C_SkPathMeasure_destruct, SkPathMeasure};

pub type PathMeasure = Handle<SkPathMeasure>;

impl NativeDrop for SkPathMeasure {
    fn drop(&mut self) {
        // does not link:
        // unsafe { SkPathMeasure::destruct()(self) }
        unsafe { C_SkPathMeasure_destruct(self) }
    }
}

#[allow(clippy::module_inception)]
pub mod path_measure {
    bitflags! {
        pub struct MatrixFlags : u32 {
            const GET_POSITION = skia_bindings::SkPathMeasure_MatrixFlags_kGetPosition_MatrixFlag as _;
            const GET_TANGENT = skia_bindings::SkPathMeasure_MatrixFlags_kGetTangent_MatrixFlag as _;
            const GET_POS_AND_TAN = Self::GET_POSITION.bits | Self::GET_TANGENT.bits;
        }
    }

    impl Default for MatrixFlags {
        fn default() -> Self {
            Self::GET_POS_AND_TAN
        }
    }
}

impl Default for Handle<SkPathMeasure> {
    fn default() -> Self {
        Self::from_native(unsafe { SkPathMeasure::new() })
    }
}

impl Handle<SkPathMeasure> {
    // TODO: rename for_path / of_path?
    pub fn from_path<RS: Into<Option<scalar>>>(
        path: &Path,
        force_closed: bool,
        res_scale: RS,
    ) -> Self {
        Self::from_native(unsafe {
            SkPathMeasure::new1(path.native(), force_closed, res_scale.into().unwrap_or(1.0))
        })
    }

    pub fn set_path(&mut self, path: &Path, force_closed: bool) -> &mut Self {
        unsafe { self.native_mut().setPath(path.native(), force_closed) }
        self
    }

    // TODO: why is getLength() non-const.
    pub fn length(&mut self) -> scalar {
        unsafe { self.native_mut().getLength() }
    }

    // TODO: why is getPosTan() non-const?
    pub fn pos_tan(&mut self, distance: scalar) -> Option<(Point, Vector)> {
        let mut p = Point::default();
        let mut v = Vector::default();
        unsafe {
            self.native_mut()
                .getPosTan(distance, p.native_mut(), v.native_mut())
        }
        .if_true_some((p, v))
    }

    // TODO: why is getMatrix() non-const?
    pub fn matrix<F: Into<Option<path_measure::MatrixFlags>>>(
        &mut self,
        distance: scalar,
        flags: F,
    ) -> Option<Matrix> {
        let mut m = Matrix::default();
        unsafe {
            self.native_mut().getMatrix(
                distance,
                m.native_mut(),
                // note: depending on the OS, different representation types are generated for MatrixFlags
                flags.into().unwrap_or_default().bits().try_into().unwrap(),
            )
        }
        .if_true_some(m)
    }

    // TODO: why is getSegment() non-const?
    pub fn segment(
        &mut self,
        start_d: scalar,
        stop_d: scalar,
        start_with_move_to: bool,
    ) -> Option<Path> {
        let mut p = Path::default();
        unsafe {
            self.native_mut()
                .getSegment(start_d, stop_d, p.native_mut(), start_with_move_to)
        }
        .if_true_some(p)
    }

    // TODO: why is isClosed() non-const?
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed(&mut self) -> bool {
        unsafe { self.native_mut().isClosed() }
    }

    pub fn next_contour(&mut self) -> bool {
        unsafe { self.native_mut().nextContour() }
    }
}
