use num_traits::Zero;

use super::VecN;

/// [docs.opencv.org](https://docs.opencv.org/4.x/d1/da0/classcv_1_1Scalar__.html)
pub type Scalar_<T> = VecN<T, 4>;

impl<T> Scalar_<T> {
	#[inline]
	pub const fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self::from_array([v0, v1, v2, v3])
	}

	/// returns true iff v1 == v2 == v3 == 0
	#[inline]
	pub fn is_real(&self) -> bool
	where
		T: Zero + PartialEq,
	{
		self[1] == T::zero() && self[2] == T::zero() && self[3] == T::zero()
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
		Self::from_array([v0, T::zero(), T::zero(), T::zero()])
	}
}

impl<T: Zero> From<(T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T)) -> Self {
		Self::from_array([v.0, v.1, T::zero(), T::zero()])
	}
}

impl<T: Zero> From<(T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T)) -> Self {
		Self::from_array([v.0, v.1, v.2, T::zero()])
	}
}

impl<T> From<(T, T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T, T)) -> Self {
		Self::from_array([v.0, v.1, v.2, v.3])
	}
}
