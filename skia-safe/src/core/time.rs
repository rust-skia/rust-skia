use skia_bindings::SkTime_DateTime;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct DateTime {
    pub time_zone_minutes: i16,
    pub year: u16,
    pub month: u8,
    pub day_of_week: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

native_transmutable!(SkTime_DateTime, DateTime, date_time_layout);

// TODO: may wrap SkAutoTime?
