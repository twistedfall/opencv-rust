type Arg = {{inner_rust_full}};

#[inline]
fn get(&self, index: size_t) -> Result<Self::Storage> {
	extern "C" { fn cv_{{rust_local}}_get(instance: {{rust_extern}}, index: size_t) -> sys::{{rust_return_wrapper_type}}; }
	unsafe { cv_{{rust_local}}_get(self.as_raw_{{rust_local}}(), index) }
		.into_result()
}


