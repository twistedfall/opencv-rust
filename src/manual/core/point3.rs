use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{NumCast, NumOps, ToPrimitive, Zero};

use crate::{
	core::{Point_, VecN},
	opencv_type_simple_generic,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/df/d6c/classcv_1_1Point3__.html)
pub struct Point3_<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Point3_<T> {
	#[inline]
	pub const fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}

	#[inline]
	pub fn from_vec3(vec: VecN<T, 3>) -> Self {
		let [x, y, z] = vec.0;
		Self::new(x, y, z)
	}

	#[inline]
	pub fn from_point(pt: Point_<T>) -> Self
	where
		T: Zero,
	{
		Self::new(pt.x, pt.y, T::zero())
	}

	#[inline]
	pub fn cross(self, pt: Self) -> Self
	where
		T: NumOps + Copy,
	{
		Self::new(
			self.y * pt.z - self.z * pt.y,
			self.z * pt.x - self.x * pt.z,
			self.x * pt.y - self.y * pt.x,
		)
	}

	#[inline]
	pub fn dot(self, pt: Self) -> T
	where
		T: NumOps,
	{
		self.x * pt.x + self.y * pt.y + self.z * pt.z
	}

	#[inline]
	pub fn ddot(self, pt: Self) -> f64
	where
		f64: From<T>,
	{
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let self_z: f64 = From::from(self.z);
		let pt_x: f64 = From::from(pt.x);
		let pt_y: f64 = From::from(pt.y);
		let pt_z: f64 = From::from(pt.z);
		self_x * pt_x + self_y * pt_y + self_z * pt_z
	}

	#[inline]
	pub fn norm(self) -> f64
	where
		f64: From<T>,
	{
		let self_x: f64 = From::from(self.x);
		let self_y: f64 = From::from(self.y);
		let self_z: f64 = From::from(self.z);
		(self_x.powi(2) + self_y.powi(2) + self_z.powi(2)).sqrt()
	}

	#[inline]
	pub fn to<D: NumCast>(self) -> Option<Point3_<D>>
	where
		T: ToPrimitive,
	{
		Some(Point3_::new(D::from(self.x)?, D::from(self.y)?, D::from(self.z)?))
	}

	#[inline]
	pub fn to_vec3(self) -> VecN<T, 3> {
		VecN::<_, 3>::from_array([self.x, self.y, self.z])
	}
}

impl<T> From<(T, T, T)> for Point3_<T> {
	#[inline]
	fn from(s: (T, T, T)) -> Self {
		Self::new(s.0, s.1, s.2)
	}
}

impl<T> From<VecN<T, 3>> for Point3_<T> {
	#[inline]
	fn from(s: VecN<T, 3>) -> Self {
		Self::from_vec3(s)
	}
}

impl<T: Zero> From<Point_<T>> for Point3_<T> {
	#[inline]
	fn from(s: Point_<T>) -> Self {
		Self::from_point(s)
	}
}

impl<T> Add for Point3_<T>
where
	Self: AddAssign,
{
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T> Sub for Point3_<T>
where
	Self: SubAssign,
{
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T> Mul<T> for Point3_<T>
where
	Self: MulAssign<T>,
{
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T> Div<T> for Point3_<T>
where
	Self: DivAssign<T>,
{
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: AddAssign> AddAssign for Point3_<T> {
	fn add_assign(&mut self, rhs: Point3_<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl<T: SubAssign> SubAssign for Point3_<T> {
	fn sub_assign(&mut self, rhs: Point3_<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl<T: MulAssign + Copy> MulAssign<T> for Point3_<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl<T: DivAssign + Copy> DivAssign<T> for Point3_<T> {
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}

opencv_type_simple_generic! { Point3_<Copy> }
