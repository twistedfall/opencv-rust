use opencv::highgui::{MouseEventFlags, WindowFlags};
use opencv::prelude::*;
use opencv::{core, Error};

#[test]
fn bitfield_enum_try_from() {
	let e = MouseEventFlags::try_from(0).unwrap();
	assert_eq!(MouseEventFlags::NONE, e);
	assert!(e.is_set(MouseEventFlags::NONE));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert_eq!(0, e.to_i32());

	let e = MouseEventFlags::try_from_i32(1).unwrap();
	assert_eq!(MouseEventFlags::EVENT_FLAG_LBUTTON, e);
	assert_eq!(1, e.to_i32());

	let e = MouseEventFlags::try_from_i32(2).unwrap();
	assert_eq!(MouseEventFlags::EVENT_FLAG_RBUTTON, e);
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::NONE));
	assert_eq!(2, e.to_i32());

	let e = MouseEventFlags::try_from(3).unwrap();
	assert!(
		e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON)
			&& e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON)
			&& !e.is_set(MouseEventFlags::EVENT_FLAG_MBUTTON)
			&& !e.is_set(MouseEventFlags::NONE)
	);
	assert_eq!(3, e.to_i32());

	let e = MouseEventFlags::try_from(152);
	assert!(matches!(
		e,
		Err(Error {
			code: core::StsBadArg,
			..
		})
	));
}

#[test]
fn bitfield_enum_is_set() {
	{
		let e = MouseEventFlags::EVENT_FLAG_LBUTTON.with(MouseEventFlags::EVENT_FLAG_MBUTTON);
		assert!(e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
		assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
		assert!(e.is_set(MouseEventFlags::EVENT_FLAG_MBUTTON));
		assert!(!e.is_set(MouseEventFlags::NONE));
	}

	{
		let e = MouseEventFlags::NONE;
		assert!(e.is_set(MouseEventFlags::NONE));
		assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	}

	{
		let e = WindowFlags::WINDOW_NORMAL;
		assert!(e.is_set(WindowFlags::WINDOW_NORMAL));
		assert!(e.is_set(WindowFlags::WINDOW_KEEPRATIO)); // also 0 value
	}
}

#[test]
fn bitfield_enum_set() {
	let mut e = MouseEventFlags::NONE;
	e.set(MouseEventFlags::EVENT_FLAG_RBUTTON);
	assert_eq!(2, e.to_i32());
	assert!(!e.is_set(MouseEventFlags::NONE));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));

	e.set(MouseEventFlags::EVENT_FLAG_ALTKEY);
	assert_eq!(34, e.to_i32());
	assert!(!e.is_set(MouseEventFlags::NONE));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_ALTKEY));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_CTRLKEY));

	e.set(MouseEventFlags::NONE);
	assert_eq!(MouseEventFlags::NONE, e);
}

#[test]
fn bitfield_enum_clear() {
	let mut e = MouseEventFlags::EVENT_FLAG_LBUTTON
		.with(MouseEventFlags::EVENT_FLAG_RBUTTON)
		.with(MouseEventFlags::EVENT_FLAG_CTRLKEY);
	assert_eq!(11, e.to_i32());

	e.clear(MouseEventFlags::NONE);
	assert_eq!(11, e.to_i32());

	e.clear(MouseEventFlags::EVENT_FLAG_RBUTTON);
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_CTRLKEY));
	assert_eq!(9, e.to_i32());

	e.clear(MouseEventFlags::EVENT_FLAG_LBUTTON);
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_CTRLKEY));
	assert_eq!(8, e.to_i32());
}

#[test]
fn bitfield_enum_toggle() {
	let mut e = MouseEventFlags::EVENT_FLAG_LBUTTON
		.with(MouseEventFlags::EVENT_FLAG_RBUTTON)
		.with(MouseEventFlags::EVENT_FLAG_CTRLKEY);
	assert_eq!(11, e.to_i32());

	e.toggle(MouseEventFlags::NONE);
	assert_eq!(11, e.to_i32());

	e.toggle(MouseEventFlags::EVENT_FLAG_RBUTTON);
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_CTRLKEY));
	assert_eq!(9, e.to_i32());

	e.toggle(MouseEventFlags::EVENT_FLAG_RBUTTON);
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_CTRLKEY));
	assert_eq!(11, e.to_i32());

	e.toggle(MouseEventFlags::EVENT_FLAG_LBUTTON);
	e.toggle(MouseEventFlags::EVENT_FLAG_RBUTTON);
	e.toggle(MouseEventFlags::EVENT_FLAG_CTRLKEY);
	assert_eq!(MouseEventFlags::NONE, e);
}

#[test]
fn bitfield_enum_with() {
	let e = MouseEventFlags::EVENT_FLAG_LBUTTON.with(MouseEventFlags::EVENT_FLAG_RBUTTON);
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_MBUTTON));
	assert!(!e.is_set(MouseEventFlags::NONE));
	assert_eq!(3, e.to_i32());

	let e = MouseEventFlags::NONE.with(MouseEventFlags::EVENT_FLAG_CTRLKEY);
	assert!(!e.is_set(MouseEventFlags::NONE));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_CTRLKEY));
	assert_eq!(8, e.to_i32());

	let e = MouseEventFlags::EVENT_FLAG_LBUTTON
		.with(MouseEventFlags::EVENT_FLAG_SHIFTKEY)
		.with(MouseEventFlags::NONE);
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_SHIFTKEY));
	assert!(e.is_set(MouseEventFlags::NONE));
}

#[test]
fn bitfield_enum_without() {
	let e = MouseEventFlags::EVENT_FLAG_LBUTTON
		.with(MouseEventFlags::EVENT_FLAG_RBUTTON)
		.with(MouseEventFlags::EVENT_FLAG_SHIFTKEY);
	assert_eq!(19, e.to_i32());

	let e = e.without(MouseEventFlags::EVENT_FLAG_RBUTTON);
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_SHIFTKEY));
	assert_eq!(17, e.to_i32());

	let e = e.without(MouseEventFlags::NONE);
	assert_eq!(17, e.to_i32());

	let e = e.without(MouseEventFlags::EVENT_FLAG_LBUTTON);
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_LBUTTON));
	assert!(!e.is_set(MouseEventFlags::EVENT_FLAG_RBUTTON));
	assert!(e.is_set(MouseEventFlags::EVENT_FLAG_SHIFTKEY));
	assert_eq!(16, e.to_i32());

	let e = e.without(MouseEventFlags::EVENT_FLAG_SHIFTKEY);
	assert_eq!(MouseEventFlags::NONE, e);
}
