#![allow(intra_doc_link_resolution_failure)]

pub use error::{Error, Result};

pub use crate::opencv::hub::*;

#[macro_use]
mod templ;

mod error;
mod opencv;
mod manual;

pub mod prelude {
	pub use crate::{
		core::{DataType, Mat},
		templ::Vector,
	};
	pub use crate::hub_prelude::*;
	pub use crate::manual::prelude::*;
}

pub(crate) mod mod_prelude_types {
	pub use std::os::raw::{c_char, c_void};

	pub use libc::{clock_t, FILE, ptrdiff_t, size_t};

	pub use crate::core::Boxed;
}

pub(crate) mod mod_prelude {
	pub use crate::{
		boxed_ptr,
		core::{CV_MAKE_TYPE, CV_MAKETYPE},
		Error,
		mod_prelude_types::*,
		ptr_extern,
		Result,
	};
}

#[cfg(test)]
mod test;
