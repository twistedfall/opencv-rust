#![allow(broken_intra_doc_links)]

pub use error::{Error, Result};

pub use crate::opencv::hub::*;

#[macro_use]
mod templ;

mod error;
mod opencv;
mod manual;
mod traits;

pub mod prelude {
	#[cfg(ocvrs_has_module_core)]
	pub use crate::core::{DataType, Mat};
	pub use crate::{
		hub_prelude::*,
		manual::prelude::*,
		traits::Boxed,
	};
}

/// Reexported platform types that are used by OpenCV
pub mod platform_types {
	pub use libc::{clock_t, FILE, ptrdiff_t, size_t};
}

pub(crate) mod mod_prelude_types {
	pub use std::os::raw::{c_char, c_void};

	pub use crate::{
		platform_types::*,
		traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
	};
}

pub(crate) mod mod_prelude {
	pub use crate::{
		Error,
		Result,
		core::{CV_MAKE_TYPE, CV_MAKETYPE},
		hub_prelude::*,
		mod_prelude_types::*,
		boxed_cast_base,
		boxed_cast_descendant,
		opencv_type_boxed,
		opencv_type_enum,
		opencv_type_simple,
		ptr_cast_base,
		ptr_extern,
		ptr_extern_ctor,
		vector_copy_non_bool,
		vector_extern,
		input_output_array_ref_forward,
		vector_non_copy_or_bool,
	};
}

#[cfg(test)]
mod test;
