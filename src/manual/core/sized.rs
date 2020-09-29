use std::ops::{Deref, DerefMut};

pub trait SizedArray<T>: Copy + Clone {
	type Storage: AsRef<[T]> + AsMut<[T]> + Copy + Clone;

	const ROWS: usize;
	const COLS: usize;
	fn all(alpha: T) -> Self::Storage;
}

macro_rules! sized_array_impl {
	($type: ident, $rows: expr, $cols: expr) => {
		#[derive(Copy, Clone)]
		pub struct $type;

		impl<T: Copy> $crate::core::SizedArray<T> for $type {
			type Storage = [T; $rows * $cols];

			const ROWS: usize = $rows;
			const COLS: usize = $cols;

			fn all(alpha: T) -> Self::Storage {
				[alpha; $rows * $cols]
			}
		}
	};
}

sized_array_impl!(SizedArray12, 1, 2);
sized_array_impl!(SizedArray13, 1, 3);
sized_array_impl!(SizedArray14, 1, 4);
sized_array_impl!(SizedArray16, 1, 6);

sized_array_impl!(SizedArray21, 2, 1);
sized_array_impl!(SizedArray31, 3, 1);
sized_array_impl!(SizedArray41, 4, 1);
sized_array_impl!(SizedArray61, 6, 1);

sized_array_impl!(SizedArray22, 2, 2);
sized_array_impl!(SizedArray23, 2, 3);
sized_array_impl!(SizedArray32, 3, 2);

sized_array_impl!(SizedArray33, 3, 3);

sized_array_impl!(SizedArray34, 3, 4);
sized_array_impl!(SizedArray43, 4, 3);

sized_array_impl!(SizedArray44, 4, 4);

// fixme workaround for missing standard derives in stdlib for arrays with len > 32
// in stable since 1.47 replace until "endfixme" with ```sized_array_impl!(SizedArray66, 6, 6);``` around rust 1.49
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Array66<T>([T; 6 * 6]);

impl<T> AsRef<[T]> for Array66<T> {
	fn as_ref(&self) -> &[T] {
		self.0.as_ref()
	}
}

impl<T> AsMut<[T]> for Array66<T> {
	fn as_mut(&mut self) -> &mut [T] {
		self.0.as_mut()
	}
}

impl<T> Deref for Array66<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.as_ref()
	}
}

impl<T> DerefMut for Array66<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut()
	}
}

#[derive(Copy, Clone)]
pub struct SizedArray66;

impl<T: Copy> SizedArray<T> for SizedArray66 {
	type Storage = Array66<T>;

	const ROWS: usize = 6;
	const COLS: usize = 6;

	fn all(alpha: T) -> Self::Storage {
		Array66([alpha; 6 * 6])
	}
}
// endfixme
