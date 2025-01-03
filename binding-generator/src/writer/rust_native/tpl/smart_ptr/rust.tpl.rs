ptr_extern! { {{inner_rust_full}},
	{{extern_new_null}}, {{extern_delete}}, {{extern_get_inner_ptr}}, {{extern_get_inner_ptr_mut}}
}

{{ctor}}
impl {{rust_full}} {
	#[inline] pub fn {{rust_as_raw_const}}(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn {{rust_as_raw_mut}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

{{impls}}

