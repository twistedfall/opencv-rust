use crate::core::Vector;
use crate::platform_types::size_t;
use crate::traits::OpenCVType;
use crate::{extern_arg_send, extern_container_send, extern_receive};

#[doc(hidden)]
pub trait VectorExtern<T: for<'a> OpenCVType<'a>> {
	#[doc(hidden)]
	unsafe fn extern_new<'a>() -> extern_receive!(Self: 'a)
	where
		Self: OpenCVType<'a>;
	#[doc(hidden)]
	unsafe fn extern_delete(&mut self);
	#[doc(hidden)]
	unsafe fn extern_len(&self) -> size_t;
	#[doc(hidden)]
	unsafe fn extern_is_empty(&self) -> bool;
	#[doc(hidden)]
	unsafe fn extern_capacity(&self) -> size_t;
	#[doc(hidden)]
	unsafe fn extern_shrink_to_fit(&mut self);
	#[doc(hidden)]
	unsafe fn extern_reserve(&mut self, additional: size_t);
	#[doc(hidden)]
	unsafe fn extern_remove(&mut self, index: size_t);
	#[doc(hidden)]
	unsafe fn extern_swap(&mut self, index1: size_t, index2: size_t);
	#[doc(hidden)]
	unsafe fn extern_clear(&mut self);
	#[doc(hidden)]
	unsafe fn extern_get(&self, index: size_t) -> extern_receive!(T);
	#[doc(hidden)]
	unsafe fn extern_push<'a>(&mut self, val: extern_arg_send!(T: 'a));
	#[doc(hidden)]
	unsafe fn extern_push_owned(&mut self, val: extern_container_send!(T));
	#[doc(hidden)]
	unsafe fn extern_insert<'a>(&mut self, index: size_t, val: extern_arg_send!(T: 'a));
	#[doc(hidden)]
	unsafe fn extern_set<'a>(&mut self, index: size_t, val: extern_arg_send!(T: 'a));
}

#[doc(hidden)]
#[macro_export]
macro_rules! vector_extern {
	(
		$type: ty,
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
		impl $crate::core::VectorExtern<$type> for $crate::core::Vector<$type> {
			#[inline]
			unsafe fn extern_new<'a>() -> extern_receive!(Self: 'a) {
				$crate::sys::$extern_new()
			}

			#[inline]
			unsafe fn extern_delete(&mut self) {
				$crate::sys::$extern_delete(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_len(&self) -> $crate::platform_types::size_t {
				$crate::sys::$extern_len(self.as_raw())
			}

			#[inline]
			unsafe fn extern_is_empty(&self) -> bool {
				$crate::sys::$extern_is_empty(self.as_raw())
			}

			#[inline]
			unsafe fn extern_capacity(&self) -> $crate::platform_types::size_t {
				$crate::sys::$extern_capacity(self.as_raw())
			}

			#[inline]
			unsafe fn extern_shrink_to_fit(&mut self) {
				$crate::sys::$extern_shrink_to_fit(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_reserve(&mut self, additional: $crate::platform_types::size_t) {
				$crate::sys::$extern_reserve(self.as_raw_mut(), additional)
			}

			#[inline]
			unsafe fn extern_remove(&mut self, index: $crate::platform_types::size_t) {
				$crate::sys::$extern_remove(self.as_raw_mut(), index)
			}

			#[inline]
			unsafe fn extern_swap(&mut self, index1: $crate::platform_types::size_t, index2: $crate::platform_types::size_t) {
				$crate::sys::$extern_swap(self.as_raw_mut(), index1, index2)
			}

			#[inline]
			unsafe fn extern_clear(&mut self) {
				$crate::sys::$extern_clear(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_get(&self, index: $crate::platform_types::size_t) -> extern_receive!($type) {
				return_send!(via ocvrs_return);
				$crate::sys::$extern_get(self.as_raw(), index, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_push<'a>(&mut self, val: extern_arg_send!($type: 'a)) {
				$crate::sys::$extern_push(self.as_raw_mut(), val)
			}

			#[inline]
			unsafe fn extern_push_owned(&mut self, val: extern_container_send!($type)) {
				$crate::sys::$extern_push(self.as_raw_mut(), val)
			}

			#[inline]
			unsafe fn extern_insert<'a>(&mut self, index: $crate::platform_types::size_t, val: extern_arg_send!($type: 'a)) {
				$crate::sys::$extern_insert(self.as_raw_mut(), index, val)
			}

			#[inline]
			unsafe fn extern_set<'a>(&mut self, index: $crate::platform_types::size_t, val: extern_arg_send!($type: 'a)) {
				$crate::sys::$extern_set(self.as_raw_mut(), index, val)
			}
		}
	};
}

#[doc(hidden)]
pub trait VectorExternCopyNonBool<T: for<'o> OpenCVType<'o>>
where
	Vector<T>: VectorExtern<T>,
{
	#[doc(hidden)]
	unsafe fn extern_data(&self) -> *const T;
	#[doc(hidden)]
	unsafe fn extern_data_mut(&mut self) -> *mut T;
	#[doc(hidden)]
	unsafe fn extern_from_slice<'a>(data: *const T, len: size_t) -> extern_receive!(Vector<T>: 'a);
}

#[doc(hidden)]
#[macro_export]
macro_rules! vector_copy_non_bool {
	(
		$type: ty,
		$extern_data_const: ident,
		$extern_data_mut: ident,
		$extern_from_slice: ident,
		$extern_clone: ident $(,)?
	) => {
		impl $crate::core::VectorToVec for $crate::core::Vector<$type> {
			type Element = $type;

			#[inline]
			fn to_vec(&self) -> std::vec::Vec<Self::Element> {
				self.as_slice().to_vec()
			}
		}

		impl std::clone::Clone for $crate::core::Vector<$type>
		where
			Self: $crate::core::VectorExtern<$type>,
		{
			#[inline]
			fn clone(&self) -> Self {
				unsafe { Self::opencv_from_extern($crate::sys::$extern_clone(self.as_raw())) }
			}
		}

		impl $crate::core::VectorExternCopyNonBool<$type> for $crate::core::Vector<$type> {
			#[inline]
			unsafe fn extern_data(&self) -> *const $type {
				$crate::sys::$extern_data_const(self.as_raw())
			}

			#[inline]
			unsafe fn extern_data_mut(&mut self) -> *mut $type {
				$crate::sys::$extern_data_mut(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_from_slice<'a>(data: *const $type, len: $crate::platform_types::size_t) -> extern_receive!(Self: 'a) {
				$crate::sys::$extern_from_slice(data, len)
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vector_non_copy_or_bool {
	(clone $type: ty) => {
		impl std::clone::Clone for $crate::core::Vector<$type>
		where
			Self: $crate::core::VectorExtern<$type>,
		{
			#[inline]
			fn clone(&self) -> Self {
				let mut out = Self::with_capacity(self.capacity());
				self.iter().for_each(|elem| out.push_owned(elem.clone()));
				out
			}
		}
		vector_non_copy_or_bool! { $type }
	};
	($type: ty) => {
		impl $crate::core::VectorToVec for $crate::core::Vector<$type> {
			type Element = $type;

			#[inline]
			fn to_vec(&self) -> std::vec::Vec<Self::Element> {
				(0..self.len()).map(|x| unsafe { self.get_unchecked(x) }).collect()
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vector_boxed_ref {
	($type: ty) => {
		impl<'b> $crate::core::VectorToVec for $crate::core::Vector<$crate::boxed_ref::BoxedRef<'b, $type>> {
			type Element = $crate::boxed_ref::BoxedRef<'b, $type>;

			#[inline]
			fn to_vec(&self) -> std::vec::Vec<Self::Element> {
				(0..self.len()).map(|x| unsafe { self.get_unchecked(x) }).collect()
			}
		}
	};
}
