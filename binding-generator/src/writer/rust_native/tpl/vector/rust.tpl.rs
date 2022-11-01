pub type {{rust_localalias}} = {{rust_full}};

impl {{rust_localalias}} {
	pub fn as_raw_{{rust_localalias}}(&self) -> {{rust_extern_const}} { self.as_raw() }
	pub fn as_raw_mut_{{rust_localalias}}(&mut self) -> {{rust_extern_mut}} { self.as_raw_mut() }
}
{{extern}}
{{additional_methods}}
{{impls}}


