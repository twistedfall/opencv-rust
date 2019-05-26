//! Shared Test Utils

use std::fmt::Debug;

const F32_EPSILON: f32 = 1.0e-4;

pub fn assert_almost_eq(left: f32, right: f32) {
    assert!(
        f32_almost_eq(left, right),
        "{} is not approximately equal to {}.",
        left,
        right,
    );
}

pub fn assert_is_err<T, E>(res: Result<T, E>)
where
    T: Debug,
    E: Debug,
{
    if res.is_ok() {
        assert!(false, "Expected {:?} to be an Error", res);
    }
}

fn f32_almost_eq(left: f32, right: f32) -> bool {
    (left - right).abs() < F32_EPSILON
}
