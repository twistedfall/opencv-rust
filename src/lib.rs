#![allow(broken_intra_doc_links)]

pub use cond_macros::*;
pub use error::{Error, Result};

pub use crate::opencv::hub::*;

#[macro_use]
mod templ;

mod error;
mod manual;
mod opencv;
mod traits;

pub mod prelude {
	#[cfg(ocvrs_has_module_core)]
	pub use crate::core::{DataType, Mat};
	pub use crate::{hub_prelude::*, manual::prelude::*, traits::Boxed};
}

/// Reexported platform types that are used by OpenCV
pub mod platform_types {
	pub use libc::{clock_t, ptrdiff_t, size_t, FILE};
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
		boxed_cast_base, boxed_cast_descendant,
		core::{CV_MAKETYPE, CV_MAKE_TYPE},
		extern_arg_send, extern_container_send, extern_receive, extern_send,
		hub_prelude::*,
		input_output_array_ref_forward,
		mod_prelude_sys::*,
		opencv_type_boxed, opencv_type_enum, opencv_type_simple, ptr_cast_base, ptr_extern, ptr_extern_ctor, tuple_extern,
		vector_copy_non_bool, vector_extern, vector_non_copy_or_bool, Error, Result,
	};
	pub use std::convert::TryFrom;
}

mod cond_macros;
#[cfg(test)]
mod test;
