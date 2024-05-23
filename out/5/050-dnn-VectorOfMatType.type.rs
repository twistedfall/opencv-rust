impl core::Vector<crate::dnn::MatType> {
	pub fn as_raw_VectorOfMatType(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfMatType(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

