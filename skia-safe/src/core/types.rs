use crate::prelude::NativeTransmutable;
use skia_bindings::{SkBudgeted, SkFourByteTag};
use std::ops::Deref;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Budgeted(bool);

impl NativeTransmutable<SkBudgeted> for Budgeted {}

#[test]
fn test_budgeted_layout() {
    Budgeted::test_layout()
}

impl Budgeted {
    pub const NO: Budgeted = Budgeted(false);
    pub const YES: Budgeted = Budgeted(true);
}

//
// FourByteTag
//

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct FourByteTag(SkFourByteTag);

impl NativeTransmutable<SkFourByteTag> for FourByteTag {}

#[test]
fn test_four_byte_tag_layout() {
    FourByteTag::test_layout()
}

impl Deref for FourByteTag {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for FourByteTag {
    fn from(v: u32) -> Self {
        FourByteTag(v)
    }
}

impl FourByteTag {
    pub fn a(&self) -> u8 {
        (self.into_native() >> 24) as u8
    }

    pub fn b(&self) -> u8 {
        (self.into_native() >> 16) as u8
    }

    pub fn c(&self) -> u8 {
        (self.into_native() >> 8) as u8
    }

    pub fn d(&self) -> u8 {
        self.into_native() as u8
    }
}
