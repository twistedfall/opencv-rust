impl {{base_const_rust_full}} for {{rust_localalias}} {
	#[inline] fn as_raw_{{base_rust_local}}(&self) -> {{rust_extern_const}} { self.inner_as_raw() }
}

impl {{base_rust_full}} for {{rust_localalias}} {
	#[inline] fn as_raw_mut_{{base_rust_local}}(&mut self) -> {{rust_extern_mut}} { self.inner_as_raw_mut() }
}


