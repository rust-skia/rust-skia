pub mod sk64 {
    use crate::{SK_MAX_S32, SK_MIN_S32};

    pub fn pin_to_s32(x: i64) -> i32 {
        if x < i64::from(SK_MIN_S32) {
            return SK_MIN_S32;
        }
        if x > i64::from(SK_MAX_S32) {
            return SK_MAX_S32;
        }
        x as i32
    }
}

pub mod sk32 {
    use super::sk64;

    pub fn sat_add(a: i32, b: i32) -> i32 {
        sk64::pin_to_s32(i64::from(a) + i64::from(b))
    }

    pub fn sat_sub(a: i32, b: i32) -> i32 {
        sk64::pin_to_s32(i64::from(a) - i64::from(b))
    }

    #[allow(dead_code)]
    pub const fn can_overflow_add(a: i32, b: i32) -> i32 {
        ((a as u32) + (b as u32)) as i32
    }

    pub const fn can_overflow_sub(a: i32, b: i32) -> i32 {
        ((a as u32) - (b as u32)) as i32
    }
}
