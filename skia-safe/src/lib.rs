#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::non_send_fields_in_send_ty)]
// https://github.com/rust-lang/rust/issues/93367
#![allow(unknown_lints)]
#![allow(clippy::too_long_first_doc_paragraph)]
#![allow(clippy::doc_overindented_list_items)]
#![allow(mismatched_lifetime_syntaxes)]

#[cfg(feature = "gpu")]
compile_error!(
    "feature `gpu` has been renamed to `ganesh`; replace `gpu` with `ganesh`. The `vulkan` and `metal` features require either `ganesh` or `graphite`."
);

#[cfg(all(
    any(feature = "vulkan", feature = "metal"),
    not(any(feature = "ganesh", feature = "graphite"))
))]
compile_error!(
    "the `vulkan` and `metal` features require at least one rendering engine: `ganesh` or `graphite`"
);

mod macros;

pub mod codec;
#[deprecated(since = "0.33.1", note = "use codec::Result")]
pub use codec::Result as CodecResult;
pub use codec::{Codec, EncodedImageFormat, EncodedOrigin, codecs};

mod core;
#[cfg(feature = "pdf")]
mod docs;
mod effects;
mod encode_;
pub mod gpu;
mod interop;
mod modules;
mod pathops;
mod prelude;
pub(crate) mod private;
pub mod skottie;
pub mod svg;
pub mod wrapper;
// TODO: We don't export utils/* into the crate's root yet. Should we?
pub mod utils;

#[macro_use]
extern crate bitflags;

// Prelude re-exports
pub use crate::prelude::{Borrows, ConditionallySend, Handle, RCHandle, RefHandle, Sendable};

// All Sk* types are accessible via skia_safe::
pub use crate::core::*;
#[cfg(feature = "pdf")]
pub use docs::*;
pub use effects::*;
pub use encode_::*;
#[allow(unused_imports)]
pub use modules::*;
pub use pathops::*;

#[cfg(test)]
mod transmutation_tests {
    use crate::{Point, prelude::NativeTransmutableSliceAccess};
    use skia_bindings::SkPoint;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_transmutation_of_fixed_size_arrays_to_slice() {
        let mut points = [Point::default(); 4];

        let points_native = points.native_mut();
        let native_point = SkPoint { fX: 10.0, fY: 11.0 };
        points_native[1] = native_point;

        assert_eq!(points[1].x, native_point.fX);
        assert_eq!(points[1].y, native_point.fY);
    }
}
