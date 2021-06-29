impl {{base_const_rust_full}} for {{rust_local}} {
	#[inline] fn as_raw_{{base_rust_local}}(&self) -> {{base_rust_extern_const}} { self.as_raw() }
}

impl {{base_rust_full}} for {{rust_local}} {
	#[inline] fn as_raw_mut_{{base_rust_local}}(&mut self) -> {{base_rust_extern_mut}} { self.as_raw_mut() }
}


