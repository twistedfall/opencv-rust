impl core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>> {
	pub fn as_raw_VectorOfTupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfTupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_new_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_delete,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_len_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_isEmpty_const,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_capacity_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_shrinkToFit,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_reserve_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_remove_size_t,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_swap_size_t_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_clear,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_get_const_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_set_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG,
	std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_push_const_pairLcv_dnn_Backend__cv_dnn_TargetG, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_insert_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG,
}

vector_non_copy_or_bool! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> }


