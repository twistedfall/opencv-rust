{{doc_comment}}
{{debug}}
pub struct {{rust_local}} {
	pub(crate) ptr: {{rust_extern}}
}

impl Drop for {{rust_local}} {
	fn drop(&mut self) {
		extern "C" { fn cv_{{rust_local}}_delete(instance: {{rust_extern}}); }
		unsafe { cv_{{rust_local}}_delete(self.as_raw_{{rust_local}}()) };
	}
}

impl {{rust_local}} {
	pub fn as_raw_{{rust_local}}(&self) -> {{rust_extern}} { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: {{rust_extern}}) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for {{rust_local}} {}

{{bases}}
{{impl}}


