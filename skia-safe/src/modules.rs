#[cfg(feature = "textlayout")]
pub(crate) mod paragraph;
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
