impl core::Vector<crate::dnn::Arg> {
	pub fn as_raw_VectorOfArg(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::dnn::Arg,
	std_vectorLcv_dnn_ArgG_new_const, std_vectorLcv_dnn_ArgG_delete,
	std_vectorLcv_dnn_ArgG_len_const, std_vectorLcv_dnn_ArgG_isEmpty_const,
	std_vectorLcv_dnn_ArgG_capacity_const, std_vectorLcv_dnn_ArgG_shrinkToFit,
	std_vectorLcv_dnn_ArgG_reserve_size_t, std_vectorLcv_dnn_ArgG_remove_size_t,
	std_vectorLcv_dnn_ArgG_swap_size_t_size_t, std_vectorLcv_dnn_ArgG_clear,
	std_vectorLcv_dnn_ArgG_get_const_size_t, std_vectorLcv_dnn_ArgG_set_size_t_const_Arg,
	std_vectorLcv_dnn_ArgG_push_const_Arg, std_vectorLcv_dnn_ArgG_insert_size_t_const_Arg,
}

vector_non_copy_or_bool! { crate::dnn::Arg }

vector_boxed_ref! { crate::dnn::Arg }

vector_extern! { BoxedRef<'t, crate::dnn::Arg>,
	std_vectorLcv_dnn_ArgG_new_const, std_vectorLcv_dnn_ArgG_delete,
	std_vectorLcv_dnn_ArgG_len_const, std_vectorLcv_dnn_ArgG_isEmpty_const,
	std_vectorLcv_dnn_ArgG_capacity_const, std_vectorLcv_dnn_ArgG_shrinkToFit,
	std_vectorLcv_dnn_ArgG_reserve_size_t, std_vectorLcv_dnn_ArgG_remove_size_t,
	std_vectorLcv_dnn_ArgG_swap_size_t_size_t, std_vectorLcv_dnn_ArgG_clear,
	std_vectorLcv_dnn_ArgG_get_const_size_t, std_vectorLcv_dnn_ArgG_set_size_t_const_Arg,
	std_vectorLcv_dnn_ArgG_push_const_Arg, std_vectorLcv_dnn_ArgG_insert_size_t_const_Arg,
}


