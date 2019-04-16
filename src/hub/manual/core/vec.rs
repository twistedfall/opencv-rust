use std::ops::{Deref, DerefMut};

valid_types!(ValidVecType, u8, i16, u16, i32, f32, f64);

macro_rules! vec_impl {
    ($type: ident, $count: expr) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $type<T: ValidVecType>(pub [T; $count]);

        impl<T: ValidVecType> From<[T; $count]> for $type<T> {
            fn from(s: [T; $count]) -> $type<T> {
                $type::<T>(s)
            }
        }

        impl<T: ValidVecType> Deref for $type<T> {
            type Target = [T; $count];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T: ValidVecType> DerefMut for $type<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T: ValidVecType + Default + Copy> Default for $type<T> {
            fn default() -> Self {
                Self([Default::default(); $count])
            }
        }
    };
}

vec_impl!(Vec2, 2);
vec_impl!(Vec3, 3);
vec_impl!(Vec4, 4);
vec_impl!(Vec6, 6);
vec_impl!(Vec8, 8);
