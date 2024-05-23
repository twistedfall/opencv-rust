impl core::Vector<core::DMatch> {
	pub fn as_raw_VectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::DMatch,
	std_vectorLcv_DMatchG_new_const, std_vectorLcv_DMatchG_delete,
	std_vectorLcv_DMatchG_len_const, std_vectorLcv_DMatchG_isEmpty_const,
	std_vectorLcv_DMatchG_capacity_const, std_vectorLcv_DMatchG_shrinkToFit,
	std_vectorLcv_DMatchG_reserve_size_t, std_vectorLcv_DMatchG_remove_size_t,
	std_vectorLcv_DMatchG_swap_size_t_size_t, std_vectorLcv_DMatchG_clear,
	std_vectorLcv_DMatchG_get_const_size_t, std_vectorLcv_DMatchG_set_size_t_const_DMatch,
	std_vectorLcv_DMatchG_push_const_DMatch, std_vectorLcv_DMatchG_insert_size_t_const_DMatch,
}

vector_copy_non_bool! { core::DMatch,
	std_vectorLcv_DMatchG_data_const, std_vectorLcv_DMatchG_dataMut, cv_fromSlice_const_const_DMatchX_size_t,
	std_vectorLcv_DMatchG_clone_const,
}


