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
