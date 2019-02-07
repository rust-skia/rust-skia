#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod canvas;
pub use canvas::Canvas;

pub use self::bindings::*;
