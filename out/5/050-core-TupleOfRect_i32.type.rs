impl core::Tuple<(core::Rect, i32)> {
	pub fn as_raw_TupleOfRect_i32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfRect_i32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (core::Rect, i32),
	std_pairLcv_Rect__intG_new_const_Rect_int, std_pairLcv_Rect__intG_delete,
	0 = arg: core::Rect, get_0 via std_pairLcv_Rect__intG_get_0_const,
	1 = arg_1: i32, get_1 via std_pairLcv_Rect__intG_get_1_const
}

