impl core::Vector<core::Ptr<crate::dnn::Layer>> {
	pub fn as_raw_VectorOfPtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::dnn::Layer>,
	std_vectorLcv_PtrLcv_dnn_LayerGG_new_const, std_vectorLcv_PtrLcv_dnn_LayerGG_delete,
	std_vectorLcv_PtrLcv_dnn_LayerGG_len_const, std_vectorLcv_PtrLcv_dnn_LayerGG_isEmpty_const,
	std_vectorLcv_PtrLcv_dnn_LayerGG_capacity_const, std_vectorLcv_PtrLcv_dnn_LayerGG_shrinkToFit,
	std_vectorLcv_PtrLcv_dnn_LayerGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_remove_size_t,
	std_vectorLcv_PtrLcv_dnn_LayerGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_clear,
	std_vectorLcv_PtrLcv_dnn_LayerGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_set_size_t_const_PtrLLayerG,
	std_vectorLcv_PtrLcv_dnn_LayerGG_push_const_PtrLLayerG, std_vectorLcv_PtrLcv_dnn_LayerGG_insert_size_t_const_PtrLLayerG,
}

vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }


