{{alias}}
vector_extern! { {{inner_rust_full}}, {{rust_extern_const}}, {{rust_extern_mut}},
	cv_{{rust_localalias}}_new, cv_{{rust_localalias}}_delete,
	cv_{{rust_localalias}}_len, cv_{{rust_localalias}}_is_empty,
	cv_{{rust_localalias}}_capacity, cv_{{rust_localalias}}_shrink_to_fit,
	cv_{{rust_localalias}}_reserve, cv_{{rust_localalias}}_remove,
	cv_{{rust_localalias}}_swap, cv_{{rust_localalias}}_clear,
	cv_{{rust_localalias}}_get -> sys::{{inner_rust_extern_return_wrapper}},
	ret_map: {{ret_map}}
}
{{additional_methods}}

impl<'i> core::VectorTrait<'i> for {{rust_full}} {
	type Arg = {{inner_rust_full_arg}};

	fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }

	#[inline]
	fn push(&mut self, {{inner_rust_func_decl}}) {
		extern "C" { fn cv_{{rust_localalias}}_push(instance: {{rust_extern_mut}}, {{inner_rust_extern_func_decl}}); }
		{{pre_call_infallible}}
		unsafe { cv_{{rust_localalias}}_push(self.as_raw_mut_{{rust_localalias}}(), {{inner_rust_func_call}}) }
	}

	#[inline]
	fn insert(&mut self, index: size_t, {{inner_rust_func_decl}}) -> Result<()> {
		extern "C" { fn cv_{{rust_localalias}}_insert(instance: {{rust_extern_mut}}, index: size_t, {{inner_rust_extern_func_decl}}); }
		core::vector_index_check(index, self.len() + 1)?;
		{{pre_call}}
		unsafe { cv_{{rust_localalias}}_insert(self.as_raw_mut_{{rust_localalias}}(), index, {{inner_rust_func_call}}) }
		Ok(())
	}

	#[inline]
	fn set(&mut self, index: size_t, {{inner_rust_func_decl}}) -> Result<()> {
		extern "C" { fn cv_{{rust_localalias}}_set(instance: {{rust_extern_mut}}, index: size_t, {{inner_rust_extern_func_decl}}); }
		core::vector_index_check(index, self.len())?;
		{{pre_call}}
		unsafe { cv_{{rust_localalias}}_set(self.as_raw_mut_{{rust_localalias}}(), index, {{inner_rust_func_call}}) }
		Ok(())
	}

	#[inline]
	unsafe fn set_unchecked(&mut self, index: size_t, {{inner_rust_func_decl}}) {
		extern "C" { fn cv_{{rust_localalias}}_set(instance: {{rust_extern_mut}}, index: size_t, {{inner_rust_extern_func_decl}}); }
		{{pre_call_infallible}}
		cv_{{rust_localalias}}_set(self.as_raw_mut_{{rust_localalias}}(), index, {{inner_rust_func_call}})
	}
}

unsafe impl Send for {{rust_full}} {}
{{impls}}


