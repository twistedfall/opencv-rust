//! Shared Test Utils

use std::fmt::Debug;

pub fn assert_is_err<T, E>(res: Result<T, E>)
where
    T: Debug,
    E: Debug,
{
    if res.is_ok() {
        assert!(false, "Expected {:?} to be an Error", res);
    }
}
