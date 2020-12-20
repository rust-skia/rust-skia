pub use skia_bindings::SkCoverageMode as CoverageMode;
#[test]
fn test_coverage_mode_naming() {
    let _ = CoverageMode::Union;
}
