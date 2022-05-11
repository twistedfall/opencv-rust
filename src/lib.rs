#![allow(broken_intra_doc_links)]

pub use cond_macros::*;
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
		hub_prelude::*,
		manual::prelude::*,
		traits::Boxed,
	};
	#[cfg(ocvrs_has_module_core)]
	pub use crate::core::{DataType, Mat};
}

/// Reexported platform types that are used by OpenCV
pub mod platform_types {
	pub use libc::{clock_t, FILE, ptrdiff_t, size_t};
}

/// Prelude for sys (externs) module and types
pub(crate) mod mod_prelude_sys {
	pub use std::os::raw::{c_char, c_void};

	pub use crate::{
		platform_types::*,
		traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
	};
}

/// Prelude for generated modules and types
pub(crate) mod mod_prelude {
	pub use crate::{
		Error,
		Result,
		core::{CV_MAKE_TYPE, CV_MAKETYPE},
		hub_prelude::*,
		mod_prelude_sys::*,
		boxed_cast_base,
		boxed_cast_descendant,
		input_output_array_ref_forward,
		opencv_type_boxed,
		opencv_type_enum,
		opencv_type_simple,
		ptr_cast_base,
		ptr_extern,
		ptr_extern_ctor,
		vector_copy_non_bool,
		vector_extern,
		vector_non_copy_or_bool,
	};
}

#[cfg(test)]
mod test;
mod cond_macros;
