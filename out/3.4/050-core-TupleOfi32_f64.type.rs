impl core::Tuple<(i32, f64)> {
	pub fn as_raw_TupleOfi32_f64(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_TupleOfi32_f64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { (i32, f64),
	std_pairLint__doubleG_new_const_int_double, std_pairLint__doubleG_delete,
	0 = arg: i32, get_0 via std_pairLint__doubleG_get_0_const,
	1 = arg_1: f64, get_1 via std_pairLint__doubleG_get_1_const
}

