impl core::Vector<core::Ptr<crate::dnn::Graph>> {
	pub fn as_raw_VectorOfPtrOfGraph(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfGraph(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::dnn::Graph>,
	std_vectorLcv_PtrLcv_dnn_GraphGG_new_const, std_vectorLcv_PtrLcv_dnn_GraphGG_delete,
	std_vectorLcv_PtrLcv_dnn_GraphGG_len_const, std_vectorLcv_PtrLcv_dnn_GraphGG_isEmpty_const,
	std_vectorLcv_PtrLcv_dnn_GraphGG_capacity_const, std_vectorLcv_PtrLcv_dnn_GraphGG_shrinkToFit,
	std_vectorLcv_PtrLcv_dnn_GraphGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_GraphGG_remove_size_t,
	std_vectorLcv_PtrLcv_dnn_GraphGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_GraphGG_clear,
	std_vectorLcv_PtrLcv_dnn_GraphGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_GraphGG_set_size_t_const_PtrLGraphG,
	std_vectorLcv_PtrLcv_dnn_GraphGG_push_const_PtrLGraphG, std_vectorLcv_PtrLcv_dnn_GraphGG_insert_size_t_const_PtrLGraphG,
}

vector_non_copy_or_bool! { core::Ptr<crate::dnn::Graph> }


