impl Clone for {{rust_local}} {
	#[inline]
	fn clone(&self) -> Self {
		extern "C" { fn cv_{{rust_local}}_implicitClone_const_{{rust_local}}(val: extern_send!({{rust_local}})) -> extern_receive!({{rust_local}}: 'static); }
		unsafe { Self::from_raw(cv_{{rust_local}}_implicitClone_const_{{rust_local}}(self.as_raw_{{rust_local}}())) }
	}
}


