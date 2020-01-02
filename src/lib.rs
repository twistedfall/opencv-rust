#![allow(intra_doc_link_resolution_failure)]

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

pub(crate) mod mod_prelude {
	pub use std::os::raw::{c_char, c_void};

	pub use libc::{ptrdiff_t, size_t};

	pub use crate::{
		Error,
		Result,
	};
}

#[cfg(test)]
mod test;

cpp! {{
    #include "../common_opencv.h"
    using namespace cv;
    #include "cpp/common.hpp"
    #include "../types.h"
    #include "../return_types.h"
}}
