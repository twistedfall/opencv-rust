use std::ffi::c_void;
use std::ops::{Deref, DerefMut};

use crate::{
	core::Ptr,
	traits::Boxed,
};

impl Ptr<f32> {
	pub fn new(val: f32) -> Self {
		extern "C" { fn cv_PtrOff32_new(val: f32) -> *mut c_void; }
		unsafe { Self::from_raw(cv_PtrOff32_new(val)) }
	}
}

impl Default for Ptr<f32> {
	fn default() -> Self {
		Self::new(Default::default())
	}
}

impl Deref for Ptr<f32> {
	type Target = f32;

	fn deref(&self) -> &Self::Target {
		unsafe { (self.inner_as_raw() as *const f32).as_ref() }.expect("Got null inner pointer for Ptr<f32>")
	}
}

impl DerefMut for Ptr<f32> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { (self.inner_as_raw() as *mut f32).as_mut() }.expect("Got null mut inner pointer for Ptr<f32>")
	}
}

