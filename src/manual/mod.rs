#[cfg(ocvrs_has_module_core)]
pub mod core;
#[cfg(ocvrs_has_module_dnn)]
pub mod dnn;
pub mod sys;
pub mod types;

pub mod prelude {
	pub use super::core::VectorToVec;
	#[cfg(ocvrs_has_module_core)]
	pub use super::core::{MatConstIteratorTraitManual, MatTraitConstManual, MatTraitManual, MatxTrait, ModifyInplace};
}
