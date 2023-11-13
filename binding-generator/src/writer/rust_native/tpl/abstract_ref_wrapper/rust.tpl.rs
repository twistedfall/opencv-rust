impl {{rust_const_trait}} for types::AbstractRefMut<'static, {{rust_full}}> {
	#[inline] fn as_raw_{{rust_local}}(&self) -> extern_send!(Self) { self.as_raw() }
}

impl {{rust_mut_trait}} for types::AbstractRefMut<'static, {{rust_full}}> {
	#[inline] fn as_raw_mut_{{rust_local}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}


