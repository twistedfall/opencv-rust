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
sized_array_impl!(SizedArray66, 6, 6);
