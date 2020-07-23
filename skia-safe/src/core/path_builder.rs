use crate::{prelude::*, scalar, Path, PathDirection, PathFillType, Point, RRect, Rect};
use skia_bindings as sb;
use skia_bindings::SkPathBuilder;

pub type PathBuilder = Handle<SkPathBuilder>;

impl NativeDrop for SkPathBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathBuilder_destruct(self) }
    }
}

impl PathBuilder {
    pub fn new() -> Self {
        Self::from_native(unsafe { SkPathBuilder::new() })
    }

    pub fn snapshot(&mut self) -> Path {
        let mut path = Path::default();
        unsafe { sb::C_SkPathBuilder_snapshot(self.native_mut(), path.native_mut()) };
        path
    }

    pub fn detach(&mut self) -> Path {
        let mut path = Path::default();
        unsafe { sb::C_SkPathBuilder_detach(self.native_mut(), path.native_mut()) };
        path
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() };
        self
    }

    pub fn move_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe { self.native_mut().moveTo(pt.into().into_native()) };
        self
    }

    pub fn line_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe { self.native_mut().lineTo(pt.into().into_native()) };
        self
    }

    pub fn quad_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut()
                .quadTo(p1.into().into_native(), p2.into().into_native())
        };
        self
    }

    pub fn conic_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>, w: scalar) -> &mut Self {
        unsafe {
            self.native_mut()
                .conicTo(p1.into().into_native(), p2.into().into_native(), w)
        };
        self
    }

    pub fn cubic_to(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        p3: impl Into<Point>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().cubicTo(
                p1.into().into_native(),
                p2.into().into_native(),
                p3.into().into_native(),
            )
        };
        self
    }

    pub fn close(&mut self) -> &mut Self {
        unsafe { self.native_mut().close() };
        self
    }

    pub fn add_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        let start_index = start_index.into().unwrap_or(0);
        unsafe {
            self.native_mut()
                .addRect(rect.as_ref().native(), dir, start_index.try_into().unwrap())
        };
        self
    }

    pub fn add_oval(
        &mut self,
        rect: impl AsRef<Rect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        let start_index = start_index.into().unwrap_or(0);
        unsafe {
            self.native_mut()
                .addOval(rect.as_ref().native(), dir, start_index.try_into().unwrap())
        };
        self
    }

    pub fn add_rrect(
        &mut self,
        rect: impl AsRef<RRect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        let start_index = start_index.into().unwrap_or(0);
        unsafe {
            self.native_mut()
                .addRRect(rect.as_ref().native(), dir, start_index.try_into().unwrap())
        };
        self
    }

    pub fn inc_reserve(&mut self, extra_pt_count: usize, extra_verb_count: usize) {
        unsafe {
            self.native_mut().incReserve(
                extra_pt_count.try_into().unwrap(),
                extra_verb_count.try_into().unwrap(),
            )
        }
    }

    pub fn make(
        points: &[Point],
        verbs: &[u8],
        conic_weights: &[scalar],
        fill_type: PathFillType,
        is_volatile: impl Into<Option<bool>>,
    ) -> Path {
        let mut path = Path::default();
        unsafe {
            sb::C_SkPathBuilder_Make(
                points.native().as_ptr(),
                points.len().try_into().unwrap(),
                verbs.as_ptr(),
                verbs.len().try_into().unwrap(),
                conic_weights.as_ptr(),
                conic_weights.len().try_into().unwrap(),
                fill_type,
                is_volatile.into().unwrap_or(false),
                path.native_mut(),
            )
        }
        path
    }
}

#[test]
fn test_creation_snapshot_and_detach() {
    let mut builder = PathBuilder::new();
    let _path = builder.snapshot();
    let _path = builder.detach();
}
