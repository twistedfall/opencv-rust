pub mod core;
#[cfg(any(not(feature = "opencv-32"), feature = "contrib"))]
pub mod dnn;
pub mod features2d;
pub mod sys;
pub mod types;

pub mod prelude {
	pub use super::core::{Boxed, MatConstIteratorTraitManual, MatTraitManual, MatxTrait, UMatTraitManual};
	#[cfg(feature = "opencv-32")]
	pub use super::core::MatSizeTraitManual;
}
