pub use skia_bindings::SkBlendMode as BlendMode;
#[test]
pub fn test_blend_mode_naming() {
    let _ = BlendMode::ColorBurn;
}

pub use skia_bindings::SkBlendModeCoeff as BlendModeCoeff;
#[test]
pub fn test_blend_mode_coeff_naming() {
    let _ = BlendModeCoeff::IDA;
}
