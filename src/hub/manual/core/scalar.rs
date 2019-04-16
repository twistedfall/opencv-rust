use std::ops::{Deref, DerefMut};

valid_types!(ValidScalarType, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Scalar_<T: ValidScalarType>(pub [T; 4]);

impl<T: ValidScalarType> Scalar_<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Self([v0, v1, v2, v3])
    }
}

impl<T: ValidScalarType + Copy> Scalar_<T> {
    pub fn all(v0: T) -> Self {
        Self([v0; 4])
    }
}

impl<T: ValidScalarType> Deref for Scalar_<T> {
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ValidScalarType> DerefMut for Scalar_<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ValidScalarType + Default + Copy> Default for Scalar_<T> {
    fn default() -> Self {
        Self([Default::default(); 4])
    }
}
