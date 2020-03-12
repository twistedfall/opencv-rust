type Arg = &'i str;

#[inline]
fn push(&mut self, val: Self::Arg) {
	extern "C" { fn cv_{{rust_local}}_push(instance: {{rust_extern}}, val: {{inner_rust_extern}}); }
	string_arg_infallible!(val);
	unsafe { cv_{{rust_local}}_push(self.as_raw_{{rust_local}}(), val.as_ptr() as _) }
}

#[inline]
fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
	extern "C" { fn cv_{{rust_local}}_insert(instance: {{rust_extern}}, index: size_t, val: {{inner_rust_extern}}); }
	crate::templ::vector_index_check(index, self.len() + 1)?;
	string_arg_infallible!(val);
	unsafe { cv_{{rust_local}}_insert(self.as_raw_{{rust_local}}(), index, val.as_ptr() as _) }
	Ok(())
}

#[inline]
fn get(&self, index: size_t) -> Result<Self::Storage> {
	extern "C" { fn cv_{{rust_local}}_get(instance: {{rust_extern}}, index: size_t) -> sys::Result<*mut c_void>; }
	unsafe { cv_{{rust_local}}_get(self.as_raw_{{rust_local}}(), index) }
		.into_result()
		.map(|s| unsafe { crate::templ::receive_string(s as *mut String) })
}

#[inline]
unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
	extern "C" { fn cv_{{rust_local}}_get_unchecked(instance: {{rust_extern}}, index: size_t) -> *mut c_void; }
	crate::templ::receive_string(cv_{{rust_local}}_get_unchecked(self.as_raw_{{rust_local}}(), index) as *mut String)
}

#[inline]
fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
	extern "C" { fn cv_{{rust_local}}_set(instance: {{rust_extern}}, index: size_t, val: {{inner_rust_extern}}) -> sys::Result_void; }
	string_arg_infallible!(val);
	unsafe { cv_{{rust_local}}_set(self.as_raw_{{rust_local}}(), index, val.as_ptr() as _) }
		.into_result()
}

#[inline]
unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
	extern "C" { fn cv_{{rust_local}}_set_unchecked(instance: {{rust_extern}}, index: size_t, val: {{inner_rust_extern}}); }
	string_arg_infallible!(val);
	cv_{{rust_local}}_set_unchecked(self.as_raw_{{rust_local}}(), index, val.as_ptr() as _)
}
