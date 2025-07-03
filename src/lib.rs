#![allow(rustdoc::broken_intra_doc_links)]
// MSRV: remove this hint when edition is "2024" because it's included in it
#![deny(unsafe_op_in_unsafe_fn)]
//! Rust bindings for the OpenCV computer vision library
//!
//! [Git](https://github.com/twistedfall/opencv-rust) | [Readme](https://github.com/twistedfall/opencv-rust/blob/master/README.md) |
//! [Docs](https://docs.rs/opencv) | [Sponsor](https://github.com/sponsors/twistedfall)
//!
//! Please note that this documentation is automatically generated from the C++ headers. So expect references to C++ items,
//! code examples in C++ and other strange things.
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
	pub use crate::traits::{Boxed, OpenCVBitfieldEnum};
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
	pub use std::marker::PhantomData;

	pub use crate::boxed_ref::{BoxedRef, BoxedRefMut};
	pub use crate::core::{ToInputArray, ToInputOutputArray, ToOutputArray};
	pub use crate::hub_prelude::*;
	pub use crate::mod_prelude_sys::*;
	pub use crate::traits::OpenCVBitfieldEnum;
	pub use crate::{
		boxed_cast_base, boxed_cast_descendant, boxed_ref, extern_arg_send, extern_container_send, extern_receive, extern_send,
		input_array_ref_forward, opencv_type_bitfield_enum, opencv_type_boxed, opencv_type_enum, opencv_type_simple,
		output_array_ref_forward, ptr_cast_base, ptr_extern, ptr_extern_ctor, tuple_extern, vector_boxed_ref, vector_copy_non_bool,
		vector_extern, vector_non_copy_or_bool, Result,
	};
}

pub mod boxed_ref;
