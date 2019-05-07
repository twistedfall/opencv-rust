use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::core::{Point_, Size_, ValidPointType, ValidSizeType};

valid_types!(ValidRectType, i32, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect_<T: ValidRectType> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: ValidRectType> Rect_<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, width, height }
    }
}

impl<T: ValidRectType + Default> Default for Rect_<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), width: Default::default(), height: Default::default() }
    }
}

impl<P, R> Add<Point_<P>> for Rect_<R>
    where
        P: ValidPointType,
        R: ValidRectType + AddAssign<P>
{
    type Output = Rect_<R>;

    fn add(mut self, rhs: Point_<P>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<P, R> Sub<Point_<P>> for Rect_<R>
    where
        P: ValidPointType,
        R: ValidRectType + SubAssign<P>
{
    type Output = Rect_<R>;

    fn sub(mut self, rhs: Point_<P>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<S, R> Add<Size_<S>> for Rect_<R>
    where
        S: ValidSizeType,
        R: ValidRectType + AddAssign<S>
{
    type Output = Rect_<R>;

    fn add(mut self, rhs: Size_<S>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<S, R> Sub<Size_<S>> for Rect_<R>
    where
        S: ValidSizeType,
        R: ValidRectType + SubAssign<S>
{
    type Output = Rect_<R>;

    fn sub(mut self, rhs: Size_<S>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<P, R> AddAssign<Point_<P>> for Rect_<R>
    where
        P: ValidPointType,
        R: ValidRectType + AddAssign<P>
{
    fn add_assign(&mut self, rhs: Point_<P>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<P, R> SubAssign<Point_<P>> for Rect_<R>
    where
        P: ValidPointType,
        R: ValidRectType + SubAssign<P>
{
    fn sub_assign(&mut self, rhs: Point_<P>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<S, R> AddAssign<Size_<S>> for Rect_<R>
    where
        S: ValidSizeType,
        R: ValidRectType + AddAssign<S>
{
    fn add_assign(&mut self, rhs: Size_<S>) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<S, R> SubAssign<Size_<S>> for Rect_<R>
    where
        S: ValidSizeType,
        R: ValidRectType + SubAssign<S>
{
    fn sub_assign(&mut self, rhs: Size_<S>) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}
