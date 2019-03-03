use std::{slice, mem};
use crate::prelude::*;
use crate::skia::{
    Rect,
    StrokeRec,
    Path,
    Matrix,
    scalar
};
use rust_skia::{
    SkPathEffect_PointData,
    SkVector,
    SkPoint,
    SkPathEffect_PointData_PointFlags,
    SkRect,
    C_SkPathEffect_MakeCompose,
    C_SkPathEffect_MakeSum,
    SkRefCntBase,
    SkPathEffect,
    C_SkPathEffect_PointData_deletePoints,
    SkPathEffect_DashInfo,
    SkPathEffect_DashType
};

bitflags! {
    pub struct PointDataPointFlags: u32 {
        const Circles = SkPathEffect_PointData_PointFlags::kCircles_PointFlag as _;
        const UsePath = SkPathEffect_PointData_PointFlags::kUsePath_PointFlag as _;
        const UseClip = SkPathEffect_PointData_PointFlags::kUseClip_PointFlag as _;
    }
}

/// TODO: review as soon we support transmutable Point, Vector, and Rect.
#[repr(C)]
pub struct PathEffectPointData {
    pub flags: PointDataPointFlags,
    points: *const SkPoint,
    num_points: i32,
    pub size: SkVector,
    pub clip_rect: SkRect,
    pub path: Path,
    pub first: Path,
    pub last: Path
}

impl NativeTransmutable<SkPathEffect_PointData> for PathEffectPointData {}

impl Drop for PathEffectPointData {
    fn drop(&mut self) {
        unsafe {
            // we can't call destruct, because it would destruct
            // other fields like Path, which would also be dropped individually,
            // so we just delete the points array here.
            C_SkPathEffect_PointData_deletePoints(self.native_mut())
        }
    }
}

impl PathEffectPointData {
    pub fn new() -> Self {
        PathEffectPointData::from_native(unsafe {
            SkPathEffect_PointData::new()
        })
    }

    /// TODO: review as soon we support transmutable Points.
    pub fn points(&self) -> &[SkPoint] {
        unsafe {
            slice::from_raw_parts(self.points, self.num_points.try_into().unwrap())
        }
    }
}

#[test]
fn point_data_layout() {
    PathEffectPointData::test_layout();
}

/*
pub type DashType = EnumHandle<SkPathEffect_DashType>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPathEffect_DashType> {
    pub const None: Self = Self(SkPathEffect_DashType::kNone_DashType);
    pub const Dash: Self = Self(SkPathEffect_DashType::kDash_DashType);
}
*/

#[derive(Clone, PartialEq, Debug)]
pub struct PathEffectDashInfo {
    intervals: Vec<scalar>,
    phase: scalar
}

type PathEffect = RCHandle<SkPathEffect>;

impl NativeRefCountedBase for SkPathEffect {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl RCHandle<SkPathEffect> {
    pub fn sum(first: &PathEffect, second: &PathEffect) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            C_SkPathEffect_MakeSum(first.shared_native(), second.shared_native())
        }).unwrap()
    }

    pub fn compose(first: &PathEffect, second: &PathEffect) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            C_SkPathEffect_MakeCompose(first.shared_native(), second.shared_native())
        }).unwrap()
    }

    pub fn filter_path_inplace(
        &self, dst: &mut Path, src: &Path,
        stroke_rec: &mut StrokeRec, cull_rect: &Rect) -> bool {
        unsafe {
            self.native().filterPath(
                dst.native_mut(), src.native(),
                stroke_rec.native_mut(),
                &cull_rect.into_native())
        }
    }

    // for convenience
    pub fn filter_path(&self, src: &Path, stroke_rec: &StrokeRec, cull_rect: &Rect)
        -> Option<(Path, StrokeRec)> {
        let mut dst = Path::new();
        let mut stroke_rec_r = stroke_rec.clone();
        self.filter_path_inplace(&mut dst, src, &mut stroke_rec_r, &cull_rect)
            .if_true_some((dst, stroke_rec_r))
    }

    pub fn compute_fast_bounds(&self, src: &Rect) -> Rect {
        // TODO: use Rect::empty() as soon it's available.
        let mut r : SkRect = unsafe { mem::zeroed() };
        unsafe { self.native().computeFastBounds(&mut r, &src.into_native()) };
        Rect::from_native(r)
    }

    pub fn as_points(
        &self,
        src: &Path,
        stroke_rect: &StrokeRec,
        matrix: &Matrix,
        cull_rect: &Rect)
        -> Option<PathEffectPointData> {
        let mut point_data = PathEffectPointData::new();
        unsafe {
            self.native().asPoints(
                point_data.native_mut(),
                src.native(),
                stroke_rect.native(),
                matrix.native(),
                &cull_rect.into_native())
        }.if_true_some(point_data)
    }

    pub fn as_dash(&self) -> Option<PathEffectDashInfo> {
        let mut dash_info = unsafe { SkPathEffect_DashInfo::new() };

        let dash_type = unsafe {
            self.native().asADash(&mut dash_info)
        };

        match dash_type {
            SkPathEffect_DashType::kDash_DashType => {
                let mut v: Vec<scalar> = vec![0.0; dash_info.fCount.try_into().unwrap()];
                dash_info.fIntervals = v.as_mut_ptr();
                unsafe {
                    assert_eq!(dash_type, self.native().asADash(&mut dash_info));
                }
                Some(PathEffectDashInfo { intervals: v, phase: dash_info.fPhase })
            },
            SkPathEffect_DashType::kNone_DashType => {
                None
            }
        }
    }
}