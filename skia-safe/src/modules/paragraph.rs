use crate::interop;
use std::ops::Index;

mod dart_types;
pub use dart_types::*;

mod font_collection;
pub use font_collection::*;

mod paragraph;
pub use paragraph::*;

mod paragraph_builder;
pub use paragraph_builder::*;

mod paragraph_style;
pub use paragraph_style::*;

mod text_shadow;
pub use text_shadow::*;

mod text_style;
pub use text_style::*;

mod typeface_font_provider;
pub use typeface_font_provider::*;

/// Efficient reference type to a C++ vector of font family SkStrings.
///
/// Use indexer or .iter() to access the Rust str references.
pub struct FontFamilies<'a>(&'a [skia_bindings::SkString]);

impl Index<usize> for FontFamilies<'_> {
    type Output = str;
    fn index(&self, index: usize) -> &Self::Output {
        interop::String::from_native_ref(&self.0[index]).as_str()
    }
}

impl FontFamilies<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0
            .iter()
            .map(|str| interop::String::from_native_ref(str).as_str())
    }
}
