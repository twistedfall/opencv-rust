use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{NumCast, ToPrimitive};

use crate::core::{Rect_, Size_, ValidRectType, ValidSizeType, ValidVecType, Vec2};

valid_types!(ValidPointType: i32, i64, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/db/d4e/classcv_1_1Point__.html)
pub struct Point_<T: ValidPointType> {
	pub x: T,
	pub y: T,
}

impl<T: ValidPointType> Point_<T> {
	#[inline]
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}

	#[inline]
	pub fn from_vec2(vec: Vec2<T>) -> Self where T: ValidVecType {
		Self::new(vec[0], vec[1])
	}

	#[inline]
	pub fn from_size(sz: Size_<T>) -> Self where T: ValidSizeType {
		Self::new(sz.width, sz.height)
	}

	#[inline]
	pub fn cross(self, pt: Point_<T>) -> f64 where f64: From<T> {
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let pt_x: f64 = From::from(pt.x);
		let pt_y: f64 = From::from(pt.y);
		self_x * pt_y - self_y * pt_x
	}

	#[inline]
	pub fn dot(self, pt: Point_<T>) -> T {
		self.x * pt.x + self.y * pt.y
	}

	#[inline]
	pub fn ddot(self, pt: Point_<T>) -> f64 where f64: From<T> {
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let pt_x: f64 = From::from(pt.x);
		let pt_y: f64 = From::from(pt.y);
		self_x * pt_x + self_y * pt_y
	}

	#[inline]
	pub fn inside(self, rect: Rect_<T>) -> bool where T: ValidRectType {
		rect.contains(self)
	}

	#[inline]
	pub fn norm(self) -> f64 where f64: From<T> {
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		(self_x.powi(2) + self_y.powi(2)).sqrt()
	}

	#[inline]
	pub fn to<D: ValidPointType + NumCast>(self) -> Option<Point_<D>> where T: ToPrimitive {
		Some(Point_::new(D::from(self.x)?, D::from(self.y)?))
	}

	#[inline]
	pub fn to_vec2(&self) -> Vec2<T> where T: ValidVecType {
		Vec2::from([self.x, self.y])
	}
}

opencv_type_simple_generic! { Point_<ValidPointType> }

impl<T> Add for Point_<T>
	where
		T: ValidPointType + AddAssign,
{
	type Output = Point_<T>;

	fn add(mut self, rhs: Point_<T>) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T> Sub for Point_<T>
	where
		T: ValidPointType + SubAssign,
{
	type Output = Point_<T>;

	fn sub(mut self, rhs: Point_<T>) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T> Mul<T> for Point_<T>
	where
		T: ValidPointType + MulAssign
{
	type Output = Point_<T>;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T> Div<T> for Point_<T>
	where
		T: ValidPointType + DivAssign
{
	type Output = Point_<T>;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T> AddAssign for Point_<T>
	where
		T: ValidPointType + AddAssign,
{
	fn add_assign(&mut self, rhs: Point_<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> SubAssign for Point_<T>
	where
		T: ValidPointType + SubAssign,
{
	fn sub_assign(&mut self, rhs: Point_<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> MulAssign<T> for Point_<T>
	where
		T: ValidPointType + MulAssign
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> DivAssign<T> for Point_<T>
	where
		T: ValidPointType + DivAssign
{
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}
