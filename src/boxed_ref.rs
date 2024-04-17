use std::fmt;
use std::marker::PhantomData;

use crate::mod_prelude::OpenCVTypeExternContainer;
use crate::traits::{Boxed, OpenCVType, OpenCVTypeArg};

/// Wrapper for the type implementing [Boxed] trait that allows to retain the lifetime of the referenced object.
///
/// This wrapper implements all traits that the underlying type does, but explicitly doesn't implement `Deref` and `DerefMut` to
/// avoid being able to `mem::swap` the reference out of the wrapper.
pub struct BoxedRef<'r, T: Boxed> {
	pub(crate) reference: T,
	// can make it &'r T, but then it will just take up unnecessary space
	referenced_object: PhantomData<&'r T>,
}

impl<T: Boxed> From<T> for BoxedRef<'_, T> {
	fn from(value: T) -> Self {
		Self {
			reference: value,
			referenced_object: PhantomData,
		}
	}
}

impl<T: Boxed + fmt::Debug> fmt::Debug for BoxedRef<'_, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&self.reference, f)
	}
}

impl<T: Boxed + Clone> BoxedRef<'_, T> {
	/// Clones the pointee of this BoxedRef
	#[inline]
	pub fn clone_pointee(&self) -> T {
		self.reference.clone()
	}
}

impl<'t, T: OpenCVTypeArg<'t> + Boxed> OpenCVTypeArg<'t> for BoxedRef<'_, T> {
	type ExternContainer = T::ExternContainer;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self.reference.opencv_into_extern_container_nofail()
	}
}

impl<'t, T: OpenCVType<'t> + Boxed> OpenCVType<'t> for BoxedRef<'_, T> {
	type Arg = T::Arg;
	type ExternReceive = T::ExternReceive;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		Self {
			reference: T::opencv_from_extern(s),
			referenced_object: PhantomData,
		}
	}
}

impl<T: Boxed + OpenCVTypeExternContainer> OpenCVTypeExternContainer for BoxedRef<'_, T> {
	type ExternSend = T::ExternSend;
	type ExternSendMut = T::ExternSendMut;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.reference.opencv_as_extern()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		self.reference.opencv_as_extern_mut()
	}
}

/// Mutable version of [BoxedRef], implements the traits that take `&mut self`.
pub struct BoxedRefMut<'r, T: Boxed> {
	pub(crate) reference: T,
	referenced_object: PhantomData<&'r mut T>,
}

impl<T: Boxed> From<T> for BoxedRefMut<'_, T> {
	fn from(value: T) -> Self {
		Self {
			reference: value,
			referenced_object: PhantomData,
		}
	}
}

impl<T: Boxed + fmt::Debug> fmt::Debug for BoxedRefMut<'_, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&self.reference, f)
	}
}

impl<T: Boxed + Clone> BoxedRefMut<'_, T> {
	/// Clones the pointee of this BoxedRef
	#[inline]
	pub fn clone_pointee(&self) -> T {
		self.reference.clone()
	}
}

impl<'t, T: OpenCVTypeArg<'t> + Boxed> OpenCVTypeArg<'t> for BoxedRefMut<'_, T> {
	type ExternContainer = T::ExternContainer;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self.reference.opencv_into_extern_container_nofail()
	}
}

impl<'t, T: OpenCVType<'t> + Boxed> OpenCVType<'t> for BoxedRefMut<'_, T> {
	type Arg = T::Arg;
	type ExternReceive = T::ExternReceive;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		Self {
			reference: T::opencv_from_extern(s),
			referenced_object: PhantomData,
		}
	}
}

impl<T: Boxed + OpenCVTypeExternContainer> OpenCVTypeExternContainer for BoxedRefMut<'_, T> {
	type ExternSend = T::ExternSend;
	type ExternSendMut = T::ExternSendMut;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.reference.opencv_as_extern()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		self.reference.opencv_as_extern_mut()
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! boxed_ref {
	($typ: ty, $const_trait: ty, $const_method: ident, $mut_trait: ty, $mut_method: ident) => {
		impl $const_trait for $crate::boxed_ref::BoxedRef<'_, $typ> {
			#[inline]
			fn $const_method(&self) -> extern_send!(Self) {
				self.reference.$const_method()
			}
		}

		impl $const_trait for $crate::boxed_ref::BoxedRefMut<'_, $typ> {
			#[inline]
			fn $const_method(&self) -> extern_send!(Self) {
				self.reference.$const_method()
			}
		}

		impl $mut_trait for $crate::boxed_ref::BoxedRefMut<'_, $typ> {
			#[inline]
			fn $mut_method(&mut self) -> extern_send!(mut Self) {
				self.reference.$mut_method()
			}
		}
	};
}
