impl core::Vector<c_char> {
	pub fn as_raw_VectorOfc_char(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfc_char(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

