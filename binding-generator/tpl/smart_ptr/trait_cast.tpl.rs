impl {{base_rust_full}} for {{rust_local}} {
	fn as_raw_{{base_rust_local}}(&self) -> {{rust_extern_mut}} {
		self.get_inner_ptr()
	}
}


