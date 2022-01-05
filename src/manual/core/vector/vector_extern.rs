use std::ffi::c_void;

use crate::{
	core::Vector,
	platform_types::size_t,
	traits::{OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
};

/// This trait is implemented by any type that can be stored inside `Vector`.
///
/// It is mostly used internally, feasibility of implementing it for your own types hasn't been
/// considered. Use is mostly for external type constraints.
pub trait VectorElement: for<'a> OpenCVType<'a> where Vector<Self>: VectorExtern<Self> {
	#[doc(hidden)] fn opencv_vector_to_vec(v: &Vector<Self>) -> Vec<Self>;
}

#[doc(hidden)]
pub trait VectorExtern<T: for<'a> OpenCVType<'a>> {
	#[doc(hidden)] unsafe fn extern_new() -> *mut c_void;
	#[doc(hidden)] unsafe fn extern_delete(&mut self);
	#[doc(hidden)] unsafe fn extern_len(&self) -> size_t;
	#[doc(hidden)] unsafe fn extern_is_empty(&self) -> bool;
	#[doc(hidden)] unsafe fn extern_capacity(&self) -> size_t;
	#[doc(hidden)] unsafe fn extern_shrink_to_fit(&mut self);
	#[doc(hidden)] unsafe fn extern_reserve(&mut self, additional: size_t);
	#[doc(hidden)] unsafe fn extern_remove(&mut self, index: size_t);
	#[doc(hidden)] unsafe fn extern_swap(&mut self, index1: size_t, index2: size_t);
	#[doc(hidden)] unsafe fn extern_clear(&mut self);
	#[doc(hidden)] unsafe fn extern_get(&self, index: size_t) -> <T as OpenCVType<'_>>::ExternReceive;
	#[doc(hidden)] unsafe fn extern_push<'a>(&mut self, val: <<<T as OpenCVType<'a>>::Arg as OpenCVTypeArg<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
	#[doc(hidden)] unsafe fn extern_push_owned(&mut self, val: <<T as OpenCVType>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
	#[doc(hidden)] unsafe fn extern_insert<'a>(&mut self, index: size_t, val: <<<T as OpenCVType<'a>>::Arg as OpenCVTypeArg<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
	#[doc(hidden)] unsafe fn extern_set<'a>(&mut self, index: size_t, val: <<<T as OpenCVType<'a>>::Arg as OpenCVTypeArg<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
}

#[macro_export]
macro_rules! vector_extern {
	(
		$type: ty,
		$vector_extern_const: ty,
		$vector_extern_mut: ty,
		$extern_new: ident,
		$extern_delete: ident,
		$extern_len: ident,
		$extern_is_empty: ident,
		$extern_capacity: ident,
		$extern_shrink_to_fit: ident,
		$extern_reserve: ident,
		$extern_remove: ident,
		$extern_swap: ident,
		$extern_clear: ident,
		$extern_get: ident,
		$extern_set: ident,
		$extern_push: ident,
		$extern_insert: ident $(,)?
	) => {
		extern "C" {
			fn $extern_new() -> $vector_extern_mut;
			fn $extern_delete(instance: $vector_extern_mut);
			fn $extern_len(instance: $vector_extern_const) -> $crate::platform_types::size_t;
			fn $extern_is_empty(instance: $vector_extern_const) -> bool;
			fn $extern_capacity(instance: $vector_extern_const) -> $crate::platform_types::size_t;
			fn $extern_shrink_to_fit(instance: $vector_extern_mut);
			fn $extern_reserve(instance: $vector_extern_mut, additional: $crate::platform_types::size_t);
			fn $extern_remove(instance: $vector_extern_mut, index: $crate::platform_types::size_t);
			fn $extern_swap(instance: $vector_extern_mut, index1: $crate::platform_types::size_t, index2: $crate::platform_types::size_t);
			fn $extern_clear(instance: $vector_extern_mut);
			fn $extern_get<'a>(instance: $vector_extern_const, index: $crate::platform_types::size_t, ocvrs_return: *mut <$type as $crate::traits::OpenCVType<'a>>::ExternReceive);
			fn $extern_push<'a>(instance: $vector_extern_mut, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVTypeArg<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend);
			fn $extern_insert<'a>(instance: $vector_extern_mut, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVTypeArg<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend);
			fn $extern_set<'a>(instance: $vector_extern_mut, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVTypeArg<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend);
		}

		impl $crate::manual::core::VectorExtern<$type> for $crate::manual::core::Vector<$type> {
			#[inline]
			unsafe fn extern_new() -> $vector_extern_mut {
				$extern_new()
			}

			#[inline]
			unsafe fn extern_delete(&mut self) {
				$extern_delete(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_len(&self) -> $crate::platform_types::size_t {
				$extern_len(self.as_raw())
			}

			#[inline]
			unsafe fn extern_is_empty(&self) -> bool {
				$extern_is_empty(self.as_raw())
			}

			#[inline]
			unsafe fn extern_capacity(&self) -> $crate::platform_types::size_t {
				$extern_capacity(self.as_raw())
			}

			#[inline]
			unsafe fn extern_shrink_to_fit(&mut self) {
				$extern_shrink_to_fit(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_reserve(&mut self, additional: $crate::platform_types::size_t) {
				$extern_reserve(self.as_raw_mut(), additional)
			}

			#[inline]
			unsafe fn extern_remove(&mut self, index: $crate::platform_types::size_t) {
				$extern_remove(self.as_raw_mut(), index)
			}

			#[inline]
			unsafe fn extern_swap(&mut self, index1: $crate::platform_types::size_t, index2: $crate::platform_types::size_t) {
				$extern_swap(self.as_raw_mut(), index1, index2)
			}

			#[inline]
			unsafe fn extern_clear(&mut self) {
				$extern_clear(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_get(&self, index: $crate::platform_types::size_t) -> <$type as $crate::traits::OpenCVType<'_>>::ExternReceive {
				return_send!(via ocvrs_return);
				$extern_get(self.as_raw(), index, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_push<'a>(&mut self, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVTypeArg<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				$extern_push(self.as_raw_mut(), val)
			}

			#[inline]
			unsafe fn extern_push_owned(&mut self, val: <<$type as $crate::traits::OpenCVType>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				$extern_push(self.as_raw_mut(), val)
			}

			#[inline]
			unsafe fn extern_insert<'a>(&mut self, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVTypeArg<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				$extern_insert(self.as_raw_mut(), index, val)
			}

			#[inline]
			unsafe fn extern_set<'a>(&mut self, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVTypeArg<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				$extern_set(self.as_raw_mut(), index, val)
			}
		}
	};
}

#[doc(hidden)]
pub trait VectorExternCopyNonBool<T> {
	#[doc(hidden)] unsafe fn extern_data(&self) -> *const T;
	#[doc(hidden)] unsafe fn extern_data_mut(&mut self) -> *mut T;
	#[doc(hidden)] unsafe fn extern_from_slice(data: *const T, len: size_t) -> *mut c_void;
}

#[macro_export]
macro_rules! vector_copy_non_bool {
	(
		$type: ty,
		$vector_extern_const: ty,
		$vector_extern_mut: ty,
		$extern_data_const: ident,
		$extern_data_mut: ident,
		$extern_from_slice: ident,
		$extern_clone: ident $(,)?
	) => {
		impl $crate::manual::core::Vector<$type> where $crate::manual::core::Vector<$type>: $crate::manual::core::VectorExtern<$type> {
			#[inline]
			unsafe fn extern_clone(&self) -> $vector_extern_mut {
				extern "C" { fn $extern_clone(instance: $vector_extern_const) -> $vector_extern_mut; }
				$extern_clone(self.as_raw())
			}
		}

		impl $crate::manual::core::VectorElement for $type where $crate::manual::core::Vector<$type>: $crate::manual::core::VectorExtern<$type> {
			#[inline]
			fn opencv_vector_to_vec(v: &$crate::manual::core::Vector<$type>) -> std::vec::Vec<$type> {
				v.as_slice().to_vec()
			}
		}

		impl std::clone::Clone for $crate::manual::core::Vector<$type> where Self: $crate::manual::core::VectorExtern<$type> {
			#[inline]
			fn clone(&self) -> Self {
				unsafe { Self::from_raw(self.extern_clone()) }
			}
		}

		impl $crate::manual::core::VectorExternCopyNonBool<$type> for $crate::manual::core::Vector<$type> {
			#[inline]
			unsafe fn extern_data(&self) -> *const $type {
				extern "C" { fn $extern_data_const(instance: $vector_extern_const) -> *const $type; }
				$extern_data_const(self.as_raw())
			}

			#[inline]
			unsafe fn extern_data_mut(&mut self) -> *mut $type {
				extern "C" { fn $extern_data_mut(instance: $vector_extern_mut) -> *mut $type; }
				$extern_data_mut(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_from_slice(data: *const $type, len: $crate::platform_types::size_t) -> $vector_extern_mut {
				extern "C" { fn $extern_from_slice(data: *const $type, len: $crate::platform_types::size_t) -> $vector_extern_mut; }
				$extern_from_slice(data, len)
			}
		}
	};
}

#[macro_export]
macro_rules! vector_non_copy_or_bool {
	(clone $type: ty) => {
		impl std::clone::Clone for $crate::manual::core::Vector<$type> where Self: $crate::manual::core::VectorExtern<$type> {
			#[inline]
			fn clone(&self) -> Self {
				let mut out = Self::with_capacity(self.capacity());
				self.iter()
					.for_each(|elem| out.push_owned(elem.clone()));
				out
			}
		}
		vector_non_copy_or_bool! { $type }
	};
	($type: ty) => {
		impl $crate::manual::core::VectorElement for $type where $crate::manual::core::Vector<$type>: $crate::manual::core::VectorExtern<$type> {
			#[inline]
			fn opencv_vector_to_vec(v: &$crate::manual::core::Vector<$type>) -> std::vec::Vec<$type> {
				(0..v.len()).map(|x| unsafe { v.get_unchecked(x) }).collect()
			}
		}
	}
}
