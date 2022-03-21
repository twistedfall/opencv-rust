use num_traits::Zero;

use super::VecN;

pub type Scalar_<T> = VecN<T, 4>;

impl<T> Scalar_<T> {
	#[inline]
	pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self::from([v0, v1, v2, v3])
	}
}

impl From<i32> for Scalar_<f64> {
	#[inline]
	fn from(v0: i32) -> Self {
		Self::from(f64::from(v0))
	}
}

impl<T: Zero> From<T> for Scalar_<T> {
	#[inline]
	fn from(v0: T) -> Self {
		Self::from([v0, T::zero(), T::zero(), T::zero()])
	}
}

impl<T: Zero> From<(T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T)) -> Self {
		Self::from([v.0, v.1, T::zero(), T::zero()])
	}
}

impl<T: Zero> From<(T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T)) -> Self {
		Self::from([v.0, v.1, v.2, T::zero()])
	}
}

impl<T> From<(T, T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T, T)) -> Self {
		Self::from([v.0, v.1, v.2, v.3])
	}
}
