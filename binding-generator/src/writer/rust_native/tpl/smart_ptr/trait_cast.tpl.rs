impl {{base_rust_full}} for {{rust_localalias}} {
	fn as_raw_{{base_rust_local}}(&self) -> {{rust_extern_const}} { self.inner_as_raw() }
	fn as_raw_mut_{{base_rust_local}}(&mut self) -> {{rust_extern_mut}} { self.inner_as_raw_mut() }
}


