impl core::Tuple<(i32, f32)> {
	pub fn as_raw_TupleOfi32_f32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfi32_f32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (i32, f32),
	std_pairLint__floatG_new_const_int_float, std_pairLint__floatG_delete,
	0 = arg: i32, get_0 via std_pairLint__floatG_get_0_const,
	1 = arg_1: f32, get_1 via std_pairLint__floatG_get_1_const
}

