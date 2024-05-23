impl core::Vector<i8> {
	pub fn as_raw_VectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { i8,
	std_vectorLsigned_charG_new_const, std_vectorLsigned_charG_delete,
	std_vectorLsigned_charG_len_const, std_vectorLsigned_charG_isEmpty_const,
	std_vectorLsigned_charG_capacity_const, std_vectorLsigned_charG_shrinkToFit,
	std_vectorLsigned_charG_reserve_size_t, std_vectorLsigned_charG_remove_size_t,
	std_vectorLsigned_charG_swap_size_t_size_t, std_vectorLsigned_charG_clear,
	std_vectorLsigned_charG_get_const_size_t, std_vectorLsigned_charG_set_size_t_const_signed_char,
	std_vectorLsigned_charG_push_const_signed_char, std_vectorLsigned_charG_insert_size_t_const_signed_char,
}

vector_copy_non_bool! { i8,
	std_vectorLsigned_charG_data_const, std_vectorLsigned_charG_dataMut, cv_fromSlice_const_const_signed_charX_size_t,
	std_vectorLsigned_charG_clone_const,
}


