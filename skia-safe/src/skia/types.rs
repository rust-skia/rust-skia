use crate::prelude::NativeTransmutable;
use skia_bindings::SkBudgeted;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Budgeted(bool);

impl NativeTransmutable<SkBudgeted> for Budgeted {}

impl Budgeted {
    pub const NO : Budgeted = Budgeted(false);
    pub const YES : Budgeted = Budgeted(true);
}
