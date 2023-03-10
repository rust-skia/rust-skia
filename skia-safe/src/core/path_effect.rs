use crate::{prelude::*, scalar, Matrix, NativeFlattenable, Path, Rect, StrokeRec};
use sb::SkPathEffect_INHERITED;
use skia_bindings::{self as sb, SkFlattenable, SkPathEffect, SkPathEffect_DashType, SkRefCntBase};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct DashInfo {
    pub intervals: Vec<scalar>,
    pub phase: scalar,
}

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
            .field("as_a_dash", &self.as_a_dash())
            .field("needs_ctm", &self.needs_ctm())
            .finish()
    }
}

impl PathEffect {
    pub fn sum(first: impl Into<PathEffect>, second: impl Into<PathEffect>) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            sb::C_SkPathEffect_MakeSum(first.into().into_ptr(), second.into().into_ptr())
        })
        .unwrap()
    }

    pub fn compose(first: impl Into<PathEffect>, second: impl Into<PathEffect>) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            sb::C_SkPathEffect_MakeCompose(first.into().into_ptr(), second.into().into_ptr())
        })
        .unwrap()
    }

    // TODO: rename to to_a_dash()?
    pub fn as_a_dash(&self) -> Option<DashInfo> {
        let mut dash_info = construct(|di| unsafe { sb::C_SkPathEffect_DashInfo_Construct(di) });

        let dash_type = unsafe { self.native().asADash(&mut dash_info) };

        match dash_type {
            SkPathEffect_DashType::kDash_DashType => {
                let mut v: Vec<scalar> = vec![0.0; dash_info.fCount.try_into().unwrap()];
                dash_info.fIntervals = v.as_mut_ptr();
                unsafe {
                    assert_eq!(dash_type, self.native().asADash(&mut dash_info));
                }
                Some(DashInfo {
                    intervals: v,
                    phase: dash_info.fPhase,
                })
            }
            SkPathEffect_DashType::kNone_DashType => None,
        }
    }

    pub fn filter_path(
        &self,
        src: &Path,
        stroke_rec: &StrokeRec,
        cull_rect: impl AsRef<Rect>,
    ) -> Option<(Path, StrokeRec)> {
        let mut dst = Path::default();
        let mut stroke_rec_r = stroke_rec.clone();
        self.filter_path_inplace(&mut dst, src, &mut stroke_rec_r, cull_rect)
            .if_true_some((dst, stroke_rec_r))
    }

    pub fn filter_path_inplace(
        &self,
        dst: &mut Path,
        src: &Path,
        stroke_rec: &mut StrokeRec,
        cull_rect: impl AsRef<Rect>,
    ) -> bool {
        unsafe {
            self.native().filterPath(
                dst.native_mut(),
                src.native(),
                stroke_rec.native_mut(),
                cull_rect.as_ref().native(),
            )
        }
    }

    pub fn filter_path_inplace_with_matrix(
        &self,
        dst: &mut Path,
        src: &Path,
        stroke_rec: &mut StrokeRec,
        cull_rect: impl AsRef<Rect>,
        ctm: &Matrix,
    ) -> bool {
        unsafe {
            self.native().filterPath1(
                dst.native_mut(),
                src.native(),
                stroke_rec.native_mut(),
                cull_rect.as_ref().native(),
                ctm.native(),
            )
        }
    }

    pub fn needs_ctm(&self) -> bool {
        unsafe { self.native().needsCTM() }
    }
}
