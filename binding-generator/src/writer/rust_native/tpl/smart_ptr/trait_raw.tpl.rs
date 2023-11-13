impl {{base_const_rust_full}} for {{rust_full}} {
	#[inline] fn as_raw_{{base_rust_local}}(&self) -> *const c_void { self.inner_as_raw() }
}

impl {{base_rust_full}} for {{rust_full}} {
	#[inline] fn as_raw_mut_{{base_rust_local}}(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


