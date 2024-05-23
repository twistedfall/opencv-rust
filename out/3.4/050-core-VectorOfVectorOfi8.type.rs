impl core::Vector<core::Vector<i8>> {
	pub fn as_raw_VectorOfVectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<i8>,
	std_vectorLstd_vectorLsigned_charGG_new_const, std_vectorLstd_vectorLsigned_charGG_delete,
	std_vectorLstd_vectorLsigned_charGG_len_const, std_vectorLstd_vectorLsigned_charGG_isEmpty_const,
	std_vectorLstd_vectorLsigned_charGG_capacity_const, std_vectorLstd_vectorLsigned_charGG_shrinkToFit,
	std_vectorLstd_vectorLsigned_charGG_reserve_size_t, std_vectorLstd_vectorLsigned_charGG_remove_size_t,
	std_vectorLstd_vectorLsigned_charGG_swap_size_t_size_t, std_vectorLstd_vectorLsigned_charGG_clear,
	std_vectorLstd_vectorLsigned_charGG_get_const_size_t, std_vectorLstd_vectorLsigned_charGG_set_size_t_const_vectorLsigned_charG,
	std_vectorLstd_vectorLsigned_charGG_push_const_vectorLsigned_charG, std_vectorLstd_vectorLsigned_charGG_insert_size_t_const_vectorLsigned_charG,
}

vector_non_copy_or_bool! { clone core::Vector<i8> }


