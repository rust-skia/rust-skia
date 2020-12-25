use crate::prelude::*;
use skia_bindings::{SkCubicResampler, SkFilterOptions, SkSamplingOptions};

pub use skia_bindings::SkSamplingMode as SamplingMode;

pub use skia_bindings::SkMipmapMode as MipmapMode;

/// Specify B and C (each between 0...1) to create a shader that applies the corresponding
/// cubic reconstruction filter to the image.
///
/// Example values:
///     b = 1/3, c = 1/3        "Mitchell" filter
///     b = 0,   c = 1/2        "Catmull-Rom" filter
///
/// See "Reconstruction Filters in Computer Graphics"
///         Don P. Mitchell
///         Arun N. Netravali
///         1988
/// https://www.cs.utexas.edu/~fussell/courses/cs384g-fall2013/lectures/mitchell/Mitchell.pdf
/// Desmos worksheet https://www.desmos.com/calculator/aghdpicrvr
/// Nice overview https://entropymine.com/imageworsener/bicubic/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CubicResampler {
    b: f32,
    c: f32,
}

impl NativeTransmutable<SkCubicResampler> for CubicResampler {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FilterOptions {
    pub sampling: SamplingMode,
    pub mipmap: MipmapMode,
}

impl NativeTransmutable<SkFilterOptions> for FilterOptions {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SamplingOptions {
    pub use_cubic: bool,
    /// use if `use_cubic` is `true`
    pub cubic: CubicResampler,
    /// use if `use_cubic` is `false`
    pub filter: FilterOptions,
}

impl NativeTransmutable<SkSamplingOptions> for SamplingOptions {}

impl Default for SamplingOptions {
    fn default() -> Self {
        Self {
            use_cubic: false,
            // ignored
            cubic: CubicResampler { b: 0.0, c: 0.0 },
            filter: FilterOptions {
                sampling: SamplingMode::Nearest,
                mipmap: MipmapMode::None,
            },
        }
    }
}

impl From<FilterOptions> for SamplingOptions {
    fn from(filter: FilterOptions) -> Self {
        Self {
            use_cubic: false,
            cubic: CubicResampler { b: 0.0, c: 0.0 },
            filter,
        }
    }
}

impl From<CubicResampler> for SamplingOptions {
    fn from(cubic: CubicResampler) -> Self {
        Self {
            use_cubic: true,
            cubic,
            // ignored
            filter: FilterOptions {
                sampling: SamplingMode::Nearest,
                mipmap: MipmapMode::None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::NativeTransmutable;

    #[test]
    fn test_naming() {
        let _ = super::SamplingMode::Linear;
        let _ = super::MipmapMode::Nearest;
    }

    #[test]
    fn test_cubic_sampler_layout() {
        super::CubicResampler::test_layout();
    }

    #[test]
    fn test_filter_options_layout() {
        super::FilterOptions::test_layout()
    }

    #[test]
    fn test_sampling_options_layout() {
        super::SamplingOptions::test_layout()
    }
}
