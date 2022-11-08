use crate::core::Vector;
use crate::platform_types::size_t;
use crate::traits::{OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};
use crate::{extern_arg_send, extern_container_send, extern_receive, extern_send};

/// This trait is implemented by any type that can be stored inside `Vector`.
///
/// It is mostly used internally, feasibility of implementing it for your own types hasn't been
/// considered. Use is mostly for external type constraints.
pub trait VectorElement: for<'a> OpenCVType<'a>
where
	Vector<Self>: VectorExtern<Self>,
{
	#[doc(hidden)]
	fn opencv_vector_to_vec(v: &Vector<Self>) -> Vec<Self>;
}

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
	unsafe fn extern_push_owned<'a>(&mut self, val: extern_container_send!(T: 'a));
	#[doc(hidden)]
	unsafe fn extern_insert<'a>(&mut self, index: size_t, val: extern_arg_send!(T: 'a));
	#[doc(hidden)]
	unsafe fn extern_set<'a>(&mut self, index: size_t, val: extern_arg_send!(T: 'a));
}

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
		extern "C" {
			fn $extern_new<'a>() -> extern_receive!($crate::core::Vector<$type>: 'a);
			fn $extern_delete(instance: extern_send!(mut $crate::core::Vector<$type>));
			fn $extern_len(instance: extern_send!($crate::core::Vector<$type>)) -> $crate::platform_types::size_t;
			fn $extern_is_empty(instance: extern_send!($crate::core::Vector<$type>)) -> bool;
			fn $extern_capacity(instance: extern_send!($crate::core::Vector<$type>)) -> $crate::platform_types::size_t;
			fn $extern_shrink_to_fit(instance: extern_send!(mut $crate::core::Vector<$type>));
			fn $extern_reserve(instance: extern_send!(mut $crate::core::Vector<$type>), additional: $crate::platform_types::size_t);
			fn $extern_remove(instance: extern_send!(mut $crate::core::Vector<$type>), index: $crate::platform_types::size_t);
			fn $extern_swap(instance: extern_send!(mut $crate::core::Vector<$type>), index1: $crate::platform_types::size_t, index2: $crate::platform_types::size_t);
			fn $extern_clear(instance: extern_send!(mut $crate::core::Vector<$type>));
			fn $extern_get<'a>(instance: extern_send!($crate::core::Vector<$type>), index: $crate::platform_types::size_t, ocvrs_return: *mut extern_receive!($type: 'a));
			fn $extern_push<'a>(instance: extern_send!(mut $crate::core::Vector<$type>), val: extern_arg_send!($type: 'a));
			fn $extern_insert<'a>(instance: extern_send!(mut $crate::core::Vector<$type>), index: $crate::platform_types::size_t, val: extern_arg_send!($type: 'a));
			fn $extern_set<'a>(instance: extern_send!(mut $crate::core::Vector<$type>), index: $crate::platform_types::size_t, val: extern_arg_send!($type: 'a));
		}

		impl $crate::core::VectorExtern<$type> for $crate::core::Vector<$type> {
			#[inline]
			unsafe fn extern_new<'a>() -> extern_receive!(Self: 'a) {
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
			unsafe fn extern_get(&self, index: $crate::platform_types::size_t) -> extern_receive!($type) {
				return_send!(via ocvrs_return);
				$extern_get(self.as_raw(), index, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_push<'a>(&mut self, val: extern_arg_send!($type: 'a)) {
				$extern_push(self.as_raw_mut(), val)
			}

			#[inline]
			unsafe fn extern_push_owned<'a>(&mut self, val: extern_container_send!($type: 'a)) {
				$extern_push(self.as_raw_mut(), val)
			}

			#[inline]
			unsafe fn extern_insert<'a>(&mut self, index: $crate::platform_types::size_t, val: extern_arg_send!($type: 'a)) {
				$extern_insert(self.as_raw_mut(), index, val)
			}

			#[inline]
			unsafe fn extern_set<'a>(&mut self, index: $crate::platform_types::size_t, val: extern_arg_send!($type: 'a)) {
				$extern_set(self.as_raw_mut(), index, val)
			}
		}
	};
}

#[doc(hidden)]
pub trait VectorExternCopyNonBool<T: VectorElement>
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

#[macro_export]
macro_rules! vector_copy_non_bool {
	(
		$type: ty,
		$extern_data_const: ident,
		$extern_data_mut: ident,
		$extern_from_slice: ident,
		$extern_clone: ident $(,)?
	) => {
		extern "C" {
			fn $extern_clone<'a>(
				instance: extern_send!($crate::core::Vector<$type>),
	) -> extern_receive!($crate::core::Vector<$type>: 'a);
			fn $extern_data_const(instance: extern_send!($crate::core::Vector<$type>)) -> *const $type;
	fn $extern_data_mut(instance: extern_send!(mut $crate::core::Vector<$type>)) -> *mut $type;
			fn $extern_from_slice<'a>(
				data: *const $type,
				len: $crate::platform_types::size_t,
	) -> extern_receive!($crate::core::Vector<$type>: 'a);
		}

		impl $crate::core::Vector<$type>
		where
			$crate::core::Vector<$type>: $crate::core::VectorExtern<$type>,
		{
			#[inline]
			unsafe fn extern_clone(&self) -> extern_receive!(Self) {
				$extern_clone(self.as_raw())
			}
		}

		impl $crate::core::VectorElement for $type
		where
			$crate::core::Vector<$type>: $crate::core::VectorExtern<$type>,
		{
			#[inline]
			fn opencv_vector_to_vec(v: &$crate::core::Vector<$type>) -> std::vec::Vec<$type> {
				v.as_slice().to_vec()
			}
		}

		impl std::clone::Clone for $crate::core::Vector<$type>
		where
			Self: $crate::core::VectorExtern<$type>,
		{
			#[inline]
			fn clone(&self) -> Self {
				unsafe { Self::opencv_from_extern(self.extern_clone()) }
			}
		}

		impl $crate::core::VectorExternCopyNonBool<$type> for $crate::core::Vector<$type> {
			#[inline]
			unsafe fn extern_data(&self) -> *const $type {
				$extern_data_const(self.as_raw())
			}

			#[inline]
			unsafe fn extern_data_mut(&mut self) -> *mut $type {
				$extern_data_mut(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_from_slice<'a>(data: *const $type, len: $crate::platform_types::size_t) -> extern_receive!(Self: 'a) {
				$extern_from_slice(data, len)
			}
		}
	};
}

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
		impl $crate::core::VectorElement for $type
		where
			$crate::core::Vector<$type>: $crate::core::VectorExtern<$type>,
		{
			#[inline]
			fn opencv_vector_to_vec(v: &$crate::core::Vector<$type>) -> std::vec::Vec<$type> {
				(0..v.len()).map(|x| unsafe { v.get_unchecked(x) }).collect()
			}
		}
	};
}
