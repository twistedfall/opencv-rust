pub type {{rust_localalias}} = {{rust_full}};

ptr_extern! { {{inner_rust_full}},
	cv_{{rust_localalias}}_delete, cv_{{rust_localalias}}_get_inner_ptr
}

impl {{rust_localalias}} {
	#[inline] pub fn as_raw_{{rust_localalias}}(&self) -> {{rust_extern_const}} { self.as_raw() }
	#[inline] pub fn as_raw_mut_{{rust_localalias}}(&mut self) -> {{rust_extern_mut}} { self.as_raw_mut() }
}

{{impls}}

