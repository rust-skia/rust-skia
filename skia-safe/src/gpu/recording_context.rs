use crate::prelude::*;
use skia_bindings::{GrContext, GrDirectContext, GrRecordingContext, SkRefCntBase};

pub type RecordingContext = RCHandle<GrRecordingContext>;

impl NativeRefCountedBase for GrRecordingContext {
    type Base = SkRefCntBase;
}

impl From<RCHandle<GrContext>> for RCHandle<GrRecordingContext> {
    fn from(direct_context: RCHandle<GrContext>) -> Self {
        unsafe { std::mem::transmute(direct_context) }
    }
}
impl From<RCHandle<GrDirectContext>> for RCHandle<GrRecordingContext> {
    fn from(direct_context: RCHandle<GrDirectContext>) -> Self {
        unsafe { std::mem::transmute(direct_context) }
    }
}
