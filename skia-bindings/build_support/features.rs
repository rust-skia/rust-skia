use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Features {
    /// Build with OpenGL support?
    pub gl: bool,

    /// Build with EGL support? If you set X11, setting this to false will use LibGL (GLX)
    pub egl: bool,

    /// Build with Wayland support? This requires EGL, as GLX does not work on Wayland.
    pub wayland: bool,

    /// Build with X11 support?
    pub x11: bool,

    /// Build with Vulkan support?
    pub vulkan: bool,

    /// Build with Metal support?
    pub metal: bool,

    /// Build with Direct3D support?
    pub d3d: bool,

    /// Features related to text layout. Modules skshaper and skparagraph.
    pub text_layout: bool,

    /// Support for rendering SVG.
    pub svg: bool,

    /// Support the encoding of bitmap data to the WEBP image format.
    pub webp_encode: bool,

    /// Support the decoding of the WEBP image format to bitmap data.
    pub webp_decode: bool,

    /// Build with FreeType embedded.
    pub embed_freetype: bool,

    /// Build with animation support (yet unsupported, no wrappers).
    pub animation: bool,

    /// Support DNG file format (currently unsupported because of build errors).
    pub dng: bool,

    /// Build the particles module (unsupported, no wrappers).
    pub particles: bool,
}

impl Default for Features {
    /// Build a Features set based on the current environment cargo supplies us with.
    fn default() -> Self {
        Features {
            gl: cfg!(feature = "gl"),
            egl: cfg!(feature = "egl"),
            wayland: cfg!(feature = "wayland"),
            x11: cfg!(feature = "x11"),
            vulkan: cfg!(feature = "vulkan"),
            metal: cfg!(feature = "metal"),
            d3d: cfg!(feature = "d3d"),
            text_layout: cfg!(feature = "textlayout"),
            svg: cfg!(feature = "svg"),
            webp_encode: cfg!(feature = "webp-encode"),
            webp_decode: cfg!(feature = "webp-decode"),
            embed_freetype: cfg!(feature = "embed-freetype"),
            animation: false,
            dng: false,
            particles: false,
        }
    }
}

impl Features {
    pub fn gpu(&self) -> bool {
        self.gl || self.vulkan || self.metal || self.d3d
    }

    /// Feature Ids used to look up prebuilt binaries.
    pub fn ids(&self) -> HashSet<&str> {
        let mut feature_ids = Vec::new();

        if self.gl {
            feature_ids.push(feature_id::GL);
        }
        if self.egl {
            feature_ids.push(feature_id::EGL);
        }
        if self.x11 {
            feature_ids.push(feature_id::X11);
        }
        if self.wayland {
            feature_ids.push(feature_id::WAYLAND);
        }
        if self.vulkan {
            feature_ids.push(feature_id::VULKAN);
        }
        if self.metal {
            feature_ids.push(feature_id::METAL);
        }
        if self.d3d {
            feature_ids.push(feature_id::D3D);
        }
        if self.text_layout {
            feature_ids.push(feature_id::TEXTLAYOUT);
        }
        if self.svg {
            feature_ids.push(feature_id::SVG);
        }
        if self.webp_encode {
            feature_ids.push(feature_id::WEBPE);
        }
        if self.webp_decode {
            feature_ids.push(feature_id::WEBPD);
        }
        if self.embed_freetype {
            feature_ids.push(feature_id::EMBED_FREETYPE);
        }

        feature_ids.into_iter().collect()
    }
}

/// Feature identifiers define the additional configuration parts of the binaries to download.
mod feature_id {
    pub const GL: &str = "gl";
    pub const VULKAN: &str = "vulkan";
    pub const METAL: &str = "metal";
    pub const D3D: &str = "d3d";
    pub const TEXTLAYOUT: &str = "textlayout";
    pub const SVG: &str = "svg";
    pub const WEBPE: &str = "webpe";
    pub const WEBPD: &str = "webpd";
    pub const EGL: &str = "egl";
    pub const X11: &str = "x11";
    pub const WAYLAND: &str = "wayland";
    pub const EMBED_FREETYPE: &str = "freetype";
}
