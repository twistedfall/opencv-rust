use std::mem::size_of;

use crate::hub::sys::cv_return_value;

#[test]
fn cv_return_type() {
    assert_eq!(size_of::<cv_return_value<i32>>(), 24);
    assert_eq!(size_of::<cv_return_value<u8>>(), 24);
    assert_eq!(size_of::<cv_return_value<crate::types::Unit, ()>>(), 16);
}
