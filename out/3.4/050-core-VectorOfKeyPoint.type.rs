impl core::Vector<core::KeyPoint> {
	pub fn as_raw_VectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::KeyPoint,
	std_vectorLcv_KeyPointG_new_const, std_vectorLcv_KeyPointG_delete,
	std_vectorLcv_KeyPointG_len_const, std_vectorLcv_KeyPointG_isEmpty_const,
	std_vectorLcv_KeyPointG_capacity_const, std_vectorLcv_KeyPointG_shrinkToFit,
	std_vectorLcv_KeyPointG_reserve_size_t, std_vectorLcv_KeyPointG_remove_size_t,
	std_vectorLcv_KeyPointG_swap_size_t_size_t, std_vectorLcv_KeyPointG_clear,
	std_vectorLcv_KeyPointG_get_const_size_t, std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint,
	std_vectorLcv_KeyPointG_push_const_KeyPoint, std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint,
}

vector_non_copy_or_bool! { clone core::KeyPoint }

vector_boxed_ref! { core::KeyPoint }

vector_extern! { BoxedRef<'t, core::KeyPoint>,
	std_vectorLcv_KeyPointG_new_const, std_vectorLcv_KeyPointG_delete,
	std_vectorLcv_KeyPointG_len_const, std_vectorLcv_KeyPointG_isEmpty_const,
	std_vectorLcv_KeyPointG_capacity_const, std_vectorLcv_KeyPointG_shrinkToFit,
	std_vectorLcv_KeyPointG_reserve_size_t, std_vectorLcv_KeyPointG_remove_size_t,
	std_vectorLcv_KeyPointG_swap_size_t_size_t, std_vectorLcv_KeyPointG_clear,
	std_vectorLcv_KeyPointG_get_const_size_t, std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint,
	std_vectorLcv_KeyPointG_push_const_KeyPoint, std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint,
}


