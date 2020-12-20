pub use skia_bindings::SkBlurStyle as BlurStyle;

#[test]
fn test_blur_style_naming() {
    let _ = BlurStyle::Outer;
}
