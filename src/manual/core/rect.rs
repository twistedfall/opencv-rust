use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Mul, Sub, SubAssign};

use num_traits::{NumCast, NumOps, ToPrimitive, Zero};

use crate::core::{Point_, Size_};
use crate::opencv_type_simple_generic;

#[inline(always)]
fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
	if a <= b {
		a
	} else {
		b
	}
}

#[inline(always)]
fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
	if b >= a {
		b
	} else {
		a
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/d2/d44/classcv_1_1Rect__.html)
pub struct Rect_<T> {
	pub x: T,
	pub y: T,
	pub width: T,
	pub height: T,
}

impl<T> Rect_<T> {
	#[inline]
	pub const fn new(x: T, y: T, width: T, height: T) -> Self {
		Self { x, y, width, height }
	}

	#[inline]
	pub fn from_point_size(pt: Point_<T>, sz: Size_<T>) -> Self {
		Self::new(pt.x, pt.y, sz.width, sz.height)
	}

	#[inline]
	pub fn from_points(pt1: Point_<T>, pt2: Point_<T>) -> Self
	where
		T: PartialOrd + Sub<Output = T> + Copy,
	{
		let x = partial_min(pt1.x, pt2.x);
		let y = partial_min(pt1.y, pt2.y);
		Self::new(x, y, partial_max(pt1.x, pt2.x) - x, partial_max(pt1.y, pt2.y) - y)
	}

	#[inline]
	pub fn tl(&self) -> Point_<T>
	where
		T: Copy,
	{
		Point_::new(self.x, self.y)
	}

	#[inline]
	pub fn br(&self) -> Point_<T>
	where
		T: Add<Output = T> + Copy,
	{
		Point_::new(self.x + self.width, self.y + self.height)
	}

	#[inline]
	pub fn size(&self) -> Size_<T>
	where
		T: Copy,
	{
		Size_::new(self.width, self.height)
	}

	#[inline]
	pub fn area(&self) -> T
	where
		T: Mul<Output = T> + Copy,
	{
		self.width * self.height
	}

	#[inline]
	pub fn empty(&self) -> bool
	where
		T: PartialOrd + Zero,
	{
		self.width <= T::zero() || self.height <= T::zero()
	}

	#[inline]
	pub fn contains(&self, pt: Point_<T>) -> bool
	where
		T: PartialOrd + Add<Output = T> + Copy,
	{
		self.x <= pt.x && pt.x < self.x + self.width && self.y <= pt.y && pt.y < self.y + self.height
	}

	#[inline]
	pub fn to<D: NumCast>(&self) -> Option<Rect_<D>>
	where
		T: ToPrimitive + Copy,
	{
		Some(Rect_ {
			x: D::from(self.x)?,
			y: D::from(self.y)?,
			width: D::from(self.width)?,
			height: D::from(self.height)?,
		})
	}
}

impl<T> From<(T, T, T, T)> for Rect_<T> {
	#[inline]
	fn from(s: (T, T, T, T)) -> Self {
		Self::new(s.0, s.1, s.2, s.3)
	}
}

impl<T> From<(Point_<T>, Size_<T>)> for Rect_<T> {
	#[inline]
	fn from(s: (Point_<T>, Size_<T>)) -> Self {
		Self::from_point_size(s.0, s.1)
	}
}

impl<T: PartialOrd + Sub<Output = T> + Copy> From<(Point_<T>, Point_<T>)> for Rect_<T> {
	#[inline]
	fn from(s: (Point_<T>, Point_<T>)) -> Self {
		Self::from_points(s.0, s.1)
	}
}

impl<P, R> Add<Point_<P>> for Rect_<R>
where
	Self: AddAssign<Point_<P>>,
{
	type Output = Self;

	fn add(mut self, rhs: Point_<P>) -> Self::Output {
		self += rhs;
		self
	}
}

impl<P, R> Sub<Point_<P>> for Rect_<R>
where
	Self: SubAssign<Point_<P>>,
{
	type Output = Self;

	fn sub(mut self, rhs: Point_<P>) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<S, R> Add<Size_<S>> for Rect_<R>
where
	Self: AddAssign<Size_<S>>,
{
	type Output = Self;

	fn add(mut self, rhs: Size_<S>) -> Self::Output {
		self += rhs;
		self
	}
}

impl<S, R> Sub<Size_<S>> for Rect_<R>
where
	Self: SubAssign<Size_<S>>,
{
	type Output = Self;

	fn sub(mut self, rhs: Size_<S>) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T> BitOr for Rect_<T>
where
	Rect_<T>: BitOrAssign,
{
	type Output = Rect_<T>;

	fn bitor(mut self, rhs: Self) -> Self::Output {
		self |= rhs;
		self
	}
}

impl<T> BitAnd for Rect_<T>
where
	Rect_<T>: BitAndAssign,
{
	type Output = Rect_<T>;

	fn bitand(mut self, rhs: Self) -> Self::Output {
		self &= rhs;
		self
	}
}

impl<P, R: AddAssign<P>> AddAssign<Point_<P>> for Rect_<R> {
	fn add_assign(&mut self, rhs: Point_<P>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<P, R: SubAssign<P>> SubAssign<Point_<P>> for Rect_<R> {
	fn sub_assign(&mut self, rhs: Point_<P>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<S, R: AddAssign<S>> AddAssign<Size_<S>> for Rect_<R> {
	fn add_assign(&mut self, rhs: Size_<S>) {
		self.width += rhs.width;
		self.height += rhs.height;
	}
}

impl<S, R: SubAssign<S>> SubAssign<Size_<S>> for Rect_<R> {
	fn sub_assign(&mut self, rhs: Size_<S>) {
		self.width -= rhs.width;
		self.height -= rhs.height;
	}
}

impl<T: PartialOrd + NumOps + Zero + Copy> BitOrAssign for Rect_<T> {
	fn bitor_assign(&mut self, rhs: Self) {
		if self.empty() {
			*self = rhs;
		} else if !rhs.empty() {
			let x1 = partial_min(self.x, rhs.x);
			let y1 = partial_min(self.y, rhs.y);
			self.width = partial_max(self.x + self.width, rhs.x + rhs.width) - x1;
			self.height = partial_max(self.y + self.height, rhs.y + rhs.height) - y1;
			self.x = x1;
			self.y = y1;
		}
	}
}

impl<T: PartialOrd + NumOps + Zero + Copy> BitAndAssign for Rect_<T>
where
	Self: Default,
{
	fn bitand_assign(&mut self, rhs: Self) {
		let x1 = partial_max(self.x, rhs.x);
		let y1 = partial_max(self.y, rhs.y);
		self.width = partial_min(self.x + self.width, rhs.x + rhs.width) - x1;
		self.height = partial_min(self.y + self.height, rhs.y + rhs.height) - y1;
		self.x = x1;
		self.y = y1;
		if self.empty() {
			*self = Self::default();
		}
	}
}

#[test]
fn test_partial() {
	assert_eq!(1., partial_min(1., 2.));
	assert_eq!(1., partial_min(2., 1.));
	assert_eq!(1., partial_min(1., 1.));
	assert_eq!(1, partial_min(1, 2));
	assert_eq!(1, partial_min(2, 1));
	assert_eq!(1, partial_min(1, 1));

	assert_eq!(2., partial_max(1., 2.));
	assert_eq!(2., partial_max(2., 1.));
	assert_eq!(2., partial_max(2., 2.));
	assert_eq!(2, partial_max(1, 2));
	assert_eq!(2, partial_max(2, 1));
	assert_eq!(2, partial_max(2, 2));
}

opencv_type_simple_generic! { Rect_<Copy> }
