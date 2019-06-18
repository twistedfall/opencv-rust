valid_types!(ValidSizeType, i32, i64, f32, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size_<T: ValidSizeType> {
    pub width: T,
    pub height: T,
}

impl<T: ValidSizeType> Size_<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: ValidSizeType + Default> Default for Size_<T> {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default() }
    }
}
