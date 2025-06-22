use std::{
    collections::HashSet,
    fmt,
    ops::{self, Index},
};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Features(HashSet<&'static str>);

impl fmt::Display for Features {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strs: Vec<_> = self.0.iter().cloned().collect();
        strs.sort();
        strs.join(",").fmt(f)
    }
}

impl Features {
    /// Returns the set of features compatible with this platform and warns about features that were
    /// configured, but not supported on the target.
    pub fn from_cargo_env() -> Self {
        let mut features = Self::default();

        if cfg!(feature = "pdf") {
            features += feature_id::PDF;
        }

        if cfg!(feature = "gl") {
            features += feature_id::GL;
        }
        if cfg!(feature = "egl") {
            features += feature_id::EGL;
        }
        if cfg!(feature = "wayland") {
            features += feature_id::WAYLAND;
        }
        if cfg!(feature = "x11") {
            features += feature_id::X11;
        }

        if cfg!(feature = "vulkan") {
            features += feature_id::VULKAN;
        }
        if cfg!(feature = "metal") {
            features += feature_id::METAL;
        }
        if cfg!(feature = "d3d") {
            features += feature_id::D3D;
        }

        if cfg!(feature = "textlayout") {
            features += feature_id::TEXTLAYOUT;
        }
        if cfg!(feature = "svg") {
            features += feature_id::SVG;
        }
        if cfg!(feature = "webp-encode") {
            features += feature_id::WEBPE;
        }
        if cfg!(feature = "webp-decode") {
            features += feature_id::WEBPD;
        }

        if cfg!(feature = "embed-freetype") {
            features += feature_id::FT_EMBED;
        }
        if cfg!(feature = "freetype-woff2") {
            features += feature_id::FT_WOFF2;
        }

        features
    }

    pub fn gpu(&self) -> bool {
        self[feature_id::GL]
            || self[feature_id::VULKAN]
            || self[feature_id::METAL]
            || self[feature_id::D3D]
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn enable(&mut self, feature: &'static str) {
        self.0.insert(feature);
    }

    fn disable(&mut self, feature: &'static str) {
        self.0.remove(feature);
    }

    pub fn set(&mut self, feature: &'static str, enabled: bool) {
        if enabled {
            self.enable(feature);
        } else {
            self.disable(feature);
        }
    }

    /// A comparable set of feature ids (sorted and joined by `-`).
    pub fn to_key(&self) -> String {
        let mut features: Vec<&str> = self.0.iter().cloned().collect();
        features.sort();
        features.join("-")
    }

    /// Returns missing dependent features.
    ///
    /// If the returned set of features is not empty, this is most likely a configuration error.
    pub fn missing_dependencies(&self) -> Features {
        let mut missing = Features::default();

        for (feature, dependencies) in feature_id::DEPENDENCIES {
            if !self[feature] {
                continue;
            }

            for dependency in *dependencies {
                if !self[dependency] {
                    missing += dependency
                }
            }
        }

        missing
    }
}

impl<const N: usize> From<[&'static str; N]> for Features {
    fn from(features_array: [&'static str; N]) -> Self {
        Features(HashSet::from_iter(features_array))
    }
}

impl Index<&str> for Features {
    type Output = bool;

    fn index(&self, index: &str) -> &Self::Output {
        if self.0.contains(index) {
            &true
        } else {
            &false
        }
    }
}

impl ops::AddAssign<&'static str> for Features {
    fn add_assign(&mut self, rhs: &'static str) {
        self.enable(rhs);
    }
}

impl ops::SubAssign<&'static str> for Features {
    fn sub_assign(&mut self, rhs: &'static str) {
        self.disable(rhs);
    }
}

/// Feature identifiers define the additional configuration parts of the binaries to download.
pub mod feature_id {
    /// Support PDF rendering.
    pub const PDF: &str = "pdf";

    /// Build with OpenGL support
    pub const GL: &str = "gl";
    /// Build with EGL support. If you set X11, setting this to false will use LibGL (GLX)
    pub const EGL: &str = "egl";
    /// Build with X11 support
    pub const X11: &str = "x11";
    /// Build with Wayland support. This requires EGL, as GLX does not work on Wayland
    pub const WAYLAND: &str = "wayland";

    /// Build with Vulkan support
    pub const VULKAN: &str = "vulkan";
    /// Build with Metal support
    pub const METAL: &str = "metal";
    /// Build with Direct3D support
    pub const D3D: &str = "d3d";

    /// Features related to text layout. Modules skshaper and skparagraph
    pub const TEXTLAYOUT: &str = "textlayout";
    /// Support for rendering SVG
    pub const SVG: &str = "svg";
    /// Support the encoding of bitmap data to the WEBP image format
    pub const WEBPE: &str = "webpe";
    /// Support the decoding of the WEBP image format to bitmap data
    pub const WEBPD: &str = "webpd";

    /// Build with FreeType embedded
    pub const FT_EMBED: &str = "freetype";
    /// Build with FreeType WOFF2 support
    pub const FT_WOFF2: &str = "ftwoff2";

    pub const FREETYPE_SPECIFIC: &[&str] = &[FT_EMBED, FT_WOFF2];

    pub const DEPENDENCIES: &[(&str, &[&str])] = &[(EGL, &[GL]), (X11, &[GL]), (WAYLAND, &[EGL])];
}
