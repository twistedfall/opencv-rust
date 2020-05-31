use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{NumCast, ToPrimitive};

use crate::core::{Point_, ValidPointType};

valid_types!(ValidSizeType, i32, i64, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
/// [docs.opencv.org](https://docs.opencv.org/master/d6/d50/classcv_1_1Size__.html)
pub struct Size_<T: ValidSizeType> {
    pub width: T,
    pub height: T,
}

impl<T: ValidSizeType> Size_<T> {
    #[inline]
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn from_point(pt: Point_<T>) -> Self where T: ValidPointType {
        Self { width: pt.x, height: pt.y }
    }

    #[inline]
    pub fn area(self) -> T {
        self.width * self.height
    }

    #[inline]
    pub fn empty(self) -> bool {
        self.width <= T::zero() || self.height <= T::zero()
    }

    #[inline]
    pub fn to<D: ValidSizeType + NumCast>(self) -> Option<Size_<D>> where T: ToPrimitive {
        Some(Size_ { width: D::from(self.width)?, height: D::from(self.height)? })
    }
}

impl<T> Add for Size_<T>
    where
        T: ValidSizeType + AddAssign,
{
    type Output = Size_<T>;

    fn add(mut self, rhs: Size_<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub for Size_<T>
    where
        T: ValidSizeType + SubAssign,
{
    type Output = Size_<T>;

    fn sub(mut self, rhs: Size_<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Mul<T> for Size_<T>
    where
        T: ValidSizeType + MulAssign
{
    type Output = Size_<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T> Div<T> for Size_<T>
    where
        T: ValidSizeType + DivAssign
{
    type Output = Size_<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T> AddAssign for Size_<T>
    where
        T: ValidSizeType + AddAssign,
{
    fn add_assign(&mut self, rhs: Size_<T>) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<T> SubAssign for Size_<T>
    where
        T: ValidSizeType + SubAssign,
{
    fn sub_assign(&mut self, rhs: Size_<T>) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T> MulAssign<T> for Size_<T>
    where
        T: ValidSizeType + MulAssign
{
    fn mul_assign(&mut self, rhs: T) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<T> DivAssign<T> for Size_<T>
    where
        T: ValidSizeType + DivAssign
{
    fn div_assign(&mut self, rhs: T) {
        self.width /= rhs;
        self.height /= rhs;
    }
}
