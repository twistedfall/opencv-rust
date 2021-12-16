use std::{
	array,
	ops::{Deref, DerefMut, Mul},
};

use crate::traits::{OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};

mod operations;

valid_types!(ValidVecType: i8, u8, i16, u16, i32, f32, f64);

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

impl<T: ValidVecType, const N: usize> OpenCVType<'_> for VecN<T, N> {
	type Arg = Self;
	type ExternReceive = Self;
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }

	#[inline]
	unsafe fn opencv_from_extern(s: Self) -> Self { s }
}

impl<T: ValidVecType, const N: usize> OpenCVTypeArg<'_> for VecN<T, N> {
	type ExternContainer = Self;

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
