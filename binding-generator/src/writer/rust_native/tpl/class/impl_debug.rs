impl std::fmt::Debug for {{rust_local}}{{rust_elided_lt}} {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("{{rust_local}}"){{debug_fields}}
			.finish()
	}
}


