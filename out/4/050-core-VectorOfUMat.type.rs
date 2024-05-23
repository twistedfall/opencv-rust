impl core::Vector<core::UMat> {
	pub fn as_raw_VectorOfUMat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfUMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::UMat,
	std_vectorLcv_UMatG_new_const, std_vectorLcv_UMatG_delete,
	std_vectorLcv_UMatG_len_const, std_vectorLcv_UMatG_isEmpty_const,
	std_vectorLcv_UMatG_capacity_const, std_vectorLcv_UMatG_shrinkToFit,
	std_vectorLcv_UMatG_reserve_size_t, std_vectorLcv_UMatG_remove_size_t,
	std_vectorLcv_UMatG_swap_size_t_size_t, std_vectorLcv_UMatG_clear,
	std_vectorLcv_UMatG_get_const_size_t, std_vectorLcv_UMatG_set_size_t_const_UMat,
	std_vectorLcv_UMatG_push_const_UMat, std_vectorLcv_UMatG_insert_size_t_const_UMat,
}

vector_non_copy_or_bool! { clone core::UMat }

vector_boxed_ref! { core::UMat }

vector_extern! { BoxedRef<'t, core::UMat>,
	std_vectorLcv_UMatG_new_const, std_vectorLcv_UMatG_delete,
	std_vectorLcv_UMatG_len_const, std_vectorLcv_UMatG_isEmpty_const,
	std_vectorLcv_UMatG_capacity_const, std_vectorLcv_UMatG_shrinkToFit,
	std_vectorLcv_UMatG_reserve_size_t, std_vectorLcv_UMatG_remove_size_t,
	std_vectorLcv_UMatG_swap_size_t_size_t, std_vectorLcv_UMatG_clear,
	std_vectorLcv_UMatG_get_const_size_t, std_vectorLcv_UMatG_set_size_t_const_UMat,
	std_vectorLcv_UMatG_push_const_UMat, std_vectorLcv_UMatG_insert_size_t_const_UMat,
}


