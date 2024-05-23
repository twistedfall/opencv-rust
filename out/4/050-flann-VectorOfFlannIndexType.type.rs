impl core::Vector<crate::flann::FlannIndexType> {
	pub fn as_raw_VectorOfFlannIndexType(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfFlannIndexType(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::flann::FlannIndexType,
	std_vectorLcv_flann_FlannIndexTypeG_new_const, std_vectorLcv_flann_FlannIndexTypeG_delete,
	std_vectorLcv_flann_FlannIndexTypeG_len_const, std_vectorLcv_flann_FlannIndexTypeG_isEmpty_const,
	std_vectorLcv_flann_FlannIndexTypeG_capacity_const, std_vectorLcv_flann_FlannIndexTypeG_shrinkToFit,
	std_vectorLcv_flann_FlannIndexTypeG_reserve_size_t, std_vectorLcv_flann_FlannIndexTypeG_remove_size_t,
	std_vectorLcv_flann_FlannIndexTypeG_swap_size_t_size_t, std_vectorLcv_flann_FlannIndexTypeG_clear,
	std_vectorLcv_flann_FlannIndexTypeG_get_const_size_t, std_vectorLcv_flann_FlannIndexTypeG_set_size_t_const_FlannIndexType,
	std_vectorLcv_flann_FlannIndexTypeG_push_const_FlannIndexType, std_vectorLcv_flann_FlannIndexTypeG_insert_size_t_const_FlannIndexType,
}

vector_copy_non_bool! { crate::flann::FlannIndexType,
	std_vectorLcv_flann_FlannIndexTypeG_data_const, std_vectorLcv_flann_FlannIndexTypeG_dataMut, cv_fromSlice_const_const_FlannIndexTypeX_size_t,
	std_vectorLcv_flann_FlannIndexTypeG_clone_const,
}


