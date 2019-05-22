use crate::prelude::*;
use skia_bindings::{
    C_SkFontArguments_construct, C_SkFontArguments_destruct,
    C_SkFontArguments_setVariationDesignPosition, SkFontArguments,
    SkFontArguments_VariationPosition,
};
use std::marker::PhantomData;
use std::mem::forget;
use std::{mem, slice};

#[deprecated(since = "0.11.0", note = "use font_arguments::VariationPosition instead")]
pub type FontArgumentsVariationPosition<'a> = font_arguments::VariationPosition<'a>;

#[deprecated(since = "0.11.0", note = "use font_arguments::variation_position::Coordinate instead")]
pub type FontArgumentsVariationPositionCoordinate = font_arguments::variation_position::Coordinate;

pub mod font_arguments {

    #[derive(Debug)]
    pub struct VariationPosition<'a> {
        pub coordinates: &'a [variation_position::Coordinate],
    }

    pub mod variation_position {
        use crate::prelude::*;
        use crate::FourByteTag;
        use skia_bindings::SkFontArguments_VariationPosition_Coordinate;

        #[derive(Copy, Clone, PartialEq, Debug)]
        #[repr(C)]
        pub struct Coordinate {
            pub axis: FourByteTag,
            pub value: f32,
        }

        impl NativeTransmutable<SkFontArguments_VariationPosition_Coordinate> for Coordinate {}
        #[test]
        fn test_coordinate_layout() {
            Coordinate::test_layout()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FontArguments<'a> {
    args: SkFontArguments,
    pd: PhantomData<&'a [font_arguments::variation_position::Coordinate]>,
}

impl<'a> NativeTransmutable<SkFontArguments> for FontArguments<'a> {}
#[test]
fn test_font_arguments_layout() {
    FontArguments::test_layout()
}

impl<'a> Drop for FontArguments<'a> {
    fn drop(&mut self) {
        unsafe { C_SkFontArguments_destruct(self.native_mut()) }
    }
}

impl<'a> Default for FontArguments<'a> {
    fn default() -> Self {
        FontArguments::new()
    }
}

impl<'a> FontArguments<'a> {
    pub fn new() -> Self {
        Self::from_native(unsafe {
            // does not link under Linux / macOS
            // SkFontArguments::new()
            let mut font_arguments = mem::zeroed();
            C_SkFontArguments_construct(&mut font_arguments);
            font_arguments
        })
    }

    pub fn set_collection_index(&mut self, collection_index: usize) -> &mut Self {
        unsafe {
            self.native_mut()
                .setCollectionIndex(collection_index.try_into().unwrap());
        }
        self
    }

    // This function consumes self for it to be able to change its lifetime,
    // because it borrows the coordinates referenced by FontArgumentsVariationPosition.
    #[allow(clippy::needless_lifetimes)]
    pub fn set_variation_design_position<'position>(
        mut self,
        position: font_arguments::VariationPosition<'position>,
    ) -> FontArguments<'position> {
        let position = SkFontArguments_VariationPosition {
            coordinates: position.coordinates.native().as_ptr(),
            coordinateCount: position.coordinates.len().try_into().unwrap(),
        };
        unsafe {
            // does not link on Linux / MacOS:
            C_SkFontArguments_setVariationDesignPosition(self.native_mut(), position);
            // TODO: is there a more elegant way to change the lifetime of self?
            let r = mem::transmute_copy(&self);
            forget(self);
            r
        }
    }

    pub fn collection_index(&self) -> usize {
        unsafe { self.native().getCollectionIndex() }
            .try_into()
            .unwrap()
    }

    pub fn variation_design_position(&self) -> font_arguments::VariationPosition {
        // TODO: build a extern "C" wrapper for the function getVariationDesignPosition().
        let position = self.native().fVariationDesignPosition;
        font_arguments::VariationPosition {
            coordinates: unsafe {
                slice::from_raw_parts(
                    position.coordinates as *const _,
                    position.coordinateCount.try_into().unwrap(),
                )
            },
        }
    }
}

#[test]
fn test_font_arguments_with_no_coordinates() {
    let fa = FontArguments::new();
    dbg!(&fa);
    let coordinates = fa.variation_design_position();
    assert_eq!(coordinates.coordinates, []);
}

#[test]
fn access_coordinates() {
    let coordinates = Box::new([font_arguments::variation_position::Coordinate {
        axis: 0.into(),
        value: 1.0,
    }]);
    let args = FontArguments::new();
    let args = args.set_variation_design_position(font_arguments::VariationPosition {
        coordinates: coordinates.as_ref(),
    });
    assert_eq!(args.variation_design_position().coordinates[0].value, 1.0);
    drop(args);
}
