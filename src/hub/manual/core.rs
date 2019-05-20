pub use self::mat::*;
pub use self::point::*;
pub use self::rect::*;
pub use self::size::*;
pub use self::vec::*;

macro_rules! valid_types {
    ($trait: ident, $($rust_type: ty),+) => {
        /// This sealed trait is implemented for types that are valid to use in corresponding context
        pub trait $trait: Copy + private::Sealed {}

        mod private {
            pub trait Sealed {}
        }

        $(
            impl $trait for $rust_type {}
            impl private::Sealed for $rust_type {}
        )+
    };
}

mod mat;
mod point;
mod rect;
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
