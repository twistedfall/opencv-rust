impl core::Vector<core::Ptr<crate::dnn::BackendWrapper>> {
	pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::dnn::BackendWrapper>,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_new_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_delete,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_len_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_isEmpty_const,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_capacity_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_shrinkToFit,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_remove_size_t,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_clear,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_set_size_t_const_PtrLBackendWrapperG,
	std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_push_const_PtrLBackendWrapperG, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_insert_size_t_const_PtrLBackendWrapperG,
}

vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendWrapper> }


