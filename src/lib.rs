#![allow(broken_intra_doc_links)]

pub use error::{Error, Result};

pub use crate::opencv::hub::*;

#[macro_use]
mod templ;

pub mod error;
mod manual;
mod opencv;
pub mod traits;

pub mod prelude {
	#[cfg(ocvrs_has_module_core)]
	pub use crate::core::{DataType, Mat};
	pub use crate::hub_prelude::*;
	pub use crate::manual::prelude::*;
	pub use crate::traits::Boxed;
}

/// Reexported platform types that are used by OpenCV
pub mod platform_types {
	pub use libc::{clock_t, ptrdiff_t, size_t, FILE};
}

/// Prelude for sys (externs) module and types
pub mod mod_prelude_sys {
	pub use std::ffi::{c_char, c_void};

	pub use crate::platform_types::*;
	pub use crate::traits::{Boxed, OpenCVFromExtern, OpenCVIntoExternContainer, OpenCVTypeExternContainer};
}

/// Prelude for generated modules and types
pub mod mod_prelude {
	pub use crate::boxed_ref::{BoxedRef, BoxedRefMut};
	pub use crate::core::{ToInputArray, ToInputOutputArray, ToOutputArray};
	pub use crate::hub_prelude::*;
	pub use crate::mod_prelude_sys::*;
	pub use crate::{
		boxed_cast_base, boxed_cast_descendant, boxed_ref, extern_arg_send, extern_container_send, extern_receive, extern_send,
		input_array_ref_forward, opencv_type_boxed, opencv_type_enum, opencv_type_simple, output_array_ref_forward, ptr_cast_base,
		ptr_extern, ptr_extern_ctor, tuple_extern, vector_boxed_ref, vector_copy_non_bool, vector_extern, vector_non_copy_or_bool,
		Result,
	};
}

pub mod boxed_ref;
mod cond_macros;
#[cfg(test)]
mod test;
