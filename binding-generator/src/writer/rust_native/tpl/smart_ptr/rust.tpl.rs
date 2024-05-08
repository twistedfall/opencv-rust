#[deprecated = "Use the the non-alias form `{{rust_full}}` instead, removal in Nov 2024"]
pub type {{rust_localalias}} = {{rust_full}};

ptr_extern! { {{inner_rust_full}},
	{{extern_delete}}, {{extern_get_inner_ptr}}, {{extern_get_inner_ptr_mut}}
}

{{ctor}}
impl {{rust_full}} {
	#[inline] pub fn {{rust_as_raw_const}}(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn {{rust_as_raw_mut}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

{{impls}}

