impl core::MatOpTraitConst for types::AbstractRefMut<'static, core::MatOp> {
	#[inline] fn as_raw_MatOp(&self) -> extern_send!(Self) { self.as_raw() }
}

impl core::MatOpTrait for types::AbstractRefMut<'static, core::MatOp> {
	#[inline] fn as_raw_mut_MatOp(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

