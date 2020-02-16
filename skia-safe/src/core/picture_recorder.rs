use crate::prelude::*;
use crate::{BBHFactory, Canvas, Drawable, Picture, Rect};
use skia_bindings as sb;
use skia_bindings::{SkPictureRecorder, SkRect};
use std::ptr;

bitflags! {
    pub struct RecordFlags: u32 {
        const PLAYBACK_DRAW_PICTURE = sb::SkPictureRecorder_RecordFlags_kPlaybackDrawPicture_RecordFlag as _;
    }
}

pub type PictureRecorder = Handle<SkPictureRecorder>;
unsafe impl Sync for PictureRecorder {}
unsafe impl Send for PictureRecorder {}

impl NativeDrop for SkPictureRecorder {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkPictureRecorder_destruct(self);
        }
    }
}

// TODO: why is the word "recording" used in all the functions, should we
// remove it?

impl Handle<SkPictureRecorder> {
    pub fn new() -> Self {
        Self::from_native(unsafe { SkPictureRecorder::new() })
    }

    pub fn begin_recording(
        &mut self,
        bounds: impl AsRef<Rect>,
        mut bbh_factory: Option<&mut BBHFactory>,
        record_flags: impl Into<Option<RecordFlags>>,
    ) -> &mut Canvas {
        let canvas_ref = unsafe {
            &mut *self.native_mut().beginRecording(
                bounds.as_ref().native(),
                bbh_factory.native_ptr_or_null_mut(),
                record_flags
                    .into()
                    .unwrap_or_else(RecordFlags::empty)
                    .bits(),
            )
        };

        Canvas::borrow_from_native(canvas_ref)
    }

    pub fn recording_canvas(&mut self) -> &mut Canvas {
        let canvas_ref = unsafe { &mut *self.native_mut().getRecordingCanvas() };

        Canvas::borrow_from_native(canvas_ref)
    }

    pub fn finish_recording_as_picture(&mut self, cull_rect: Option<&Rect>) -> Option<Picture> {
        let cull_rect_ptr: *const SkRect =
            cull_rect.map(|r| r.native() as _).unwrap_or(ptr::null());

        let picture_ptr = unsafe {
            sb::C_SkPictureRecorder_finishRecordingAsPicture(self.native_mut(), cull_rect_ptr)
        };

        Picture::from_ptr(picture_ptr)
    }

    pub fn finish_recording_as_drawable(&mut self) -> Option<Drawable> {
        Drawable::from_ptr(unsafe {
            sb::C_SkPictureRecorder_finishRecordingAsDrawable(self.native_mut())
        })
    }
}
