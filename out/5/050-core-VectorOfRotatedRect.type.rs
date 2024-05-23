impl core::Vector<core::RotatedRect> {
	pub fn as_raw_VectorOfRotatedRect(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::RotatedRect,
	std_vectorLcv_RotatedRectG_new_const, std_vectorLcv_RotatedRectG_delete,
	std_vectorLcv_RotatedRectG_len_const, std_vectorLcv_RotatedRectG_isEmpty_const,
	std_vectorLcv_RotatedRectG_capacity_const, std_vectorLcv_RotatedRectG_shrinkToFit,
	std_vectorLcv_RotatedRectG_reserve_size_t, std_vectorLcv_RotatedRectG_remove_size_t,
	std_vectorLcv_RotatedRectG_swap_size_t_size_t, std_vectorLcv_RotatedRectG_clear,
	std_vectorLcv_RotatedRectG_get_const_size_t, std_vectorLcv_RotatedRectG_set_size_t_const_RotatedRect,
	std_vectorLcv_RotatedRectG_push_const_RotatedRect, std_vectorLcv_RotatedRectG_insert_size_t_const_RotatedRect,
}

vector_copy_non_bool! { core::RotatedRect,
	std_vectorLcv_RotatedRectG_data_const, std_vectorLcv_RotatedRectG_dataMut, cv_fromSlice_const_const_RotatedRectX_size_t,
	std_vectorLcv_RotatedRectG_clone_const,
}


