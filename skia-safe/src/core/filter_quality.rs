pub use skia_bindings::SkFilterQuality as FilterQuality;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_quality_naming() {
        let _ = FilterQuality::High;
    }
}
