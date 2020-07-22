use crate::PathEffect;
use skia_bindings as sb;

pub fn new() -> PathEffect {
    PathEffect::from_ptr(unsafe { sb::C_SkStrokeAndFillePathEffect_Make() }).unwrap()
}

#[test]
fn test_new() {
    let _effect = new();
}
