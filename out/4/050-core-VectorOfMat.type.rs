impl core::Vector<core::Mat> {
	pub fn as_raw_VectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Mat,
	std_vectorLcv_MatG_new_const, std_vectorLcv_MatG_delete,
	std_vectorLcv_MatG_len_const, std_vectorLcv_MatG_isEmpty_const,
	std_vectorLcv_MatG_capacity_const, std_vectorLcv_MatG_shrinkToFit,
	std_vectorLcv_MatG_reserve_size_t, std_vectorLcv_MatG_remove_size_t,
	std_vectorLcv_MatG_swap_size_t_size_t, std_vectorLcv_MatG_clear,
	std_vectorLcv_MatG_get_const_size_t, std_vectorLcv_MatG_set_size_t_const_Mat,
	std_vectorLcv_MatG_push_const_Mat, std_vectorLcv_MatG_insert_size_t_const_Mat,
}

vector_non_copy_or_bool! { clone core::Mat }

vector_boxed_ref! { core::Mat }

vector_extern! { BoxedRef<'t, core::Mat>,
	std_vectorLcv_MatG_new_const, std_vectorLcv_MatG_delete,
	std_vectorLcv_MatG_len_const, std_vectorLcv_MatG_isEmpty_const,
	std_vectorLcv_MatG_capacity_const, std_vectorLcv_MatG_shrinkToFit,
	std_vectorLcv_MatG_reserve_size_t, std_vectorLcv_MatG_remove_size_t,
	std_vectorLcv_MatG_swap_size_t_size_t, std_vectorLcv_MatG_clear,
	std_vectorLcv_MatG_get_const_size_t, std_vectorLcv_MatG_set_size_t_const_Mat,
	std_vectorLcv_MatG_push_const_Mat, std_vectorLcv_MatG_insert_size_t_const_Mat,
}


