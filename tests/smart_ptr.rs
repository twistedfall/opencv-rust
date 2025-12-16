use std::ffi::c_void;

use opencv::core::Ptr;
use opencv::prelude::*;
use opencv::Result;

/// Ptr<f32> creation, value mutation and read
#[test]
fn ptr_f32() {
	let mut p = Ptr::new(10f32);
	assert_eq!(10., *p);
	*p = 30.123;
	assert_eq!(30.123, *p);

	let d = Ptr::<f32>::default();
	assert_eq!(*d, f32::default());
}

#[test]
fn ptr_f32_into_raw() -> Result<()> {
	#[inline(never)]
	fn into_raw(a: Ptr<f32>) -> *mut c_void {
		a.into_raw()
	}

	let a = Ptr::<f32>::new(87.);
	let ptr = into_raw(a);
	let b = unsafe { Ptr::<f32>::from_raw(ptr) };
	assert_eq!(87., *b);
	Ok(())
}

#[test]
fn into_raw_ptrofboxed() -> Result<()> {
	#![cfg(ocvrs_has_module_flann)]
	use opencv::flann::IndexParams;

	#[inline(never)]
	fn into_raw(a: Ptr<IndexParams>) -> *mut c_void {
		a.into_raw()
	}

	let mut params = IndexParams::default()?;
	params.set_int("int", 87)?;
	params.set_double("double", 12.34)?;
	params.set_string("string", "my string")?;
	let a = Ptr::<IndexParams>::new(params);
	assert_eq!(87, a.get_int("int", 3)?);
	assert_eq!(3, a.get_int("int_non_existent", 3)?);
	assert_eq!(12.34, a.get_double("double", 99.99)?);
	assert_eq!("my string", a.get_string("string", "-")?);
	let ptr = into_raw(a);
	let b = unsafe { Ptr::<IndexParams>::from_raw(ptr) };
	assert_eq!(87, b.get_int("int", 3)?);
	assert_eq!(3, b.get_int("int_non_existent", 3)?);
	assert_eq!(12.34, b.get_double("double", 99.99)?);
	assert_eq!("my string", b.get_string("string", "-")?);
	Ok(())
}
