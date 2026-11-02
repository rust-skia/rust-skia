//! Base class for objects in a [`crate::Paint`] that affect the geometry of a drawing primitive
//! before it is transformed by the canvas' matrix and drawn.

use crate::{Matrix, NativeFlattenable, Path, PathBuilder, Rect, StrokeRec, prelude::*};
use sb::SkPathEffect_INHERITED;
use skia_bindings::{self as sb, SkFlattenable, SkPathEffect, SkRefCntBase};
use std::fmt;

pub type PathEffect = RCHandle<SkPathEffect>;
unsafe_send_sync!(PathEffect);
require_type_equality!(SkPathEffect_INHERITED, SkFlattenable);

impl NativeBase<SkRefCntBase> for SkPathEffect {}
impl NativeBase<SkFlattenable> for SkPathEffect {}

impl NativeRefCountedBase for SkPathEffect {
    type Base = SkRefCntBase;
}

impl NativeFlattenable for SkPathEffect {
    fn native_flattenable(&self) -> &SkFlattenable {
        self.base()
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkPathEffect_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl fmt::Debug for PathEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathEffect")
            .field("needs_ctm", &self.needs_ctm())
            .finish()
    }
}

impl PathEffect {
    /// Returns a path effect that applies each effect (`first` and `second`) to the original path,
    /// and returns a path with the sum of these.
    ///
    /// `result = first(path) + second(path)`
    ///
    /// - `first` first effect
    /// - `second` second effect
    pub fn sum(first: impl Into<PathEffect>, second: impl Into<PathEffect>) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            sb::C_SkPathEffect_MakeSum(first.into().into_ptr(), second.into().into_ptr())
        })
        .unwrap()
    }

    /// Returns a path effect that applies the inner effect to the path, and then applies the outer
    /// effect to the result of the inner's.
    ///
    /// `result = outer(inner(path))`
    ///
    /// - `first` outer effect
    /// - `second` inner effect
    pub fn compose(first: impl Into<PathEffect>, second: impl Into<PathEffect>) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            sb::C_SkPathEffect_MakeCompose(first.into().into_ptr(), second.into().into_ptr())
        })
        .unwrap()
    }

    /// Given a `src` path (input) and a stroke-rec (input and output), applies this effect to the
    /// `src` path, returning the new path in `dst`, and returns true. If this effect cannot be
    /// applied, returns false and ignores `dst` and the stroke-rec.
    ///
    /// The stroke-rec specifies the initial request for stroking (if any). The effect can treat
    /// this as input only, or it can choose to change the rec as well. For example, the effect can
    /// decide to change the stroke's width or join, or the effect can change the rec from stroke to
    /// fill (or fill to stroke) in addition to returning a new (`dst`) path.
    ///
    /// If this method returns true, the caller will apply (as needed) the resulting stroke-rec to
    /// `dst` and then draw.
    ///
    /// - `src` source path
    /// - `stroke_rec` stroke record
    /// - `cull_rect` cull rectangle
    pub fn filter_path(
        &self,
        src: &Path,
        stroke_rec: &StrokeRec,
        cull_rect: impl AsRef<Rect>,
    ) -> Option<(PathBuilder, StrokeRec)> {
        let mut dst = PathBuilder::new();
        let mut stroke_rec_r = stroke_rec.clone();
        self.filter_path_inplace(&mut dst, src, &mut stroke_rec_r, cull_rect)
            .then_some((dst, stroke_rec_r))
    }

    /// Applies this effect to the `src` path, returning the new path in `dst`. See
    /// [`Self::filter_path()`] for the full contract.
    ///
    /// - `dst` destination path builder
    /// - `src` source path
    /// - `stroke_rec` stroke record
    /// - `cull_rect` cull rectangle
    pub fn filter_path_inplace(
        &self,
        dst: &mut PathBuilder,
        src: &Path,
        stroke_rec: &mut StrokeRec,
        cull_rect: impl AsRef<Rect>,
    ) -> bool {
        unsafe {
            self.native().filterPath(
                dst.native_ptr(),
                src.native(),
                stroke_rec.native_mut(),
                cull_rect.as_ref().native(),
                Matrix::new_identity().native(),
            )
        }
    }

    /// Applies this effect to the `src` path, returning the new path in `dst`. See
    /// [`Self::filter_path()`] for the full contract.
    ///
    /// - `dst` destination path builder
    /// - `src` source path
    /// - `stroke_rec` stroke record
    /// - `cull_rect` cull rectangle
    /// - `ctm` current transformation matrix
    pub fn filter_path_inplace_with_matrix(
        &self,
        dst: &mut PathBuilder,
        src: &Path,
        stroke_rec: &mut StrokeRec,
        cull_rect: impl AsRef<Rect>,
        ctm: &Matrix,
    ) -> bool {
        unsafe {
            self.native().filterPath(
                dst.native_ptr(),
                src.native(),
                stroke_rec.native_mut(),
                cull_rect.as_ref().native(),
                ctm.native(),
            )
        }
    }

    /// Returns true if this path effect requires a valid CTM.
    pub fn needs_ctm(&self) -> bool {
        unsafe { self.native().needsCTM() }
    }
}
