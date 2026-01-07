#[cfg(feature = "svg")]
pub mod image_asset;
#[cfg(feature = "textlayout")]
pub(crate) mod paragraph;
#[cfg(feature = "svg")]
pub mod resources;
#[cfg(feature = "textlayout")]
pub mod shaper;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "textlayout")]
pub use shaper::{icu, Shaper};

// Export everything below paragraph under textlayout
#[cfg(feature = "textlayout")]
pub mod textlayout {
    pub use super::paragraph::*;
}

#[cfg(feature = "textlayout")]
pub mod shapers {
    // Re-exports `shapers::primitive`.
    pub use crate::shaper::shapers::*;

    pub mod ct {
        pub use crate::shaper::core_text::*;
    }

    pub mod hb {
        pub use crate::shaper::harfbuzz::*;
    }

    pub mod unicode {
        pub use crate::shaper::unicode::*;
    }
}
