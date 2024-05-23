impl core::Vector<core::Tuple<(i32, f64)>> {
	pub fn as_raw_VectorOfTupleOfi32_f64(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfTupleOfi32_f64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Tuple<(i32, f64)>,
	std_vectorLstd_pairLint__doubleGG_new_const, std_vectorLstd_pairLint__doubleGG_delete,
	std_vectorLstd_pairLint__doubleGG_len_const, std_vectorLstd_pairLint__doubleGG_isEmpty_const,
	std_vectorLstd_pairLint__doubleGG_capacity_const, std_vectorLstd_pairLint__doubleGG_shrinkToFit,
	std_vectorLstd_pairLint__doubleGG_reserve_size_t, std_vectorLstd_pairLint__doubleGG_remove_size_t,
	std_vectorLstd_pairLint__doubleGG_swap_size_t_size_t, std_vectorLstd_pairLint__doubleGG_clear,
	std_vectorLstd_pairLint__doubleGG_get_const_size_t, std_vectorLstd_pairLint__doubleGG_set_size_t_const_pairLint__doubleG,
	std_vectorLstd_pairLint__doubleGG_push_const_pairLint__doubleG, std_vectorLstd_pairLint__doubleGG_insert_size_t_const_pairLint__doubleG,
}

vector_non_copy_or_bool! { core::Tuple<(i32, f64)> }


