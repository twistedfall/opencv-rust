/// Constant methods for [{{rust_name_ref}}]
{{debug}}
pub trait {{rust_trait_local_const}}{{trait_bases_const}} {
	fn {{rust_as_raw_const}}(&self) -> {{rust_extern_const}};

	{{trait_const_methods}}
}

/// Mutable methods for [{{rust_name_ref}}]
pub trait {{rust_trait_local_mut}}{{trait_bases_mut}} {
	fn {{rust_as_raw_mut}}(&mut self) -> {{rust_extern_mut}};

	{{trait_mut_methods}}
}


