pub type {{rust_localalias}} = {{rust_full}};

impl {{rust_full}} {
	pub fn as_raw_{{rust_localalias}}(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_{{rust_localalias}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}
{{impls}}


