use std::{collections::HashSet, ops::Index};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Features(HashSet<&'static str>);

impl Default for Features {
    fn default() -> Self {
        let mut features = HashSet::new();

        if cfg!(feature = "gl") {
            features.insert(feature_id::GL);
        }
        if cfg!(feature = "egl") {
            features.insert(feature_id::EGL);
        }
        if cfg!(feature = "wayland") {
            features.insert(feature_id::WAYLAND);
        }
        if cfg!(feature = "x11") {
            features.insert(feature_id::X11);
        }
        if cfg!(feature = "vulkan") {
            features.insert(feature_id::VULKAN);
        }
        if cfg!(feature = "metal") {
            features.insert(feature_id::METAL);
        }
        if cfg!(feature = "d3d") {
            features.insert(feature_id::D3D);
        }
        if cfg!(feature = "textlayout") {
            features.insert(feature_id::TEXTLAYOUT);
        }
        if cfg!(feature = "svg") {
            features.insert(feature_id::SVG);
        }
        if cfg!(feature = "webp-encode") {
            features.insert(feature_id::WEBPE);
        }
        if cfg!(feature = "webp-decode") {
            features.insert(feature_id::WEBPD);
        }
        if cfg!(feature = "pdf") {
            features.insert(feature_id::PDF);
        }
        if cfg!(feature = "embed-freetype") {
            features.insert(feature_id::EMBED_FREETYPE);
        }
        if cfg!(feature = "freetype-woff2") {
            features.insert(feature_id::FT_WOFF2);
        }

        Features(features)
    }
}

impl Features {
    pub fn gpu(&self) -> bool {
        self[feature_id::GL]
            || self[feature_id::VULKAN]
            || self[feature_id::METAL]
            || self[feature_id::D3D]
    }

    pub fn ids(&self) -> HashSet<&str> {
        self.0.clone()
    }

    pub fn contains(&self, feature: &str) -> bool {
        self.0.contains(feature)
    }

    pub fn enable(&mut self, feature: &'static str) {
        self.0.insert(feature);
    }

    pub fn disable(&mut self, feature: &'static str) {
        self.0.remove(feature);
    }

    pub fn set(&mut self, feature: &'static str, enabled: bool) {
        if enabled {
            self.enable(feature);
        } else {
            self.disable(feature);
        }
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

/// Feature identifiers define the additional configuration parts of the binaries to download.
pub mod feature_id {
    /// Build with OpenGL support
    pub const GL: &str = "gl";
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
    /// Support PDF rendering
    pub const PDF: &str = "pdf";
    /// Build with EGL support. If you set X11, setting this to false will use LibGL (GLX)
    pub const EGL: &str = "egl";
    /// Build with X11 support
    pub const X11: &str = "x11";
    /// Build with Wayland support. This requires EGL, as GLX does not work on Wayland
    pub const WAYLAND: &str = "wayland";
    /// Build with FreeType embedded
    pub const EMBED_FREETYPE: &str = "freetype";
    /// Build with FreeType WOFF2 support
    pub const FT_WOFF2: &str = "ftwoff2";
}
