use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{NumCast, ToPrimitive, Zero};

use crate::{core::Point_, opencv_type_simple_generic};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/d6/d50/classcv_1_1Size__.html)
pub struct Size_<T> {
	pub width: T,
	pub height: T,
}

impl<T> Size_<T> {
	#[inline]
	pub const fn new(width: T, height: T) -> Self {
		Self { width, height }
	}

	#[inline]
	pub fn from_point(pt: Point_<T>) -> Self {
		Self {
			width: pt.x,
			height: pt.y,
		}
	}

	#[inline]
	pub fn area(self) -> T
	where
		T: Mul<Output = T>,
	{
		self.width * self.height
	}

	#[inline]
	pub fn empty(self) -> bool
	where
		T: PartialOrd + Zero,
	{
		self.width <= T::zero() || self.height <= T::zero()
	}

	#[inline]
	pub fn to<D: NumCast>(self) -> Option<Size_<D>>
	where
		T: ToPrimitive,
	{
		Some(Size_ {
			width: D::from(self.width)?,
			height: D::from(self.height)?,
		})
	}
}

impl<T> From<(T, T)> for Size_<T> {
	#[inline]
	fn from(s: (T, T)) -> Self {
		Self::new(s.0, s.1)
	}
}

impl<T> From<Point_<T>> for Size_<T> {
	#[inline]
	fn from(s: Point_<T>) -> Self {
		Self::from_point(s)
	}
}

impl<T> Add for Size_<T>
where
	Self: AddAssign,
{
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T> Sub for Size_<T>
where
	Self: SubAssign,
{
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T> Mul<T> for Size_<T>
where
	Self: MulAssign<T>,
{
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T> Div<T> for Size_<T>
where
	Self: DivAssign<T>,
{
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: AddAssign> AddAssign for Size_<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.width += rhs.width;
		self.height += rhs.height;
	}
}

impl<T: SubAssign> SubAssign for Size_<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.width -= rhs.width;
		self.height -= rhs.height;
	}
}

impl<T: MulAssign + Copy> MulAssign<T> for Size_<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.width *= rhs;
		self.height *= rhs;
	}
}

impl<T: DivAssign + Copy> DivAssign<T> for Size_<T> {
	fn div_assign(&mut self, rhs: T) {
		self.width /= rhs;
		self.height /= rhs;
	}
}

opencv_type_simple_generic! { Size_<Copy> }
