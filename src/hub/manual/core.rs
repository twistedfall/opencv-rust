pub use self::mat::*;

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
