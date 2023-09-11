use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::Num;

use super::VecN;

impl<T: AddAssign, const N: usize> AddAssign for VecN<T, N> {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.iter_mut().zip(rhs).for_each(|(out, v)| *out += v)
	}
}

impl<T: Add<Output = T> + Copy, const N: usize> Add for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn add(mut self, rhs: Self) -> Self::Output {
		self.iter_mut().zip(rhs).for_each(|(out, v)| *out = *out + v);
		self
	}
}

impl<T: SubAssign, const N: usize> SubAssign for VecN<T, N> {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.iter_mut().zip(rhs).for_each(|(out, v)| *out -= v)
	}
}

impl<T: Sub<Output = T> + Copy, const N: usize> Sub for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn sub(mut self, rhs: Self) -> Self::Output {
		self.iter_mut().zip(rhs).for_each(|(out, v)| *out = *out - v);
		self
	}
}

impl<Rhs: Num + Copy, T: MulAssign<Rhs>, const N: usize> MulAssign<Rhs> for VecN<T, N> {
	#[inline]
	fn mul_assign(&mut self, rhs: Rhs) {
		self.iter_mut().for_each(|out| *out *= rhs)
	}
}

impl<Rhs: Copy, T: DivAssign<Rhs>, const N: usize> DivAssign<Rhs> for VecN<T, N> {
	#[inline]
	fn div_assign(&mut self, rhs: Rhs) {
		self.iter_mut().for_each(|out| *out /= rhs)
	}
}

impl<Rhs: Num + Copy, T: Mul<Rhs, Output = T> + Copy, const N: usize> Mul<Rhs> for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn mul(mut self, rhs: Rhs) -> Self::Output {
		self.iter_mut().for_each(|out| *out = *out * rhs);
		self
	}
}

impl<Rhs: Copy, T: Div<Rhs, Output = T> + Copy, const N: usize> Div<Rhs> for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn div(mut self, rhs: Rhs) -> Self::Output {
		self.iter_mut().for_each(|out| *out = *out / rhs);
		self
	}
}

impl<T: Neg<Output = T> + Copy, const N: usize> Neg for VecN<T, N> {
	type Output = Self;

	#[inline]
	fn neg(mut self) -> Self::Output {
		self.iter_mut().for_each(|out| *out = -*out);
		self
	}
}

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy> Mul for VecN<T, 4> {
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

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy> MulAssign for VecN<T, 4> {
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}
