impl fmt::Debug for {{rust_local}}{{rust_elided_lt}} {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("{{rust_local}}"){{debug_fields}}
			.finish()
	}
}


