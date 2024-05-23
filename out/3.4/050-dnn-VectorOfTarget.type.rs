impl core::Vector<crate::dnn::Target> {
	pub fn as_raw_VectorOfTarget(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfTarget(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::dnn::Target,
	std_vectorLcv_dnn_TargetG_new_const, std_vectorLcv_dnn_TargetG_delete,
	std_vectorLcv_dnn_TargetG_len_const, std_vectorLcv_dnn_TargetG_isEmpty_const,
	std_vectorLcv_dnn_TargetG_capacity_const, std_vectorLcv_dnn_TargetG_shrinkToFit,
	std_vectorLcv_dnn_TargetG_reserve_size_t, std_vectorLcv_dnn_TargetG_remove_size_t,
	std_vectorLcv_dnn_TargetG_swap_size_t_size_t, std_vectorLcv_dnn_TargetG_clear,
	std_vectorLcv_dnn_TargetG_get_const_size_t, std_vectorLcv_dnn_TargetG_set_size_t_const_Target,
	std_vectorLcv_dnn_TargetG_push_const_Target, std_vectorLcv_dnn_TargetG_insert_size_t_const_Target,
}

vector_copy_non_bool! { crate::dnn::Target,
	std_vectorLcv_dnn_TargetG_data_const, std_vectorLcv_dnn_TargetG_dataMut, cv_fromSlice_const_const_TargetX_size_t,
	std_vectorLcv_dnn_TargetG_clone_const,
}


