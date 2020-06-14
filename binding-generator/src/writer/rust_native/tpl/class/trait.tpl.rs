{{doc_comment}}
{{debug}}
pub trait {{rust_trait_local}}{{bases}} {
	fn as_raw_{{rust_local}}(&self) -> {{rust_extern_const}};
	fn as_raw_mut_{{rust_local}}(&mut self) -> {{rust_extern_mut}};

	{{trait_methods}}
}

{{dyn_impl}}

