use crate::{prelude::*, BBHFactory, Canvas, Drawable, Picture, Rect};
use skia_bindings::{self as sb, SkPictureRecorder, SkRect};
use std::{fmt, ptr};

pub type PictureRecorder = Handle<SkPictureRecorder>;

impl NativeDrop for SkPictureRecorder {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkPictureRecorder_destruct(self);
        }
    }
}

impl fmt::Debug for PictureRecorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PictureRecorder").finish()
    }
}

impl PictureRecorder {
    pub fn new() -> Self {
        Self::construct(|pr| unsafe { sb::C_SkPictureRecorder_Construct(pr) })
    }

    // TODO: beginRecording with BBoxHierarchy

    pub fn begin_recording(
        &mut self,
        bounds: impl AsRef<Rect>,
        mut bbh_factory: Option<&mut BBHFactory>,
    ) -> &Canvas {
        let canvas_ref = unsafe {
            &*self.native_mut().beginRecording1(
                bounds.as_ref().native(),
                bbh_factory.native_ptr_or_null_mut(),
            )
        };

        Canvas::borrow_from_native(canvas_ref)
    }

    pub fn recording_canvas(&mut self) -> Option<&Canvas> {
        let canvas = unsafe { self.native_mut().getRecordingCanvas() };
        if canvas.is_null() {
            return None;
        }
        Some(Canvas::borrow_from_native(unsafe { &*canvas }))
    }

    pub fn finish_recording_as_picture(&mut self, cull_rect: Option<&Rect>) -> Option<Picture> {
        self.recording_canvas()?;
        let cull_rect_ptr: *const SkRect =
            cull_rect.map(|r| r.native() as _).unwrap_or(ptr::null());

        let picture_ptr = unsafe {
            sb::C_SkPictureRecorder_finishRecordingAsPicture(self.native_mut(), cull_rect_ptr)
        };

        Picture::from_ptr(picture_ptr)
    }

    pub fn finish_recording_as_drawable(&mut self) -> Option<Drawable> {
        self.recording_canvas()?;
        Drawable::from_ptr(unsafe {
            sb::C_SkPictureRecorder_finishRecordingAsDrawable(self.native_mut())
        })
    }
}

#[test]
fn good_case() {
    let mut recorder = PictureRecorder::new();
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), None);
    canvas.clear(crate::Color::WHITE);
    let _picture = recorder.finish_recording_as_picture(None).unwrap();
}

#[test]
fn begin_recording_two_times() {
    let mut recorder = PictureRecorder::new();
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), None);
    canvas.clear(crate::Color::WHITE);
    assert!(recorder.recording_canvas().is_some());
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), None);
    canvas.clear(crate::Color::WHITE);
    assert!(recorder.recording_canvas().is_some());
}

#[test]
fn finishing_recording_two_times() {
    let mut recorder = PictureRecorder::new();
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), None);
    canvas.clear(crate::Color::WHITE);
    assert!(recorder.finish_recording_as_picture(None).is_some());
    assert!(recorder.recording_canvas().is_none());
    assert!(recorder.finish_recording_as_picture(None).is_none());
}

#[test]
fn not_recording_no_canvas() {
    let mut recorder = PictureRecorder::new();
    assert!(recorder.recording_canvas().is_none());
}
