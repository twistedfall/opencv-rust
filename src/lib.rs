#![recursion_limit = "128"]

#[macro_use]
extern crate cpp;

pub use error::{Error, Result};
#[cfg(feature = "opencv-34")]
pub use opencv_34::hub::*;
#[cfg(feature = "opencv-41")]
pub use opencv_41::hub::*;

#[macro_use]
mod templ;

#[cfg(feature = "opencv-34")]
mod opencv_34;
#[cfg(feature = "opencv-41")]
mod opencv_41;
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
