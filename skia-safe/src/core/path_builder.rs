use std::{fmt, mem};

use crate::{
    matrix, path, prelude::*, scalar, Matrix, Path, PathDirection, PathFillType, Point, RRect,
    Rect, Vector,
};
use skia_bindings::{self as sb, SkPathBuilder, SkPath_AddPathMode};

pub use skia_bindings::SkPathBuilder_ArcSize as ArcSize;
variant_name!(ArcSize::Large);

pub type PathBuilder = Handle<SkPathBuilder>;
unsafe_send_sync!(PathBuilder);

impl NativeDrop for SkPathBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathBuilder_destruct(self) }
    }
}

impl Clone for PathBuilder {
    fn clone(&self) -> Self {
        Self::construct(|pb| unsafe { sb::C_SkPathBuilder_CopyConstruct(pb, self.native()) })
    }
}

impl fmt::Debug for PathBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathBuilder")
            .field("fill_type", &self.fill_type())
            .finish()
    }
}

impl PathBuilder {
    pub fn new() -> Self {
        Self::construct(|pb| unsafe { sb::C_SkPathBuilder_Construct(pb) })
    }

    /* m87: No Implementation.
    pub fn new_fill_type(fill_type: PathFillType) -> Self {
        Self::construct(|pb| unsafe { sb::C_SkPathBuilder_Construct2(pb, fill_type) })
    }
    */

    pub fn new_path(path: &Path) -> Self {
        Self::construct(|pb| unsafe { sb::C_SkPathBuilder_Construct3(pb, path.native()) })
    }

    pub fn fill_type(&self) -> PathFillType {
        self.native().fFillType
    }

    pub fn compute_bounds(&self) -> Rect {
        Rect::construct(|r| unsafe { sb::C_SkPathBuilder_computeBounds(self.native(), r) })
    }

    pub fn snapshot(&self) -> Path {
        let mut path = Path::default();
        unsafe { sb::C_SkPathBuilder_snapshot(self.native(), path.native_mut()) }
        path
    }

    pub fn detach(&mut self) -> Path {
        let mut path = Path::default();
        unsafe { sb::C_SkPathBuilder_detach(self.native_mut(), path.native_mut()) }
        path
    }

    pub fn set_fill_type(&mut self, ft: PathFillType) -> &mut Self {
        self.native_mut().fFillType = ft;
        self
    }

    pub fn set_is_volatile(&mut self, is_volatile: bool) -> &mut Self {
        self.native_mut().fIsVolatile = is_volatile;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().reset();
        }
        self
    }

    pub fn move_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut().moveTo(pt.into().into_native());
        }
        self
    }

    pub fn line_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut().lineTo(pt.into().into_native());
        }
        self
    }

    pub fn quad_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut()
                .quadTo(p1.into().into_native(), p2.into().into_native());
        }
        self
    }

    pub fn conic_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>, w: scalar) -> &mut Self {
        unsafe {
            self.native_mut()
                .conicTo(p1.into().into_native(), p2.into().into_native(), w);
        }
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
        unsafe {
            self.native_mut().close();
        }
        self
    }

    pub fn polyline_to(&mut self, points: &[Point]) -> &mut Self {
        unsafe {
            sb::C_SkPathBuilder_polylineTo(
                self.native_mut(),
                points.native().as_ptr(),
                points.len(),
            );
        }
        self
    }

    pub fn r_line_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut().rLineTo(pt.into().into_native());
        }
        self
    }

    pub fn r_quad_to(&mut self, pt1: impl Into<Point>, pt2: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut()
                .rQuadTo(pt1.into().into_native(), pt2.into().into_native());
        }
        self
    }

    pub fn r_conic_to(
        &mut self,
        pt1: impl Into<Point>,
        pt2: impl Into<Point>,
        w: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .rConicTo(pt1.into().into_native(), pt2.into().into_native(), w);
        }
        self
    }

    pub fn r_cubic_to(
        &mut self,
        pt1: impl Into<Point>,
        pt2: impl Into<Point>,
        pt3: impl Into<Point>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().rCubicTo(
                pt1.into().into_native(),
                pt2.into().into_native(),
                pt3.into().into_native(),
            );
        }
        self
    }

    pub fn r_arc_to(
        &mut self,
        r: (scalar, scalar),
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: PathDirection,
        d: impl Into<Vector>,
    ) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut()
                .rArcTo(r.0, r.1, x_axis_rotate, large_arc, sweep, d.x, d.y);
        }
        self
    }

    pub fn arc_to(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle_deg: scalar,
        sweep_angle_deg: scalar,
        force_move_to: bool,
    ) -> &mut Self {
        unsafe {
            self.native_mut().arcTo(
                oval.as_ref().native(),
                start_angle_deg,
                sweep_angle_deg,
                force_move_to,
            );
        }
        self
    }

    pub fn arc_to_tangent(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        radius: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .arcTo1(p1.into().into_native(), p2.into().into_native(), radius);
        }
        self
    }

    pub fn arc_to_radius(
        &mut self,
        r: impl Into<Point>,
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: PathDirection,
        xy: impl Into<Point>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().arcTo2(
                r.into().into_native(),
                x_axis_rotate,
                large_arc,
                sweep,
                xy.into().into_native(),
            );
        }
        self
    }

    pub fn add_arc(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle_deg: scalar,
        sweep_angle_deg: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .addArc(oval.as_ref().native(), start_angle_deg, sweep_angle_deg);
        }
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
                .addRect(rect.as_ref().native(), dir, start_index.try_into().unwrap());
        }
        self
    }

    pub fn add_oval(
        &mut self,
        rect: impl AsRef<Rect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        // m86: default start index changed from 0 to 1
        let start_index = start_index.into().unwrap_or(1);
        unsafe {
            self.native_mut()
                .addOval(rect.as_ref().native(), dir, start_index.try_into().unwrap());
        }
        self
    }

    pub fn add_rrect(
        &mut self,
        rect: impl AsRef<RRect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        // m86: default start index changed from 0 to 6 or 7 depending on the path's direction.
        let start_index =
            start_index
                .into()
                .unwrap_or(if dir == PathDirection::CW { 6 } else { 7 });
        unsafe {
            self.native_mut().addRRect(
                rect.as_ref().native(),
                dir,
                start_index.try_into().unwrap(),
            );
        }
        self
    }

    pub fn add_circle(
        &mut self,
        center: impl Into<Point>,
        radius: scalar,
        dir: impl Into<Option<PathDirection>>,
    ) -> &mut Self {
        let center = center.into();
        let dir = dir.into().unwrap_or(PathDirection::CW);
        unsafe {
            self.native_mut().addCircle(center.x, center.y, radius, dir);
        }
        self
    }

    pub fn add_polygon(&mut self, pts: &[Point], close: bool) -> &mut Self {
        unsafe {
            sb::C_SkPathBuilder_addPolygon(
                self.native_mut(),
                pts.native().as_ptr(),
                pts.len(),
                close,
            );
        }
        self
    }

    pub fn add_path(&mut self, path: &Path) -> &mut Self {
        unsafe {
            self.native_mut()
                .addPath(path.native(), 0., 0., SkPath_AddPathMode::Append)
        };
        self
    }

    pub fn add_path_with_transform(
        &mut self,
        src: &Path,
        matrix: &Matrix,
        mode: impl Into<Option<path::AddPathMode>>,
    ) {
        unsafe {
            self.native_mut().addPath1(
                src.native(),
                matrix.native(),
                mode.into().unwrap_or(path::AddPathMode::Append),
            )
        };
    }

    pub fn inc_reserve(&mut self, extra_pt_count: usize, extra_verb_count: usize) {
        unsafe {
            self.native_mut().incReserve(
                extra_pt_count.try_into().unwrap(),
                extra_verb_count.try_into().unwrap(),
            )
        }
    }

    pub fn offset(&mut self, d: impl Into<Vector>) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().offset(d.x, d.y);
        }
        self
    }

    pub fn transform(
        &mut self,
        matrix: &Matrix,
        pc: impl Into<Option<matrix::ApplyPerspectiveClip>>,
    ) -> &mut Self {
        let pc = pc.into().unwrap_or(matrix::ApplyPerspectiveClip::Yes);
        unsafe {
            self.native_mut().transform(matrix.native(), pc);
        }
        self
    }

    pub fn toggle_inverse_fill_type(&mut self) -> &mut Self {
        let n = self.native_mut();
        n.fFillType = unsafe { mem::transmute::<i32, sb::SkPathFillType>(n.fFillType as i32 ^ 2) };
        self
    }

    pub fn is_empty(&self) -> bool {
        unsafe { sb::C_SkPathBuilder_isEmpty(self.native()) }
    }

    pub fn get_last_pt(&self) -> Option<Point> {
        let mut p = Point::default();
        unsafe { sb::C_SkPathBuilder_getLastPt(self.native(), p.native_mut()) }
            .if_true_then_some(|| p)
    }

    pub fn set_last_pt(&mut self, p: impl Into<Point>) {
        let p = p.into();
        unsafe { self.native_mut().setLastPt(p.x, p.y) };
    }

    pub fn count_points(&self) -> usize {
        unsafe { sb::C_SkPathBuilder_countPoints(self.native()) }
    }

    pub fn is_inverse_fill_type(&self) -> bool {
        PathFillType::is_inverse(self.fill_type())
    }

    pub fn points(&self) -> &[Point] {
        unsafe {
            let mut len = 0;
            let points = sb::C_SkPathBuilder_points(self.native(), &mut len);
            safer::from_raw_parts(Point::from_native_ptr(points), len)
        }
    }

    pub fn verbs(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let verbs = sb::C_SkPathBuilder_verbs(self.native(), &mut len);
            safer::from_raw_parts(verbs, len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_snapshot_and_detach() {
        let mut builder = PathBuilder::new();
        let _path = builder.snapshot();
        let _path = builder.detach();
    }
}
