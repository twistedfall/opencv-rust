/// Constant methods for [{{rust_name_ref}}]
{{debug}}
pub trait {{rust_trait_local_const}}{{trait_bases_const}} {
	fn as_raw_{{rust_local}}(&self) -> {{rust_extern_const}};

	{{trait_const_methods}}
}

/// Mutable methods for [{{rust_name_ref}}]
pub trait {{rust_trait_local_mut}}{{trait_bases_mut}} {
	fn as_raw_mut_{{rust_local}}(&mut self) -> {{rust_extern_mut}};

	{{trait_mut_methods}}
}


