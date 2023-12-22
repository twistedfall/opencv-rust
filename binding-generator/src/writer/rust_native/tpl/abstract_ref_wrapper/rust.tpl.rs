impl {{rust_trait_const}} for types::AbstractRefMut<'static, {{rust_full}}> {
	#[inline] fn {{rust_as_raw_const}}(&self) -> extern_send!(Self) { self.as_raw() }
}

impl {{rust_trait_mut}} for types::AbstractRefMut<'static, {{rust_full}}> {
	#[inline] fn {{rust_as_raw_mut}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}


