#[cfg(ocvrs_has_module_core)]
pub mod core;
#[cfg(ocvrs_has_module_dnn)]
pub mod dnn;
pub mod sys;
pub mod types;

pub mod prelude {
	#[cfg(all(ocvrs_has_module_core, ocvrs_opencv_branch_32))]
	pub use super::core::MatSizeTraitConstManual;
	#[cfg(ocvrs_has_module_core)]
	pub use super::core::{MatConstIteratorTraitManual, MatTraitConstManual, MatTraitManual, MatxTrait};
}
