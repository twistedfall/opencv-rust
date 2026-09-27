impl fmt::Debug for {{rust_full}} {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("{{rust_localalias}}"){{debug_fields}}
			.finish()
	}
}


