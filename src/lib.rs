#![recursion_limit = "128"]

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

mod manual;

mod error;

pub mod prelude {
    pub use crate::{
        core::{DataType, Mat},
        templ::Vector,
    };
}

#[cfg(test)]
mod test;

cpp! {{
    #include "../common_opencv.h"
    using namespace cv;
    #include "common.h"
    #include "../types.h"
    #include "../return_types.h"
}}
