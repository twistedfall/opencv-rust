use std::ptr;

use opencv::core::{Mat, MatSize, Vector};
use opencv::prelude::*;
use opencv::Result;

/// test that null pointers are handled and not passed to slice::from_raw_parts
#[test]
fn from_nullptr_slice() -> Result<()> {
	{
		let mut vec = Vector::<i32>::new();
		let s = vec.as_slice();
		assert!(s.is_empty());

		let s = vec.as_mut_slice();
		assert!(s.is_empty());
	}

	{
		let mut m = Mat::default();
		assert_eq!(ptr::null(), m.data());

		let r = unsafe { m.at_row_unchecked::<u8>(0)? };
		assert!(r.is_empty());

		let r = unsafe { m.at_row_unchecked_mut::<u8>(0)? };
		assert!(r.is_empty());

		let d = unsafe { m.data_typed_unchecked::<u8>()? };
		assert!(d.is_empty());

		let d = unsafe { m.data_typed_unchecked_mut::<u8>()? };
		assert!(d.is_empty());
	}

	{
		let s = unsafe { MatSize::new(ptr::null_mut()) };
		let s = &*s;
		assert!(s.is_empty());
	}

	Ok(())
}
