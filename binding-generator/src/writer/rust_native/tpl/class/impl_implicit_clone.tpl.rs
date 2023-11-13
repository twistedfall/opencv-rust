impl Clone for {{rust_local}} {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::{{extern_implicit_clone}}(self.as_raw_{{rust_local}}())) }
	}
}


