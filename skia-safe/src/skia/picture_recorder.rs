use std::ptr;
use crate::prelude::*;
use crate::skia::{
    Rect,
    BBHFactory,
    Canvas,
    Picture
};
use skia_bindings::{
    C_SkPictureRecorder_finishRecordingAsPicture,
    C_SkPictureRecorder_destruct,
    SkPictureRecorder_RecordFlags,
    SkPictureRecorder,
    SkRect
};

bitflags! {
    pub struct PictureRecorderRecordFlags: u32 {
        const PlaybackDrawPicture = SkPictureRecorder_RecordFlags::kPlaybackDrawPicture_RecordFlag as _;
    }
}

pub type PictureRecorder = Handle<SkPictureRecorder>;

impl NativeDrop for SkPictureRecorder {
    fn drop(&mut self) {
        unsafe { C_SkPictureRecorder_destruct(self); }
    }
}

impl Handle<SkPictureRecorder> {

    pub fn new() -> Self {
        unsafe { SkPictureRecorder::new() }.into_handle()
    }

    pub fn begin_recording(
        &mut self,
        bounds: &Rect,
        mut bbh_factory: Option<&mut BBHFactory>,
        record_flags: PictureRecorderRecordFlags) -> &mut Canvas {

        let canvas_ref = unsafe {
            &mut *self.native_mut().beginRecording(
                &bounds.into_native(),
                bbh_factory.native_ptr_or_null_mut(),
                record_flags.bits())
        };

        Canvas::borrow_from_native(canvas_ref)
    }

    pub fn recording_canvas(&mut self) -> &mut Canvas {
        let canvas_ref = unsafe {
            &mut *self.native_mut().getRecordingCanvas()
        };

        Canvas::borrow_from_native(canvas_ref)
    }

    pub fn finish_recording_as_picture(&mut self, cull_rect: Option<&Rect>) -> Picture {

        let cull_rect_ptr : *const SkRect =
            cull_rect
                .map(|r| r.native() as _)
                .unwrap_or(ptr::null());

        let picture_ptr = unsafe {
            C_SkPictureRecorder_finishRecordingAsPicture(
                self.native_mut(), cull_rect_ptr)
        };

        Picture::from_ptr(picture_ptr).unwrap()
    }
}



