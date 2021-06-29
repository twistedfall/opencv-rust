{{doc_comment}}
{{debug}}
pub struct {{rust_local}} {
	ptr: {{rust_extern_mut}}
}

opencv_type_boxed! { {{rust_local}} }

impl Drop for {{rust_local}} {
	fn drop(&mut self) {
		extern "C" { fn cv_{{rust_local}}_delete(instance: {{rust_extern_mut}}); }
		unsafe { cv_{{rust_local}}_delete(self.as_raw_mut_{{rust_local}}()) };
	}
}

unsafe impl Send for {{rust_local}} {}

{{bases}}
{{impl}}
{{impls}}

