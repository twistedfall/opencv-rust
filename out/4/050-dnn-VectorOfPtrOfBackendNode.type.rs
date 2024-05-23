impl core::Vector<core::Ptr<crate::dnn::BackendNode>> {
	pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::dnn::BackendNode>,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_new_const, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_delete,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_len_const, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_isEmpty_const,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_capacity_const, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_shrinkToFit,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_remove_size_t,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_clear,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_set_size_t_const_PtrLBackendNodeG,
	std_vectorLcv_PtrLcv_dnn_BackendNodeGG_push_const_PtrLBackendNodeG, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_insert_size_t_const_PtrLBackendNodeG,
}

vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendNode> }


