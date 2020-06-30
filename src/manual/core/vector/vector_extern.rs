use std::ffi::c_void;

use crate::{
	core::Vector,
	platform_types::size_t,
	Result,
	traits::{OpenCVType, OpenCVTypeExternContainer},
};

#[doc(hidden)]
pub trait VectorElement: Sized + for<'a> OpenCVType<'a> where Vector<Self>: VectorExtern<Self> {
	fn convert_to_vec(v: &Vector<Self>) -> Vec<Self>;
}

#[doc(hidden)]
pub trait VectorExtern<T: for<'a> OpenCVType<'a>> {
	unsafe fn extern_new() -> *mut c_void;
	unsafe fn extern_clone(&self) -> *mut c_void;
	unsafe fn extern_delete(&mut self);
	unsafe fn extern_len(&self) -> size_t;
	unsafe fn extern_is_empty(&self) -> bool;
	unsafe fn extern_capacity(&self) -> size_t;
	unsafe fn extern_shrink_to_fit(&mut self);
	unsafe fn extern_reserve(&mut self, additional: size_t);
	unsafe fn extern_remove(&mut self, index: size_t);
	unsafe fn extern_swap(&mut self, index1: size_t, index2: size_t);
	unsafe fn extern_clear(&mut self);
	unsafe fn extern_get(&self, index: size_t) -> Result<T>;
	unsafe fn extern_push<'a>(&mut self, val: <<<T as OpenCVType<'a>>::Arg as OpenCVType<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
	unsafe fn extern_insert<'a>(&mut self, index: size_t, val: <<<T as OpenCVType<'a>>::Arg as OpenCVType<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
	unsafe fn extern_set<'a>(&mut self, index: size_t, val: <<<T as OpenCVType<'a>>::Arg as OpenCVType<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSend);
}

#[macro_export]
macro_rules! vector_extern {
	(
		$type: ty,
		$vector_extern_const: ty,
		$vector_extern_mut: ty,
		$extern_new: ident,
		$extern_clone: ident,
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
		$extern_insert: ident,
	) => {
		impl $crate::manual::core::VectorExtern<$type> for $crate::manual::core::Vector<$type> {
			#[inline(always)]
			unsafe fn extern_new() -> $vector_extern_mut {
				extern "C" { fn $extern_new() -> $vector_extern_mut; }
				$extern_new()
			}

			#[inline(always)]
			unsafe fn extern_clone(&self) -> $vector_extern_mut {
				extern "C" { fn $extern_clone(instance: $vector_extern_const) -> $vector_extern_mut; }
				$extern_clone(self.as_raw())
			}

			#[inline(always)]
			unsafe fn extern_delete(&mut self) {
				extern "C" { fn $extern_delete(instance: $vector_extern_mut); }
				$extern_delete(self.as_raw_mut())
			}

			#[inline(always)]
			unsafe fn extern_len(&self) -> $crate::platform_types::size_t {
				extern "C" { fn $extern_len(instance: $vector_extern_const) -> $crate::platform_types::size_t; }
				$extern_len(self.as_raw())
			}

			#[inline(always)]
			unsafe fn extern_is_empty(&self) -> bool {
				extern "C" { fn $extern_is_empty(instance: $vector_extern_const) -> bool; }
				$extern_is_empty(self.as_raw())
			}

			#[inline(always)]
			unsafe fn extern_capacity(&self) -> $crate::platform_types::size_t {
				extern "C" { fn $extern_capacity(instance: $vector_extern_const) -> $crate::platform_types::size_t; }
				$extern_capacity(self.as_raw())
			}

			#[inline(always)]
			unsafe fn extern_shrink_to_fit(&mut self) {
				extern "C" { fn $extern_shrink_to_fit(instance: $vector_extern_mut); }
				$extern_shrink_to_fit(self.as_raw_mut())
			}

			#[inline(always)]
			unsafe fn extern_reserve(&mut self, additional: $crate::platform_types::size_t) {
				extern "C" { fn $extern_reserve(instance: $vector_extern_mut, additional: $crate::platform_types::size_t); }
				$extern_reserve(self.as_raw_mut(), additional)
			}

			#[inline(always)]
			unsafe fn extern_remove(&mut self, index: $crate::platform_types::size_t) {
				extern "C" { fn $extern_remove(instance: $vector_extern_mut, index: $crate::platform_types::size_t); }
				$extern_remove(self.as_raw_mut(), index)
			}

			#[inline(always)]
			unsafe fn extern_swap(&mut self, index1: $crate::platform_types::size_t, index2: $crate::platform_types::size_t) {
				extern "C" { fn $extern_swap(instance: $vector_extern_mut, index1: $crate::platform_types::size_t, index2: $crate::platform_types::size_t); }
				$extern_swap(self.as_raw_mut(), index1, index2)
			}

			#[inline(always)]
			unsafe fn extern_clear(&mut self) {
				extern "C" { fn $extern_clear(instance: $vector_extern_mut); }
				$extern_clear(self.as_raw_mut())
			}

			#[inline(always)]
			unsafe fn extern_get(&self, index: $crate::platform_types::size_t) -> $crate::Result<$type> {
				extern "C" { fn $extern_get<'a>(instance: $vector_extern_const, index: $crate::platform_types::size_t) -> $crate::sys::Result<<$type as $crate::traits::OpenCVType<'a>>::ExternReceive>; }
				$extern_get(self.as_raw(), index)
					.into_result()
					.map(|s| <$type>::opencv_from_extern(s))
			}

			#[inline(always)]
			unsafe fn extern_push<'a>(&mut self, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				extern "C" { fn $extern_push<'a>(instance: $vector_extern_mut, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend); }
				$extern_push(self.as_raw_mut(), val)
			}

			#[inline(always)]
			unsafe fn extern_insert<'a>(&mut self, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				extern "C" { fn $extern_insert<'a>(instance: $vector_extern_mut, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend); }
				$extern_insert(self.as_raw_mut(), index, val)
			}

			#[inline(always)]
			unsafe fn extern_set<'a>(&mut self, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend) {
				extern "C" { fn $extern_set<'a>(instance: $vector_extern_mut, index: $crate::platform_types::size_t, val: <<<$type as $crate::traits::OpenCVType<'a>>::Arg as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSend); }
				$extern_set(self.as_raw_mut(), index, val)
			}
		}
	};
}

#[doc(hidden)]
pub trait VectorExternCopyNonBool<T> {
	unsafe fn extern_data(&self) -> *const T;
}

#[macro_export]
macro_rules! vector_copy_non_bool {
	(
		$type: ty,
		$vector_extern_const: ty,
		$extern_data: ident
	) => {
		impl $crate::manual::core::VectorElement for $type where $crate::manual::core::Vector<$type>: $crate::manual::core::VectorExtern<$type> {
			#[inline(always)]
			fn convert_to_vec(v: &$crate::manual::core::Vector<$type>) -> std::vec::Vec<$type> {
				v.as_slice().to_vec()
			}
		}

		impl $crate::manual::core::VectorExternCopyNonBool<$type> for $crate::manual::core::Vector<$type> {
			#[inline(always)]
			unsafe fn extern_data(&self) -> *const $type {
				extern "C" { fn $extern_data(instance: $vector_extern_const) -> *const $type; }
				$extern_data(self.as_raw())
			}
		}
	};
}

#[macro_export]
macro_rules! vector_non_copy_or_bool {
	(
		$type: ty
	) => {
		impl $crate::manual::core::VectorElement for $type where $crate::manual::core::Vector<$type>: $crate::manual::core::VectorExtern<$type> {
			#[inline(always)]
			fn convert_to_vec(v: &$crate::manual::core::Vector<$type>) -> std::vec::Vec<$type> {
				(0..v.len()).map(|x| unsafe { v.get_unchecked(x) }).collect()
			}
		}
	};
}
