impl {{base_rust_full_const}} for {{rust_local}}{{rust_elided_lt}} {
	#[inline] fn {{rust_as_raw_const}}(&self) -> {{base_rust_extern_const}} { self.as_raw() }
}

impl {{base_rust_full_mut}} for {{rust_local}}{{rust_elided_lt}} {
	#[inline] fn {{rust_as_raw_mut}}(&mut self) -> {{base_rust_extern_mut}} { self.as_raw_mut() }
}

boxed_ref! { {{rust_local}}{{rust_elided_lt}}, {{base_rust_full_const}}, {{rust_as_raw_const}}, {{base_rust_full_mut}}, {{rust_as_raw_mut}} }


