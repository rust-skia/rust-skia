use crate::prelude::*;
use skia_bindings::SkTime_DateTime;

// TODO: should we support chrono interop here?

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct DateTime {
    pub time_zone_minutes : i16,
    pub year: u16,
    pub month: u8,
    pub day_of_week: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8
}

impl NativeTransmutable<SkTime_DateTime> for DateTime {}
#[test]
fn test_date_time_layout() {
    DateTime::test_layout();
}
