type Arg = {{inner_rust_full}};

#[inline]
fn push(&mut self, val: Self::Arg) {
	extern "C" { fn cv_{{rust_local}}_push(instance: {{rust_extern}}, val: {{inner_rust_extern}}); }
	unsafe { cv_{{rust_local}}_push(self.as_raw_{{rust_local}}(), val.as_raw_{{inner_canonical_rust_local}}()) }
}

#[inline]
fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
	crate::templ::vector_index_check(index, self.len() + 1)?;
	extern "C" { fn cv_{{rust_local}}_insert(instance: {{rust_extern}}, index: size_t, val: {{inner_rust_extern}}); }
	unsafe { cv_{{rust_local}}_insert(self.as_raw_{{rust_local}}(), index, val.as_raw_{{inner_canonical_rust_local}}()) }
	Ok(())
}

#[inline]
fn get(&self, index: size_t) -> Result<Self::Storage> {
	extern "C" { fn cv_{{rust_local}}_get(instance: {{rust_extern}}, index: size_t) -> sys::{{rust_return_wrapper_type}}; }
	unsafe { cv_{{rust_local}}_get(self.as_raw_{{rust_local}}(), index) }
		.into_result()
		.map(|ptr| {{inner_rust_full}} { ptr })
}

#[inline]
unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
	extern "C" { fn cv_{{rust_local}}_get_unchecked(instance: {{rust_extern}}, index: size_t) -> {{inner_rust_extern}}; }
	{{inner_rust_full}} { ptr: cv_{{rust_local}}_get_unchecked(self.as_raw_{{rust_local}}(), index) }
}

#[inline]
fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
	extern "C" { fn cv_{{rust_local}}_set(instance: {{rust_extern}}, index: size_t, val: {{inner_rust_extern}}) -> sys::Result_void; }
	unsafe { cv_{{rust_local}}_set(self.as_raw_{{rust_local}}(), index, val.as_raw_{{inner_canonical_rust_local}}()) }
.into_result()
}

#[inline]
unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
	extern "C" { fn cv_{{rust_local}}_set_unchecked(instance: {{rust_extern}}, index: size_t, val: {{inner_rust_extern}}); }
	cv_{{rust_local}}_set_unchecked(self.as_raw_{{rust_local}}(), index, val.as_raw_{{inner_canonical_rust_local}}())
}
