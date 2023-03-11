impl Clone for {{rust_local}} {
	#[inline]
	fn clone(&self) -> Self {
		extern "C" {
			fn cv_{{rust_local}}_implicit_clone(val: extern_send!({{rust_local}})) -> extern_receive!({{rust_local}}: 'static);
		}
		unsafe { Self::from_raw(cv_{{rust_local}}_implicit_clone(self.as_raw_{{rust_local}}())) }
	}
}


