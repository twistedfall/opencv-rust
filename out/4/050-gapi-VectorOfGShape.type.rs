impl core::Vector<crate::gapi::GShape> {
	pub fn as_raw_VectorOfGShape(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GShape,
	std_vectorLcv_GShapeG_new_const, std_vectorLcv_GShapeG_delete,
	std_vectorLcv_GShapeG_len_const, std_vectorLcv_GShapeG_isEmpty_const,
	std_vectorLcv_GShapeG_capacity_const, std_vectorLcv_GShapeG_shrinkToFit,
	std_vectorLcv_GShapeG_reserve_size_t, std_vectorLcv_GShapeG_remove_size_t,
	std_vectorLcv_GShapeG_swap_size_t_size_t, std_vectorLcv_GShapeG_clear,
	std_vectorLcv_GShapeG_get_const_size_t, std_vectorLcv_GShapeG_set_size_t_const_GShape,
	std_vectorLcv_GShapeG_push_const_GShape, std_vectorLcv_GShapeG_insert_size_t_const_GShape,
}

vector_copy_non_bool! { crate::gapi::GShape,
	std_vectorLcv_GShapeG_data_const, std_vectorLcv_GShapeG_dataMut, cv_fromSlice_const_const_GShapeX_size_t,
	std_vectorLcv_GShapeG_clone_const,
}


