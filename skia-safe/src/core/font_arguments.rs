use crate::prelude::*;
use crate::FourByteTag;
use skia_bindings::{
    C_SkFontArguments_construct, C_SkFontArguments_destruct,
    C_SkFontArguments_setVariationDesignPosition, SkFontArguments,
    SkFontArguments_VariationPosition, SkFontArguments_VariationPosition_Coordinate,
};
use std::marker::PhantomData;
use std::mem::forget;
use std::{mem, slice};

#[derive(Debug)]
pub struct FontArgumentsVariationPosition<'a> {
    pub coordinates: &'a [FontArgumentsVariationPositionCoordinate],
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct FontArgumentsVariationPositionCoordinate {
    pub axis: FourByteTag,
    pub value: f32,
}

impl NativeTransmutable<SkFontArguments_VariationPosition_Coordinate>
    for FontArgumentsVariationPositionCoordinate
{
}
#[test]
fn test_variation_position_coordinate_layout() {
    FontArgumentsVariationPositionCoordinate::test_layout()
}

// Need to assign a lifetime to FontArguments, because it borrows [VariationPositionCoordinate].
#[repr(C)]
#[derive(Debug)]
pub struct FontArguments<'a> {
    args: SkFontArguments,
    pd: PhantomData<&'a ()>,
}

impl<'a> NativeTransmutable<SkFontArguments> for FontArguments<'a> {}
#[test]
fn test_font_arguments_layout() {
    FontArguments::test_layout()
}

impl<'a> Drop for FontArguments<'a> {
    fn drop(&mut self) {
        println!("drop fa");
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

    // This function consumes self to be able to change its lifetime, because it
    // borrows the coordinates referenced by FontArgumentsVariationPosition.
    // Also don't use liftime elision here for documentation.
    pub fn set_variation_design_position<'position>(
        mut self,
        position: FontArgumentsVariationPosition<'position>,
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

    pub fn variation_design_position(&self) -> FontArgumentsVariationPosition {
        // TODO: find out why calling getVariationDesignPosition() returns garbage
        // (tested on Windows).
        // TODO: build a extern "C" wrapper for the function getVariationDesignPosition().
        let position = self.native().fVariationDesignPosition;
        FontArgumentsVariationPosition {
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
    let coordinates = Box::new([FontArgumentsVariationPositionCoordinate {
        axis: 0.into(),
        value: 1.0,
    }]);
    let args = FontArguments::new();
    let args = args.set_variation_design_position(FontArgumentsVariationPosition {
        coordinates: coordinates.as_ref(),
    });
    assert_eq!(args.variation_design_position().coordinates[0].value, 1.0);
    drop(args);
}
