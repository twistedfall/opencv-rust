impl core::Vector<size_t> {
	pub fn as_raw_VectorOfsize_t(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfsize_t(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { size_t,
	std_vectorLsize_tG_new_const, std_vectorLsize_tG_delete,
	std_vectorLsize_tG_len_const, std_vectorLsize_tG_isEmpty_const,
	std_vectorLsize_tG_capacity_const, std_vectorLsize_tG_shrinkToFit,
	std_vectorLsize_tG_reserve_size_t, std_vectorLsize_tG_remove_size_t,
	std_vectorLsize_tG_swap_size_t_size_t, std_vectorLsize_tG_clear,
	std_vectorLsize_tG_get_const_size_t, std_vectorLsize_tG_set_size_t_const_size_t,
	std_vectorLsize_tG_push_const_size_t, std_vectorLsize_tG_insert_size_t_const_size_t,
}

vector_copy_non_bool! { size_t,
	std_vectorLsize_tG_data_const, std_vectorLsize_tG_dataMut, cv_fromSlice_const_const_size_tX_size_t,
	std_vectorLsize_tG_clone_const,
}


