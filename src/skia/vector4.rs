use rust_skia::SkVector4;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Vector4 { x, y, z, w }
    }
}

impl From<(f32, f32, f32)> for Vector4 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vector4::from((x, y, z, 1.0))
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(v4: [f32; 4]) -> Self {
        Vector4::from((v4[0], v4[1], v4[2], v4[3]))
    }
}

impl From<[f32; 3]> for Vector4 {
    fn from(v3: [f32; 3]) -> Self {
        Vector4::from((v3[0], v3[1], v3[2]))
    }
}

impl From<SkVector4> for Vector4 {
    fn from(v4: SkVector4) -> Self {
        let d = &v4.fData;
        Vector4::from((d[0], d[1], d[2], d[3]))
    }
}

impl Into<SkVector4> for Vector4 {
    fn into(self) -> SkVector4 {
        SkVector4 { fData: [self.x, self.y, self.z, self.w]}
    }
}

impl Vector4 {
    pub fn new() -> Vector4 {
        Vector4::from((0.0, 0.0, 0.0, 1.0))
    }
}
