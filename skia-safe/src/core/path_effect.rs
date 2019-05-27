use std::{slice, mem};
use std::os::raw;
use crate::prelude::*;
use crate::core::{
    Rect,
    StrokeRec,
    Path,
    Matrix,
    scalar,
    Vector,
    Point
};
use skia_bindings::{
    SkPathEffect_PointData,
    C_SkPathEffect_MakeCompose,
    C_SkPathEffect_MakeSum,
    SkRefCntBase,
    SkPathEffect,
    C_SkPathEffect_PointData_Construct,
    C_SkPathEffect_PointData_deletePoints,
    SkPathEffect_DashInfo,
    SkPathEffect_DashType,
};

#[repr(C)]
pub struct PointData {
    pub flags: point_data::PointFlags,
    points: *const Point,
    num_points: raw::c_int,
    pub size: Vector,
    pub clip_rect: Rect,
    pub path: Path,
    pub first: Path,
    pub last: Path,
}

impl NativeTransmutable<SkPathEffect_PointData> for PointData {}

#[test]
fn test_point_data_layout() {
    Point::test_layout();
    Vector::test_layout();
    Rect::test_layout();
    PointData::test_layout();
}

impl Drop for PointData {
    fn drop(&mut self) {
        unsafe {
            // we can't call destruct, because it would destruct
            // other fields like Path, which would also be dropped individually,
            // so we just delete the points array here.
            C_SkPathEffect_PointData_deletePoints(self.native_mut())
        }
    }
}

impl Default for PointData {
    fn default() -> Self {
        PointData::from_native(unsafe {
            // does not link under Linux:
            // SkPathEffect_PointData::new()
            let mut point_data = mem::uninitialized();
            C_SkPathEffect_PointData_Construct(&mut point_data);
            point_data
        })
    }
}

impl PointData {
    pub fn points(&self) -> &[Point] {
        unsafe {
            slice::from_raw_parts(self.points, self.num_points.try_into().unwrap())
        }
    }
}

pub mod point_data {
    bitflags! {
        pub struct PointFlags: u32 {
            const CIRCLES = skia_bindings::SkPathEffect_PointData_PointFlags_kCircles_PointFlag as _;
            const USE_PATH = skia_bindings::SkPathEffect_PointData_PointFlags_kUsePath_PointFlag as _;
            const USE_CLIP = skia_bindings::SkPathEffect_PointData_PointFlags_kUseClip_PointFlag as _;
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DashInfo {
    pub intervals: Vec<scalar>,
    pub phase: scalar
}

pub type PathEffect = RCHandle<SkPathEffect>;

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

    pub fn filter_path_inplace<R: AsRef<Rect>>(
        &self, dst: &mut Path, src: &Path,
        stroke_rec: &mut StrokeRec, cull_rect: R) -> bool {
        unsafe {
            self.native().filterPath(
                dst.native_mut(), src.native(),
                stroke_rec.native_mut(),
                cull_rect.as_ref().native())
        }
    }

    // for convenience
    pub fn filter_path<R: AsRef<Rect>>(&self, src: &Path, stroke_rec: &StrokeRec, cull_rect: R)
        -> Option<(Path, StrokeRec)> {
        let mut dst = Path::default();
        let mut stroke_rec_r = stroke_rec.clone();
        self.filter_path_inplace(&mut dst, src, &mut stroke_rec_r, cull_rect)
            .if_true_some((dst, stroke_rec_r))
    }

    pub fn compute_fast_bounds<R: AsRef<Rect>>(&self, src: R) -> Rect {
        let mut r : Rect = Rect::default();
        unsafe { self.native().computeFastBounds(r.native_mut(), src.as_ref().native()) };
        r
    }

    pub fn as_points<CR: AsRef<Rect>>(
        &self,
        src: &Path,
        stroke_rect: &StrokeRec,
        matrix: &Matrix,
        cull_rect: CR)
        -> Option<PointData> {
        let mut point_data = PointData::default();
        unsafe {
            self.native().asPoints(
                point_data.native_mut(),
                src.native(),
                stroke_rect.native(),
                matrix.native(),
                cull_rect.as_ref().native())
        }.if_true_some(point_data)
    }

    pub fn as_dash(&self) -> Option<DashInfo> {
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
                Some(DashInfo { intervals: v, phase: dash_info.fPhase })
            },
            SkPathEffect_DashType::kNone_DashType => {
                None
            }
        }
    }
}

#[test]
fn create_and_drop_point_data() {
    let data = PointData::default();
    drop(data)
}