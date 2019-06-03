#![recursion_limit="128"]

#[macro_use]
extern crate cpp;

pub use error::{Error, Result};

pub use self::hub::*;

#[macro_use]
mod templ;

#[allow(unused_imports, non_snake_case, dead_code)]
#[allow(non_upper_case_globals, overflowing_literals)]
#[allow(non_camel_case_types)]
mod hub;

mod error;

pub mod prelude {
    pub use crate::{
        core::{DataType, Mat},
        templ::Vector,
    };
}

#[cfg(test)]
mod test;

