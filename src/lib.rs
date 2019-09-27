#[macro_use]
extern crate cpp;

pub use error::{Error, Result};

pub use crate::opencv::hub::*;

#[macro_use]
mod templ;

mod opencv;
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
