impl core::Vector<crate::ximgproc::Box> {
	pub fn as_raw_VectorOfBox(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfBox(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::ximgproc::Box,
	std_vectorLcv_ximgproc_BoxG_new_const, std_vectorLcv_ximgproc_BoxG_delete,
	std_vectorLcv_ximgproc_BoxG_len_const, std_vectorLcv_ximgproc_BoxG_isEmpty_const,
	std_vectorLcv_ximgproc_BoxG_capacity_const, std_vectorLcv_ximgproc_BoxG_shrinkToFit,
	std_vectorLcv_ximgproc_BoxG_reserve_size_t, std_vectorLcv_ximgproc_BoxG_remove_size_t,
	std_vectorLcv_ximgproc_BoxG_swap_size_t_size_t, std_vectorLcv_ximgproc_BoxG_clear,
	std_vectorLcv_ximgproc_BoxG_get_const_size_t, std_vectorLcv_ximgproc_BoxG_set_size_t_const_Box,
	std_vectorLcv_ximgproc_BoxG_push_const_Box, std_vectorLcv_ximgproc_BoxG_insert_size_t_const_Box,
}

vector_copy_non_bool! { crate::ximgproc::Box,
	std_vectorLcv_ximgproc_BoxG_data_const, std_vectorLcv_ximgproc_BoxG_dataMut, cv_fromSlice_const_const_BoxX_size_t,
	std_vectorLcv_ximgproc_BoxG_clone_const,
}


