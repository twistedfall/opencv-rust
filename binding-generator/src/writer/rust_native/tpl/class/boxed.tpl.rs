{{doc_comment}}
{{debug}}
pub struct {{rust_local}}{{rust_decl_lt}} {
	ptr: {{rust_extern_mut}},
	{{rust_phantom_ref}}
}

opencv_type_boxed! { {{rust_local}}{{rust_decl_lt}}{{rust_decl_for_lt}} }

impl Drop for {{rust_local}}{{rust_elided_lt}} {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::{{extern_delete}}(self.as_raw_mut_{{rust_local}}()) };
	}
}

unsafe impl Send for {{rust_local}}{{rust_elided_lt}} {}

{{impl}}
{{traits}}
{{impls}}
{{bases}}

