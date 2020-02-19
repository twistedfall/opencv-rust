#[inline]
fn push(&mut self, val: Self::Arg) {
	extern "C" { fn cv_{{rust_local}}_push(instance: {{rust_extern}}, val: *const {{inner_rust_extern}}); }
	unsafe { cv_{{rust_local}}_push(self.as_raw_{{rust_local}}(), &val) }
}

#[inline]
fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
	crate::templ::vector_index_check(index, self.len() + 1)?;
	extern "C" { fn cv_{{rust_local}}_insert(instance: {{rust_extern}}, index: size_t, val: *const {{inner_rust_extern}}); }
	unsafe { cv_{{rust_local}}_insert(self.as_raw_{{rust_local}}(), index, &val) }
	Ok(())
}

#[inline]
unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
	extern "C" { fn cv_{{rust_local}}_get_unchecked(instance: {{rust_extern}}, index: size_t) -> sys::Result<{{inner_rust_extern}}>; }
	cv_{{rust_local}}_get_unchecked(self.as_raw_{{rust_local}}(), index)
		.into_result()
		.expect("Invalid call to vector get_unchecked() method")
}

#[inline]
fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
	extern "C" { fn cv_{{rust_local}}_set(instance: {{rust_extern}}, index: size_t, val: *const {{inner_rust_extern}}) -> sys::Result_void; }
	unsafe { cv_{{rust_local}}_set(self.as_raw_{{rust_local}}(), index, &val) }
		.into_result()
}

#[inline]
unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
	extern "C" { fn cv_{{rust_local}}_set_unchecked(instance: {{rust_extern}}, index: size_t, val: *const {{inner_rust_extern}}); }
	cv_{{rust_local}}_set_unchecked(self.as_raw_{{rust_local}}(), index, &val)
}
