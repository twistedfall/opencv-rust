impl core::Vector<crate::dnn::MatShape> {
	pub fn as_raw_VectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

