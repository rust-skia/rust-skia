#[cfg(feature = "textlayout")]
pub(crate) mod paragraph;
#[cfg(feature = "shaper")]
pub mod shaper;
#[cfg(feature = "shaper")]
pub use shaper::{icu, Shaper};

// Export everything below paragraph under textlayout
#[cfg(feature = "textlayout")]
pub mod textlayout {
    use crate::paragraph;
    pub use paragraph::*;
}
