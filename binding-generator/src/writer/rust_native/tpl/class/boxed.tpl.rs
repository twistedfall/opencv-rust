{{doc_comment}}
{{debug}}
pub struct {{rust_local}} {
	ptr: {{rust_extern_mut}}
}

opencv_type_boxed! { {{rust_local}} }

impl Drop for {{rust_local}} {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::{{extern_delete}}(self.as_raw_mut_{{rust_local}}()) };
	}
}

unsafe impl Send for {{rust_local}} {}

{{bases}}
{{impl}}
{{impls}}

