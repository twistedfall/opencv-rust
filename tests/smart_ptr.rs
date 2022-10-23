use std::ffi::c_void;

use opencv::{flann::IndexParams, prelude::*, types::PtrOfIndexParams, Result};

/// Ptr<f32> creation, value mutation and read
#[test]
fn ptr_f32() {
	use opencv::core::Ptr;

	let mut p = Ptr::new(10f32);
	assert_eq!(10., *p);
	*p = 30.123;
	assert_eq!(30.123, *p);

	let d = Ptr::<f32>::default();
	assert_eq!(*d, f32::default());
}

#[test]
fn ptr_f32_into_raw() -> Result<()> {
	use opencv::types::PtrOff32;

	#[inline(never)]
	fn into_raw(a: PtrOff32) -> *mut c_void {
		a.into_raw()
	}

	let a = PtrOff32::new(87.);
	let ptr = into_raw(a);
	let b = unsafe { PtrOff32::from_raw(ptr) };
	assert_eq!(87., *b);
	Ok(())
}

#[test]
fn into_raw_ptrofboxed() -> Result<()> {
	#[inline(never)]
	fn into_raw(a: PtrOfIndexParams) -> *mut c_void {
		a.into_raw()
	}

	let mut params = IndexParams::default()?;
	params.set_int("int", 87)?;
	params.set_double("double", 12.34)?;
	params.set_string("string", "my string")?;
	let a = PtrOfIndexParams::new(params);
	assert_eq!(87, a.get_int("int", 3)?);
	assert_eq!(3, a.get_int("int_non_existent", 3)?);
	assert_eq!(12.34, a.get_double("double", 99.99)?);
	assert_eq!("my string", a.get_string("string", "-")?);
	let ptr = into_raw(a);
	let b = unsafe { PtrOfIndexParams::from_raw(ptr) };
	assert_eq!(87, b.get_int("int", 3)?);
	assert_eq!(3, b.get_int("int_non_existent", 3)?);
	assert_eq!(12.34, b.get_double("double", 99.99)?);
	assert_eq!("my string", b.get_string("string", "-")?);
	Ok(())
}
