

#[inline]
fn to_vec(&self) -> Vec<Self::Storage> {
	self.to_slice().to_vec()
}
