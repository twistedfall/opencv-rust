use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{NumCast, NumOps, ToPrimitive};

use crate::{
	core::{Rect_, Size_, VecN},
	opencv_type_simple_generic,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/db/d4e/classcv_1_1Point__.html)
pub struct Point_<T> {
	pub x: T,
	pub y: T,
}

impl<T> Point_<T> {
	#[inline]
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}

	#[inline]
	pub fn from_vec2(vec: VecN<T, 2>) -> Self {
		let [x, y] = vec.0;
		Self::new(x, y)
	}

	#[inline]
	pub fn from_size(sz: Size_<T>) -> Self {
		Self::new(sz.width, sz.height)
	}

	#[inline]
	pub fn cross(self, pt: Point_<T>) -> f64
	where
		f64: From<T>,
	{
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let pt_x: f64 = From::from(pt.x);
		let pt_y: f64 = From::from(pt.y);
		self_x * pt_y - self_y * pt_x
	}

	#[inline]
	pub fn dot(self, pt: Point_<T>) -> T
	where
		T: NumOps,
	{
		self.x * pt.x + self.y * pt.y
	}

	#[inline]
	pub fn ddot(self, pt: Point_<T>) -> f64
	where
		f64: From<T>,
	{
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let pt_x: f64 = From::from(pt.x);
		let pt_y: f64 = From::from(pt.y);
		self_x * pt_x + self_y * pt_y
	}

	#[inline]
	pub fn inside(self, rect: Rect_<T>) -> bool
	where
		T: PartialOrd + Add<Output = T> + Copy,
	{
		rect.contains(self)
	}

	#[inline]
	pub fn norm(self) -> f64
	where
		f64: From<T>,
	{
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		(self_x.powi(2) + self_y.powi(2)).sqrt()
	}

	#[inline]
	pub fn to<D: NumCast>(self) -> Option<Point_<D>>
	where
		T: ToPrimitive,
	{
		Some(Point_::new(D::from(self.x)?, D::from(self.y)?))
	}

	#[inline]
	pub fn to_vec2(self) -> VecN<T, 2> {
		VecN::<_, 2>::from_array([self.x, self.y])
	}
}

impl<T> From<(T, T)> for Point_<T> {
	#[inline]
	fn from(s: (T, T)) -> Self {
		Self::new(s.0, s.1)
	}
}

impl<T> From<VecN<T, 2>> for Point_<T> {
	#[inline]
	fn from(s: VecN<T, 2>) -> Self {
		Self::from_vec2(s)
	}
}

impl<T> From<Size_<T>> for Point_<T> {
	#[inline]
	fn from(s: Size_<T>) -> Self {
		Self::from_size(s)
	}
}

impl<T> Add for Point_<T>
where
	Self: AddAssign,
{
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T> Sub for Point_<T>
where
	Self: SubAssign,
{
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T> Mul<T> for Point_<T>
where
	Self: MulAssign<T>,
{
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T> Div<T> for Point_<T>
where
	Self: DivAssign<T>,
{
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: AddAssign> AddAssign for Point_<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T: SubAssign> SubAssign for Point_<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T: MulAssign + Copy> MulAssign<T> for Point_<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T: DivAssign + Copy> DivAssign<T> for Point_<T> {
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

opencv_type_simple_generic! { Point_<Copy> }
