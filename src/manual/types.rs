pub use abstract_ref::*;

mod abstract_ref;

#[doc(hidden)]
#[repr(C)]
/// Needed to prevent the following error:
/// `extern` block uses type `()`, which is not FFI-safe
/// That warning is not triggered in rust 1.74.0, so drop when MSRV is high enough
pub struct Unit([u8; 0]);

impl From<Unit> for () {
	fn from(_: Unit) -> Self {}
}
