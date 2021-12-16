use super::VecN;

valid_types!(ValidScalarType: i32, f64);

pub type Scalar_<T> = VecN<T, 4>;

impl<T: ValidScalarType> Scalar_<T> {
	#[inline]
	pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self::from([v0, v1, v2, v3])
	}
}

impl<T: ValidScalarType> From<T> for Scalar_<T> {
	#[inline]
	fn from(v0: T) -> Self {
		Self::from([v0, T::zero(), T::zero(), T::zero()])
	}
}

impl<T: ValidScalarType> From<(T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T)) -> Self {
		Self::from([v.0, v.1, T::zero(), T::zero()])
	}
}

impl<T: ValidScalarType> From<(T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T)) -> Self {
		Self::from([v.0, v.1, v.2, T::zero()])
	}
}

impl<T: ValidScalarType> From<(T, T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T, T)) -> Self {
		Self::from([v.0, v.1, v.2, v.3])
	}
}
