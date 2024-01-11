impl {{rust_trait_const}} for types::AbstractRefMut<'static, {{rust_full}}> {
	#[inline] fn as_raw_{{rust_local}}(&self) -> extern_send!(Self) { self.as_raw() }
}

impl {{rust_trait_mut}} for types::AbstractRefMut<'static, {{rust_full}}> {
	#[inline] fn as_raw_mut_{{rust_local}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}


