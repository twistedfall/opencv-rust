use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::manual::core::size::ValidSizeType;

valid_types!(ValidPointType, i32, i64, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// [docs.opencv.org](https://docs.opencv.org/3.4.6/db/d4e/classcv_1_1Point__.html)
pub struct Point_<T: ValidPointType> {
    pub x: T,
    pub y: T,
}

impl<T: ValidPointType> Point_<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: ValidPointType + Mul<Output=T> + Add<Output=T>> Point_<T>
    where
        f64: From<T>
{
    pub fn norm(&self) -> f64 {
        f64::from(self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: ValidPointType + ValidSizeType> Point_<T> {
    pub fn from_size(sz: super::Size_<T>) -> Self {
        Self { x: sz.width, y: sz.height }
    }
}

impl<T: ValidPointType + Default> Default for Point_<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

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
