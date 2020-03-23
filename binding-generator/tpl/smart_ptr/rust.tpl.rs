pub struct {{rust_local}} {
	pub(crate) ptr: {{rust_extern}}
}

impl {{rust_local}} {
	pub fn as_raw_{{rust_local}}(&self) -> {{rust_extern}} { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: {{rust_extern}}) -> Self {
		Self { ptr }
	}

	/// Get raw pointer to the inner object
	pub fn get_inner_ptr(&self) -> {{rust_extern}} {
		extern "C" { fn cv_{{rust_local}}_get_inner_ptr(instance: {{rust_extern}}) -> {{rust_extern}}; }
		unsafe { cv_{{rust_local}}_get_inner_ptr(self.as_raw_{{rust_local}}()) }
	}
}

impl Drop for {{rust_local}} {
	fn drop(&mut self) {
		extern "C" { fn cv_{{rust_local}}_delete(instance: {{rust_extern}}); }
		unsafe { cv_{{rust_local}}_delete(self.as_raw_{{rust_local}}()) };
	}
}

unsafe impl Send for {{rust_local}} {}

{{impls}}

