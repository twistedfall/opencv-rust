impl {{rust_const_full}} for types::AbstractRefMut<'static, dyn {{rust_full}}> {
	#[inline] fn as_raw_{{rust_local}}(&self) -> extern_send!(Self) { self.as_raw() }
}

impl {{rust_full}} for types::AbstractRefMut<'static, dyn {{rust_full}}> {
	#[inline] fn as_raw_mut_{{rust_local}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}


