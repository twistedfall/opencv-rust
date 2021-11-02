use std::{
	array,
	ffi::c_void,
	ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
	core::{_InputArray, ToInputArray},
	input_array_ref_forward,
	Result,
	sys,
	traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
};

pub use self::{scalar_inner::ValidScalarType, vec_inner::ValidVecType};

// additional modules needed because valid_types! introduces module named "private"
mod vec_inner {
	valid_types!(ValidVecType: i8, u8, i16, u16, i32, f32, f64);
}

mod scalar_inner {
	valid_types!(ValidScalarType: i32, f64);
}

/// [docs.opencv.org](https://docs.opencv.org/master/d6/dcf/classcv_1_1Vec.html)
/// Named `VecN` to avoid name clash with std's `Vec`.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct VecN<T, const N: usize> (pub [T; N]);

impl<T, const N: usize> Default for VecN<T, N>
	where
		[T; N]: Default
{
	#[inline]
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T: Copy, const N: usize> VecN<T, N> {
	#[inline]
	pub fn all(v0: T) -> Self {
		Self::from([v0; N])
	}

	/// per-element multiplication
	#[inline]
	pub fn mul(&self, v: Self) -> Self where T: Mul<Output=T> + Copy {
		let mut out = *self;
		out.iter_mut()
			.zip(v.into_iter())
			.for_each(|(dest, m)| *dest = *dest * m);
		out
	}
}

impl<T, const N: usize> From<[T; N]> for VecN<T, N> {
	#[inline]
	fn from(s: [T; N]) -> Self {
		Self(s)
	}
}

impl<T, const N: usize> Deref for VecN<T, N> {
	type Target = [T; N];

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, const N: usize> DerefMut for VecN<T, N> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T, const N: usize> IntoIterator for VecN<T, N> {
	type Item = T;
	type IntoIter = array::IntoIter<T, N>;

	#[inline]
	fn into_iter(self) -> array::IntoIter<T, N> {
		IntoIterator::into_iter(self.0)
	}
}

impl<F: num_traits::Float> VecN<F, 2> {
	/// conjugation (makes sense for complex numbers and quaternions)
	#[inline]
	pub fn conj(&self) -> Self {
		Self([self[0], -self[1]])
	}
}

impl<F: num_traits::Float> VecN<F, 3> {
	/// cross product of the two 3D vectors
	#[inline]
	pub fn cross(&self, v: Self) -> Self {
		Self([
			self[1] * v[2] - self[2] * v[1],
			self[2] * v[0] - self[0] * v[2],
			self[0] * v[1] - self[1] * v[0]
		])
	}
}

impl<F: num_traits::Float> VecN<F, 4> {
	/// conjugation (makes sense for complex numbers and quaternions)
	#[inline]
	pub fn conj(&self) -> Self {
		Self([self[0], -self[1], -self[2], -self[3]])
	}
}

impl<T: AddAssign, const N: usize> AddAssign for VecN<T, N> {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.iter_mut()
			.zip(rhs.into_iter())
			.for_each(|(out, v)| *out += v)
	}
}

impl<T: Add<Output=T> + Copy, const N: usize> Add for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn add(mut self, rhs: Self) -> Self::Output {
		self.iter_mut()
			.zip(rhs.into_iter())
			.for_each(|(out, v)| *out = *out + v);
		self
	}
}

impl<T: SubAssign, const N: usize> SubAssign for VecN<T, N> {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.iter_mut()
			.zip(rhs.into_iter())
			.for_each(|(out, v)| *out -= v)
	}
}

impl<T: Sub<Output=T> + Copy, const N: usize> Sub for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn sub(mut self, rhs: Self) -> Self::Output {
		self.iter_mut()
			.zip(rhs.into_iter())
			.for_each(|(out, v)| *out = *out - v);
		self
	}
}

impl<Rhs: ValidVecType, T: MulAssign<Rhs>, const N: usize> MulAssign<Rhs> for VecN<T, N> {
	#[inline]
	fn mul_assign(&mut self, rhs: Rhs) {
		self.iter_mut()
			.for_each(|out| *out *= rhs)
	}
}

impl<Rhs: ValidVecType, T: DivAssign<Rhs>, const N: usize> DivAssign<Rhs> for VecN<T, N> {
	#[inline]
	fn div_assign(&mut self, rhs: Rhs) {
		self.iter_mut()
			.for_each(|out| *out /= rhs)
	}
}

impl<Rhs: ValidVecType, T: Mul<Rhs, Output=T> + Copy, const N: usize> Mul<Rhs> for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn mul(mut self, rhs: Rhs) -> Self::Output {
		self.iter_mut()
			.for_each(|out| *out = *out * rhs);
		self
	}
}

impl<Rhs: ValidVecType, T: Div<Rhs, Output=T> + Copy, const N: usize> Div<Rhs> for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn div(mut self, rhs: Rhs) -> Self::Output {
		self.iter_mut()
			.for_each(|out| *out = *out / rhs);
		self
	}
}

impl<T: Neg<Output=T> + Copy, const N: usize> Neg for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn neg(mut self) -> Self::Output {
		self.iter_mut()
			.for_each(|out| *out = -*out);
		self
	}
}

impl<T: Mul<Output=T> + Sub<Output=T> + Add<Output=T> + Copy> Mul for VecN<T, 4> {
	type Output = Self;

	#[inline]
	fn mul(self, rhs: Self) -> Self::Output {
		Self([
			self[0] * rhs[0] - self[1] * rhs[1] - self[2] * rhs[2] - self[3] * rhs[3],
			self[0] * rhs[1] + self[1] * rhs[0] + self[2] * rhs[3] - self[3] * rhs[2],
			self[0] * rhs[2] - self[1] * rhs[3] + self[2] * rhs[0] + self[3] * rhs[1],
			self[0] * rhs[3] + self[1] * rhs[2] - self[2] * rhs[1] + self[3] * rhs[0],
		])
	}
}

impl<T: Mul<Output=T> + Sub<Output=T> + Add<Output=T> + Copy> MulAssign for VecN<T, 4> {
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl<T: ValidVecType, const N: usize> OpenCVType<'_> for VecN<T, N> {
	type Arg = Self;
	type ExternReceive = Self;
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self> { Ok(self) }

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }

	#[inline]
	unsafe fn opencv_from_extern(s: Self) -> Self { s }
}

impl<T: ValidVecType, const N: usize> OpenCVTypeArg<'_> for VecN<T, N> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self> { Ok(self) }

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }
}

impl<T: ValidVecType, const N: usize> OpenCVTypeExternContainer for VecN<T, N> {
	type ExternSend = *const Self;
	type ExternSendMut = *mut Self;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend { self }

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { self }

	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut { &mut *std::mem::ManuallyDrop::new(self) as _ }
}

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

impl ToInputArray for Scalar_<f64> {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		extern "C" { fn cv_Scalar_input_array(instance: *const Scalar_<f64>) -> sys::Result<*mut c_void>; }
		unsafe { cv_Scalar_input_array(self) }
			.into_result()
			.map(|ptr| unsafe { _InputArray::from_raw(ptr) })
	}
}

input_array_ref_forward! { Scalar_<f64> }
