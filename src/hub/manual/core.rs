pub use self::mat::*;
pub use self::point::*;
pub use self::rect::*;
pub use self::scalar::*;
pub use self::size::*;
pub use self::vec::*;

macro_rules! valid_types {
    ($trait: ident, $($rust_type: ty),+) => {
        // todo, make sealed
        pub trait $trait {}

        $(
            impl $trait for $rust_type {}
        )+
    };
}

mod mat;
mod point;
mod rect;
mod scalar;
mod size;
mod vec;

#[inline(always)]
pub const fn MAT_DEPTH(flags: i32) -> i32 {
    #![allow(non_snake_case)]
    flags & crate::hub::core::Mat_DEPTH_MASK
}

#[inline(always)]
pub const fn MAKETYPE(depth: i32, cn: i32) -> i32 {
    #![allow(non_snake_case)]
    MAT_DEPTH(depth) + ((cn - 1) << crate::hub::core::CV_CN_SHIFT)
}
