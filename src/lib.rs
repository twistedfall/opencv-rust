#![allow(intra_doc_link_resolution_failure)]

pub use error::{Error, Result};

pub use crate::opencv::hub::*;

#[macro_use]
mod templ;

mod error;
mod opencv;
mod manual;
mod traits;

pub mod prelude {
	pub use crate::{
		core::{DataType, Mat},
		hub_prelude::*,
		manual::prelude::*,
		traits::{Boxed, VectorTrait},
	};
}

pub(crate) mod mod_prelude_types {
	pub use std::os::raw::{c_char, c_void};

	pub use libc::{clock_t, FILE, ptrdiff_t, size_t};

	pub use crate::traits::{Boxed, OpenCVType, OpenCVTypeExternContainer, VectorTrait};
}

pub(crate) mod mod_prelude {
	pub use crate::{
		core::{CV_MAKE_TYPE, CV_MAKETYPE},
		Error,
		mod_prelude_types::*,
		opencv_type_boxed,
		opencv_type_enum,
		opencv_type_simple,
		ptr_extern,
		Result,
		vector_copy_non_bool,
		vector_extern,
		vector_non_copy_or_bool,
	};
}

#[cfg(test)]
mod test;
