

pub fn to_slice(&self) -> &[{{inner_rust_full}}] {
	extern "C" { fn cv_{{rust_local}}_data(instance: {{rust_extern_const}}) -> *const {{inner_rust_full}}; }
	unsafe {
		let data = cv_{{rust_local}}_data(self.as_raw_{{rust_local}}());
		::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
	}
}

