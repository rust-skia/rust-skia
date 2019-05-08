use crate::prelude::*;
use crate::FourByteTag;
use skia_bindings::{
    C_SkFontArguments_destruct, SkFontArguments, SkFontArguments_VariationPosition,
    SkFontArguments_VariationPosition_Coordinate,
};
use std::marker::PhantomData;
use std::mem::forget;
use std::{mem, slice};

#[derive(Debug)]
pub struct FontArgumentsVariationPosition<'a> {
    pub coordinates: &'a [VariationPositionCoordinate],
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct VariationPositionCoordinate {
    pub axis: FourByteTag,
    pub value: f32,
}

impl NativeTransmutable<SkFontArguments_VariationPosition_Coordinate>
    for VariationPositionCoordinate
{
}
#[test]
fn test_variation_position_coordinate_layout() {
    VariationPositionCoordinate::test_layout()
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
        Self::from_native(unsafe { SkFontArguments::new() })
    }

    pub fn set_collection_index(&mut self, collection_index: usize) -> &mut Self {
        unsafe {
            self.native_mut()
                .setCollectionIndex(collection_index.try_into().unwrap());
        }
        self
    }

    // This function must consume self its lifetime needs to be changed, because it
    // borrows the coordinates referenced by FontArgumentsVariationPosition.
    pub fn set_variation_design_position(
        mut self,
        position: FontArgumentsVariationPosition,
    ) -> FontArguments /* NEVER USE Self here, this returns a different lifetime */ {
        let proxy = SkFontArguments_VariationPosition {
            coordinates: position.coordinates.native().as_ptr(),
            coordinateCount: position.coordinates.len().try_into().unwrap(),
        };
        unsafe {
            self.native_mut().setVariationDesignPosition(proxy);
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
    let coordinates = Box::new([VariationPositionCoordinate {
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
