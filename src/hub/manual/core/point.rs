use crate::hub::manual::core::size::ValidSizeType;

valid_types!(ValidPointType, i32, i64, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point_<T: ValidPointType> {
    pub x: T,
    pub y: T,
}

impl<T: ValidPointType> Point_<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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
