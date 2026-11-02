//! Records drawing commands into a [`crate::Picture`] via a [`crate::Canvas`].

use crate::{Canvas, Drawable, Picture, Rect, prelude::*};
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

    /// Returns the canvas that records the drawing commands.
    ///
    /// - `bounds` the cull rect used when recording this picture. Any drawing that falls outside
    ///   of this rect is undefined, and may be drawn or it may not
    /// - `use_bbh` whether to use a bounding box hierarchy
    pub fn begin_recording(&mut self, bounds: impl AsRef<Rect>, use_bbh: bool) -> &Canvas {
        let canvas = unsafe {
            sb::C_SkPictureRecorder_beginRecording(
                self.native_mut(),
                bounds.as_ref().native(),
                use_bbh,
            )
        };

        unsafe { Canvas::borrow_from_native(canvas) }
    }

    /// Returns the recording canvas if one is active, or `None` if recording is not active. This
    /// does not alter the ref count on the canvas (if present).
    pub fn recording_canvas(&mut self) -> Option<&Canvas> {
        let canvas = unsafe { self.native_mut().getRecordingCanvas() };
        if canvas.is_null() {
            return None;
        }
        Some(unsafe { Canvas::borrow_from_native(canvas) })
    }

    /// Signals that the caller is done recording. This invalidates the canvas returned by
    /// [`Self::begin_recording()`] or [`Self::recording_canvas()`].
    ///
    /// The returned picture is immutable. If during recording drawables were added to the canvas,
    /// these will have been "drawn" into a recording canvas, so that this resulting picture will
    /// reflect their current state, but will not contain a live reference to the drawables
    /// themselves.
    ///
    /// - `cull_rect` optional cull rectangle
    pub fn finish_recording_as_picture(&mut self, cull_rect: Option<&Rect>) -> Option<Picture> {
        self.recording_canvas()?;
        let cull_rect_ptr: *const SkRect =
            cull_rect.map(|r| r.native() as _).unwrap_or(ptr::null());

        let picture_ptr = unsafe {
            sb::C_SkPictureRecorder_finishRecordingAsPicture(self.native_mut(), cull_rect_ptr)
        };

        Picture::from_ptr(picture_ptr)
    }

    /// Signals that the caller is done recording. This invalidates the canvas returned by
    /// [`Self::begin_recording()`] or [`Self::recording_canvas()`].
    ///
    /// Unlike [`Self::finish_recording_as_picture()`], which returns an immutable picture, the
    /// returned drawable may contain live references to other drawables (if they were added to the
    /// recording canvas) and therefore this drawable will reflect the current state of those nested
    /// drawables anytime it is drawn or a new picture is snapped from it.
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
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), false);
    canvas.clear(crate::Color::WHITE);
    let _picture = recorder.finish_recording_as_picture(None).unwrap();
}

#[test]
fn begin_recording_two_times() {
    let mut recorder = PictureRecorder::new();
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), false);
    canvas.clear(crate::Color::WHITE);
    assert!(recorder.recording_canvas().is_some());
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), false);
    canvas.clear(crate::Color::WHITE);
    assert!(recorder.recording_canvas().is_some());
}

#[test]
fn finishing_recording_two_times() {
    let mut recorder = PictureRecorder::new();
    let canvas = recorder.begin_recording(Rect::new(0.0, 0.0, 100.0, 100.0), false);
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

#[test]
fn record_with_bbox_hierarchy() {
    let mut paint = crate::Paint::new(crate::Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(crate::PaintStyle::Fill);

    let frame_rect = Rect::new(0.0, 0.0, 100.0, 100.0);
    let crop_rect = Rect::new(50.0, 50.0, 100.0, 100.0);
    let drawn_rect = Rect::new(70.0, 70.0, 80.0, 80.0);

    // with bbh disabled, cull rects reflect the arg passed to begin_recording
    let mut src_rec = PictureRecorder::new();
    src_rec
        .begin_recording(frame_rect, false)
        .draw_rect(drawn_rect, &paint);
    let picture = src_rec.finish_recording_as_picture(None).unwrap();
    assert!(picture.cull_rect() == frame_rect);

    let mut no_bbh = PictureRecorder::new();
    no_bbh
        .begin_recording(crop_rect, false)
        .draw_picture(&picture, None, None);
    let no_bbh_pict = no_bbh.finish_recording_as_picture(None).unwrap();
    assert!(no_bbh_pict.cull_rect() == crop_rect);

    // with bbh enabled, cull rect contracts to just the content drawn
    let mut with_bbh = PictureRecorder::new();
    with_bbh
        .begin_recording(frame_rect, true)
        .draw_picture(&picture, None, None);
    let bbh_pict = with_bbh.finish_recording_as_picture(None).unwrap();
    assert!(bbh_pict.cull_rect() == drawn_rect);
}
