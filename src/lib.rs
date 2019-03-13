pub use self::hub::*;

#[allow(unused_imports, non_snake_case, dead_code)]
#[allow(non_upper_case_globals, overflowing_literals)]
#[allow(non_camel_case_types)]
mod hub;

pub fn mat() -> core::Mat {
    core::Mat::new().unwrap()
}
