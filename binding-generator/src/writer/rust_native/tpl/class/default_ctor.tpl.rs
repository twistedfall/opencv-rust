fn default() -> Self {
	unsafe { Self::from_raw(sys::{{extern_default_new}}()) }
}


