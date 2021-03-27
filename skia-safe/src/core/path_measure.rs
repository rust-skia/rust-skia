use crate::prelude::*;
use crate::{scalar, Matrix, Path, Point, Vector};
use skia_bindings as sb;
use skia_bindings::SkPathMeasure;

pub type PathMeasure = Handle<SkPathMeasure>;

impl NativeDrop for SkPathMeasure {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathMeasure_destruct(self) }
    }
}

bitflags! {
    pub struct MatrixFlags : u32 {
        const GET_POSITION = sb::SkPathMeasure_MatrixFlags_kGetPosition_MatrixFlag as _;
        const GET_TANGENT = sb::SkPathMeasure_MatrixFlags_kGetTangent_MatrixFlag as _;
        const GET_POS_AND_TAN = Self::GET_POSITION.bits | Self::GET_TANGENT.bits;
    }
}

impl Default for MatrixFlags {
    fn default() -> Self {
        Self::GET_POS_AND_TAN
    }
}

impl Default for Handle<SkPathMeasure> {
    fn default() -> Self {
        Self::from_native_c(unsafe { SkPathMeasure::new() })
    }
}

impl Handle<SkPathMeasure> {
    // Canonical new:
    pub fn new(path: &Path, force_closed: bool, res_scale: impl Into<Option<scalar>>) -> Self {
        Self::from_path(path, force_closed, res_scale)
    }

    // TODO: rename for_path / of_path?
    // TODO: deprecate in favor of new()?
    pub fn from_path(
        path: &Path,
        force_closed: bool,
        res_scale: impl Into<Option<scalar>>,
    ) -> Self {
        Self::from_native_c(unsafe {
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
    // TODO: rename to get_pos_tan(), because the function has arguments?
    pub fn pos_tan(&mut self, distance: scalar) -> Option<(Point, Vector)> {
        let mut position = Point::default();
        let mut tangent = Vector::default();
        unsafe {
            self.native_mut()
                .getPosTan(distance, position.native_mut(), tangent.native_mut())
        }
        .if_true_some((position, tangent))
    }

    // TODO: why is getMatrix() non-const?
    // TODO: rename to get_matrix(), because the function has arguments?
    pub fn matrix(
        &mut self,
        distance: scalar,
        flags: impl Into<Option<MatrixFlags>>,
    ) -> Option<Matrix> {
        let mut m = Matrix::default();
        unsafe {
            self.native_mut().getMatrix(
                distance,
                m.native_mut(),
                // note: depending on the OS, different representation types are generated for MatrixFlags
                #[allow(clippy::useless_conversion)]
                flags.into().unwrap_or_default().bits().try_into().unwrap(),
            )
        }
        .if_true_some(m)
    }

    // TODO: why is getSegment() non-const?
    // TODO: rename to get_segment(), because the function has arguments?
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

    // TODO: rename to has_next_contour()?
    pub fn next_contour(&mut self) -> bool {
        unsafe { self.native_mut().nextContour() }
    }
}
