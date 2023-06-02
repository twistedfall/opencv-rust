fn default() -> Self {
	extern "C" { fn cv_{{rust_local}}_defaultNew_const() -> extern_receive!({{rust_local}}: 'static); }
	unsafe { Self::from_raw(cv_{{rust_local}}_defaultNew_const()) }
}


