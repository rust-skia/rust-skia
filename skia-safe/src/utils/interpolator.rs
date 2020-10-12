//! This wrapper combines SkInterpolatorBase and SkInterpolator into the type Interpolator.

use crate::prelude::*;
use crate::{scalar, Point};
use skia_bindings as sb;
use skia_bindings::{SkInterpolator, SkUnitCubicInterp};
use std::time::Duration;

pub use skia_bindings::SkInterpolatorBase_Result as Result;
#[test]
fn test_interpolator_result_naming() {
    let _ = Result::FreezeEnd_Result;
}

pub type Interpolator = Handle<SkInterpolator>;
unsafe impl Send for Interpolator {}
unsafe impl Sync for Interpolator {}

impl NativeDrop for SkInterpolator {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkInterpolator_destruct(self);
        }
    }
}

impl Default for Handle<SkInterpolator> {
    fn default() -> Self {
        Handle::from_native(unsafe { SkInterpolator::new() })
    }
}

/// Wrapper for functions that are implemented in SkInterpolatorBase
impl Handle<SkInterpolator> {
    pub fn duration(&self) -> Option<(Duration, Duration)> {
        let mut start_time = 0;
        let mut end_time = 0;
        unsafe {
            self.native()
                ._base
                .getDuration(&mut start_time, &mut end_time)
        }
        .if_true_then_some(|| {
            (
                Duration::from_millis(start_time.try_into().unwrap()),
                Duration::from_millis(end_time.try_into().unwrap()),
            )
        })
    }

    pub fn set_mirror(&mut self, mirror: bool) -> &mut Self {
        unsafe { sb::C_SkInterpolator_setMirror(self.native_mut(), mirror) }
        self
    }

    pub fn set_repeat_count(&mut self, repeat_count: scalar) -> &mut Self {
        unsafe { sb::C_SkInterpolator_setRepeatCount(self.native_mut(), repeat_count) }
        self
    }

    pub fn set_reset(&mut self, reset: bool) -> &mut Self {
        unsafe { sb::C_SkInterpolator_setReset(self.native_mut(), reset) }
        self
    }

    pub fn time_to_t(&self, time: Duration) -> (Result, TimeToT) {
        let mut t = 0.0;
        let mut index = 0;
        let mut exact = false;
        let r = unsafe {
            self.native()._base.timeToT(
                time.as_millis().try_into().unwrap(),
                &mut t,
                &mut index,
                &mut exact,
            )
        };
        (
            r,
            TimeToT {
                t,
                index: index.try_into().unwrap(),
                exact,
            },
        )
    }
}

/// Wrapper for SkInterpolator functions.
impl Handle<SkInterpolator> {
    pub fn new(elem_count: usize, frame_count: usize) -> Self {
        Handle::from_native(unsafe {
            SkInterpolator::new1(
                elem_count.try_into().unwrap(),
                frame_count.try_into().unwrap(),
            )
        })
    }

    pub fn reset(&mut self, elem_count: usize, frame_count: usize) -> &mut Self {
        unsafe {
            self.native_mut().reset(
                elem_count.try_into().unwrap(),
                frame_count.try_into().unwrap(),
            )
        }
        self
    }

    pub fn set_key_frame<'a>(
        &mut self,
        index: usize,
        time: Duration,
        values: &[scalar],
        blend: impl Into<Option<&'a [scalar; 4]>>,
    ) -> bool {
        assert_eq!(values.len(), self.elem_count());
        unsafe {
            self.native_mut().setKeyFrame(
                index.try_into().unwrap(),
                time.as_millis().try_into().unwrap(),
                values.as_ptr(),
                blend.into().as_ptr_or_null() as _,
            )
        }
    }

    // TODO: may provide a variant that returns a Vec.
    pub fn time_to_values<'a>(
        &self,
        time: Duration,
        values: impl Into<Option<&'a mut [scalar]>>,
    ) -> Result {
        let mut values = values.into();
        if let Some(ref values) = values {
            assert_eq!(values.len(), self.elem_count());
        };
        unsafe {
            self.native().timeToValues(
                time.as_millis().try_into().unwrap(),
                values.as_ptr_or_null_mut(),
            )
        }
    }
}

/// Additional functions that seem useful.
impl Handle<SkInterpolator> {
    pub fn elem_count(&self) -> usize {
        self.native()._base.fElemCount.try_into().unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TimeToT {
    pub t: scalar,
    pub index: usize,
    pub exact: bool,
}

pub fn unit_cubic_interp(value: scalar, b: impl Into<Point>, c: impl Into<Point>) -> scalar {
    let b = b.into();
    let c = c.into();
    unsafe { SkUnitCubicInterp(value, b.x, b.y, c.x, c.y) }
}
