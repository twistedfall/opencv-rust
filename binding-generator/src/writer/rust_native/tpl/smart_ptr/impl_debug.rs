impl std::fmt::Debug for {{rust_full}} {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("{{rust_localalias}}"){{debug_fields}}
			.finish()
	}
}


