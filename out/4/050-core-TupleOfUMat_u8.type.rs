impl core::Tuple<(core::UMat, u8)> {
	pub fn as_raw_TupleOfUMat_u8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfUMat_u8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (core::UMat, u8),
	std_pairLcv_UMat__unsigned_charG_new_const_UMat_unsigned_char, std_pairLcv_UMat__unsigned_charG_delete,
	0 = arg: core::UMat, get_0 via std_pairLcv_UMat__unsigned_charG_get_0_const,
	1 = arg_1: u8, get_1 via std_pairLcv_UMat__unsigned_charG_get_1_const
}

