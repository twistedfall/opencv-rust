impl Clone for {{rust_local}} {
	#[inline]
	/// Calls try_clone() and panics if that fails
	fn clone(&self) -> Self {
		self.try_clone().expect("Cannot clone {{rust_local}}")
	}
}


