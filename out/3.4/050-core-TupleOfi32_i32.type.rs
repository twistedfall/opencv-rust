impl core::Tuple<(i32, i32)> {
	pub fn as_raw_TupleOfi32_i32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfi32_i32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (i32, i32),
	std_pairLint__intG_new_const_int_int, std_pairLint__intG_delete,
	0 = arg: i32, get_0 via std_pairLint__intG_get_0_const,
	1 = arg_1: i32, get_1 via std_pairLint__intG_get_1_const
}

