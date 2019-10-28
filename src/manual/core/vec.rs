use std::ops::{Deref, DerefMut};

pub use self::{scalar::ValidScalarType, vec::ValidVecType};

// additional modules needed because valid_types! introduces named private module
mod vec {
    valid_types!(ValidVecType, i8, u8, i16, u16, i32, f32, f64);
}

mod scalar {
    valid_types!(ValidScalarType, f64);
}

macro_rules! vec_impl {
    ($type: ident, $count: expr, $type_trait: ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
        pub struct $type<T: $type_trait>(pub [T; $count]);

        impl<T: $type_trait> From<[T; $count]> for $type<T> {
            fn from(s: [T; $count]) -> $type<T> {
                $type::<T>(s)
            }
        }

        impl<T: $type_trait> $type<T> {
            pub fn all(v0: T) -> Self {
                Self([v0; $count])
            }
        }

        impl<T: $type_trait> Deref for $type<T> {
            type Target = [T; $count];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T: $type_trait> DerefMut for $type<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T: $type_trait + Default> Default for $type<T> {
            fn default() -> Self {
                Self([Default::default(); $count])
            }
        }
    };
}

vec_impl!(Vec2, 2, ValidVecType);
vec_impl!(Vec3, 3, ValidVecType);
vec_impl!(Vec4, 4, ValidVecType);
vec_impl!(Vec6, 6, ValidVecType);
vec_impl!(Vec8, 8, ValidVecType);
vec_impl!(Scalar_, 4, ValidScalarType);

impl<T: ValidScalarType> Scalar_<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Self([v0, v1, v2, v3])
    }
}
