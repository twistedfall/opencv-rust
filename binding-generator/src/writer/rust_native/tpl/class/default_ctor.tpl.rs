/// Creates a default instance of the class by calling the default constructor
#[inline]
fn default() -> Self {
	unsafe { Self::from_raw(sys::{{extern_default_new}}()) }
}


