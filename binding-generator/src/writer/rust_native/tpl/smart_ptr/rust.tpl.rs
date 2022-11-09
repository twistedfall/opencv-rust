pub type {{rust_localalias}} = {{rust_full}};

ptr_extern! { {{inner_rust_full}},
	cv_{{rust_localalias}}_delete, cv_{{rust_localalias}}_get_inner_ptr, cv_{{rust_localalias}}_get_inner_ptr_mut
}

{{ctor}}
impl {{rust_full}} {
	#[inline] pub fn as_raw_{{rust_localalias}}(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_{{rust_localalias}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

{{impls}}

