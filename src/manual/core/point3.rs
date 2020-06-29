use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{NumCast, ToPrimitive, Zero};

use crate::core::{Point_, ValidPointType, ValidVecType, Vec3};

valid_types!(ValidPoint3Type: i32, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/df/d6c/classcv_1_1Point3__.html)
pub struct Point3_<T: ValidPoint3Type> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T: ValidPoint3Type> Point3_<T> {
	#[inline]
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}

	#[inline]
	pub fn from_vec3(vec: Vec3<T>) -> Self where T: ValidVecType {
		Self::new(vec[0], vec[1], vec[2])
	}

	#[inline]
	pub fn from_point(pt: Point_<T>) -> Self where T: ValidPointType + Zero {
		Self::new(pt.x, pt.y, T::zero())
	}

	#[inline]
	pub fn cross(self, pt: Point3_<T>) -> Point3_<T> {
		Self::new(
			self.y * pt.z - self.z * pt.y,
			self.z * pt.x - self.x * pt.z,
			self.x * pt.y - self.y * pt.x,
		)
	}

	#[inline]
	pub fn dot(self, pt: Point3_<T>) -> T {
		self.x * pt.x + self.y * pt.y + self.z * pt.z
	}

	#[inline]
	pub fn ddot(self, pt: Point3_<T>) -> f64 where f64: From<T> {
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let self_z: f64 = From::from(self.z);
		let pt_x: f64 = From::from(pt.x);
		let pt_y: f64 = From::from(pt.y);
		let pt_z: f64 = From::from(pt.z);
		self_x * pt_x + self_y * pt_y + self_z * pt_z
	}

	#[inline]
	pub fn norm(self) -> f64 where f64: From<T> {
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let self_z: f64 = From::from(self.z);
		(self_x.powi(2) + self_y.powi(2) + self_z.powi(2)).sqrt()
	}

	#[inline]
	pub fn to<D: ValidPoint3Type + NumCast>(self) -> Option<Point3_<D>> where T: ToPrimitive {
		Some(Point3_::new(D::from(self.x)?, D::from(self.y)?, D::from(self.z)?))
	}

	#[inline]
	pub fn to_vec3(&self) -> Vec3<T> where T: ValidVecType {
		Vec3::from([self.x, self.y, self.z])
	}
}
opencv_type_simple_generic! { Point3_<ValidPoint3Type> }

impl<T> Add for Point3_<T>
	where
		T: ValidPoint3Type + AddAssign,
{
	type Output = Point3_<T>;

	fn add(mut self, rhs: Point3_<T>) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T> Sub for Point3_<T>
	where
		T: ValidPoint3Type + SubAssign,
{
	type Output = Point3_<T>;

	fn sub(mut self, rhs: Point3_<T>) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T> Mul<T> for Point3_<T>
	where
		T: ValidPoint3Type + MulAssign
{
	type Output = Point3_<T>;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T> Div<T> for Point3_<T>
	where
		T: ValidPoint3Type + DivAssign
{
	type Output = Point3_<T>;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T> AddAssign for Point3_<T>
	where
		T: ValidPoint3Type + AddAssign,
{
	fn add_assign(&mut self, rhs: Point3_<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl<T> SubAssign for Point3_<T>
	where
		T: ValidPoint3Type + SubAssign,
{
	fn sub_assign(&mut self, rhs: Point3_<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl<T> MulAssign<T> for Point3_<T>
	where
		T: ValidPoint3Type + MulAssign
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl<T> DivAssign<T> for Point3_<T>
	where
		T: ValidPoint3Type + DivAssign
{
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}
