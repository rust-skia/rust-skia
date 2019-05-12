use crate::prelude::*;
use crate::{scalar, Canvas, Matrix};
use skia_bindings::{
    C_Sk3DView_delete, C_Sk3DView_new, Sk3DView, SkCamera3D, SkMatrix3D, SkPatch3D, SkPoint3D,
    SkUnit3D,
};

#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[repr(C)]
pub struct Unit3D {
    pub x: scalar,
    pub y: scalar,
    pub z: scalar,
}

impl NativeTransmutable<SkUnit3D> for Unit3D {}

#[test]
fn test_unit_3d_layout() {
    Unit3D::test_layout();
}

impl Unit3D {
    pub fn set(&mut self, x: scalar, y: scalar, z: scalar) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn dot(a: &Self, b: &Self) -> scalar {
        unsafe { SkUnit3D::Dot(a.native(), b.native()) }
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        let mut r = Self::default();
        unsafe { SkUnit3D::Cross(a.native(), b.native(), r.native_mut()) };
        r
    }
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[repr(C)]
pub struct Point3D {
    pub x: scalar,
    pub y: scalar,
    pub z: scalar,
}

impl NativeTransmutable<SkPoint3D> for Point3D {}
#[test]
fn test_point_3d_layout() {
    Point3D::test_layout();
}

impl From<(scalar, scalar, scalar)> for Point3D {
    fn from((x, y, z): (scalar, scalar, scalar)) -> Self {
        Point3D { x, y, z }
    }
}

impl Point3D {
    pub fn set(&mut self, x: scalar, y: scalar, z: scalar) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn normalize(&self, unit: &mut Unit3D) -> scalar {
        unsafe { self.native().normalize(unit.native_mut()) }
    }
}

pub type Vector3D = Point3D;

// note: Default is an empty matrix, and not the identity matrix, which is generated with reset()!
#[derive(Clone, PartialEq, Default, Debug)]
#[repr(C)]
pub struct Matrix3D {
    pub mat: [[scalar; 4]; 3],
}

impl NativeTransmutable<SkMatrix3D> for Matrix3D {}
#[test]
fn test_matrix_3d_layout() {
    Matrix3D::test_layout();
}

impl Matrix3D {
    pub fn reset(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().reset();
        }
        self
    }

    pub fn set_row(
        &mut self,
        row: usize,
        a: scalar,
        b: scalar,
        c: scalar,
        d: impl Into<Option<scalar>>,
    ) -> &mut Self {
        assert!(row < self.mat.len());
        unsafe {
            self.native_mut().setRow(
                row.try_into().unwrap(),
                a,
                b,
                c,
                d.into().unwrap_or_default(),
            )
        }
        self
    }

    pub fn set_rotate_x(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().setRotateX(deg) }
        self
    }

    pub fn set_rotate_y(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().setRotateY(deg) }
        self
    }

    pub fn set_rotate_z(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().setRotateZ(deg) }
        self
    }

    pub fn set_translate(&mut self, x: scalar, y: scalar, z: scalar) -> &mut Self {
        unsafe { self.native_mut().setTranslate(x, y, z) }
        self
    }

    pub fn pre_rotate_x(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().preRotateX(deg) }
        self
    }

    pub fn pre_rotate_y(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().preRotateY(deg) }
        self
    }

    pub fn pre_rotate_z(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().preRotateZ(deg) }
        self
    }

    pub fn pre_translate(&mut self, x: scalar, y: scalar, z: scalar) -> &mut Self {
        unsafe { self.native_mut().preTranslate(x, y, z) }
        self
    }

    pub fn set_concat(&mut self, a: &Self, b: &Self) -> &mut Self {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
        self
    }

    pub fn map_point(&self, src: impl Into<Point3D>) -> Point3D {
        let mut dst = Point3D::default();
        unsafe {
            self.native()
                .mapPoint(src.into().native(), dst.native_mut())
        };
        dst
    }

    pub fn map_vector(&self, src: impl Into<Vector3D>) -> Vector3D {
        let mut dst = Vector3D::default();
        unsafe {
            self.native()
                .mapVector(src.into().native(), dst.native_mut())
        };
        dst
    }
}

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Patch3D {
    pub u: Vector3D,
    pub v: Vector3D,
    pub origin: Point3D,
}

impl NativeTransmutable<SkPatch3D> for Patch3D {}
#[test]
fn test_patch_3d_layout() {
    Patch3D::test_layout();
}

impl Default for Patch3D {
    fn default() -> Self {
        Patch3D::from_native(unsafe { SkPatch3D::new() })
    }
}

impl Patch3D {
    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    pub fn transform(&self, m: &Matrix3D) -> Self {
        let mut dst = Patch3D::default();
        unsafe { self.native().transform(m.native(), dst.native_mut()) }
        dst
    }

    pub fn dot_with(&self, v: impl Into<Vector3D>) -> scalar {
        unsafe { self.native().dotWith1(v.into().native()) }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Camera3D {
    pub location: Point3D,
    pub axis: Point3D,
    pub zenith: Point3D,
    pub observer: Point3D,
    orientation: Matrix,
    need_to_update: bool,
}

impl NativeTransmutable<SkCamera3D> for Camera3D {}
#[test]
fn test_camera_3d_layout() {
    Camera3D::test_layout();
}

impl Default for Camera3D {
    fn default() -> Self {
        Camera3D::from_native(unsafe { SkCamera3D::new() })
    }
}

impl Camera3D {
    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    pub fn update(&mut self) -> &mut Self {
        unsafe { self.native_mut().update() }
        self
    }

    pub fn patch_to_matrix(&self, quilt: &Patch3D) -> Matrix {
        let mut matrix = Matrix::default();
        unsafe {
            self.native()
                .patchToMatrix(quilt.native(), matrix.native_mut())
        }
        matrix
    }
}

// Note: original name is Sk3DView not SkView3D
// Also note that the implementation uses interior pointers,
// so we let Skia do the allocation.

pub struct View3D(*mut Sk3DView);

impl NativeAccess<Sk3DView> for View3D {
    fn native(&self) -> &Sk3DView {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut Sk3DView {
        unsafe { &mut *self.0 }
    }
}

impl Default for View3D {
    fn default() -> Self {
        View3D(unsafe { C_Sk3DView_new() })
    }
}

impl Drop for View3D {
    fn drop(&mut self) {
        unsafe { C_Sk3DView_delete(self.native_mut()) }
    }
}

impl View3D {
    pub fn save(&mut self) -> &mut Self {
        unsafe { self.native_mut().save() }
        self
    }

    pub fn restore(&mut self) -> &mut Self {
        unsafe { self.native_mut().restore() }
        self
    }

    pub fn translate(&mut self, d: impl Into<Vector3D>) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().translate(d.x, d.y, d.z) }
        self
    }

    pub fn rotate_x(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().rotateX(deg) }
        self
    }

    pub fn rotate_y(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().rotateY(deg) }
        self
    }

    pub fn rotate_z(&mut self, deg: scalar) -> &mut Self {
        unsafe { self.native_mut().rotateZ(deg) }
        self
    }

    pub fn matrix(&self) -> Matrix {
        let mut m = Matrix::default();
        unsafe { self.native().getMatrix(m.native_mut()) }
        m
    }

    pub fn apply_to_canvas(&self, mut canvas: impl AsMut<Canvas>) -> &Self {
        unsafe {
            self.native()
                .applyToCanvas(canvas.as_mut().native_mut())
        }
        self
    }

    pub fn dot_with_normal(&self, d: impl Into<Vector3D>) -> scalar {
        let d = d.into();
        unsafe { self.native().dotWithNormal(d.x, d.y, d.z) }
    }
}

#[test]
fn test_canvas_passing_syntax() {
    use crate::Surface;
    use crate::utils::new_null_canvas;

    let mut null_canvas = new_null_canvas();
    let view = View3D::default();
    // as mutable reference
    view.apply_to_canvas(&mut null_canvas);
    // moved
    view.apply_to_canvas(null_canvas);

    // and one with a mutable reference to a shared Canvas:
    let mut surface = Surface::new_raster_n32_premul((100, 100)).unwrap();
    view.apply_to_canvas(surface.canvas());
}
