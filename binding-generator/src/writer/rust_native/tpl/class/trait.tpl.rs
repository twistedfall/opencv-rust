{{const_trait_comment}}
{{debug}}
pub trait {{rust_trait_local_const}}{{trait_bases_const}} {
	fn as_raw_{{rust_local}}(&self) -> {{rust_extern_const}};

	{{trait_const_methods}}
}

{{mut_trait_comment}}
pub trait {{rust_trait_local}}{{trait_bases_mut}} {
	fn as_raw_mut_{{rust_local}}(&mut self) -> {{rust_extern_mut}};

	{{trait_mut_methods}}
}


