impl {{base_rust_full_const}} for {{rust_full}} {
	#[inline] fn {{base_rust_as_raw_const}}(&self) -> *const c_void { self.inner_as_raw() }
}

impl {{base_rust_full_mut}} for {{rust_full}} {
	#[inline] fn {{base_rust_as_raw_mut}}(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


