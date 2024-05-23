impl core::Vector<core::Vector<core::KeyPoint>> {
	pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::KeyPoint>,
	std_vectorLstd_vectorLcv_KeyPointGG_new_const, std_vectorLstd_vectorLcv_KeyPointGG_delete,
	std_vectorLstd_vectorLcv_KeyPointGG_len_const, std_vectorLstd_vectorLcv_KeyPointGG_isEmpty_const,
	std_vectorLstd_vectorLcv_KeyPointGG_capacity_const, std_vectorLstd_vectorLcv_KeyPointGG_shrinkToFit,
	std_vectorLstd_vectorLcv_KeyPointGG_reserve_size_t, std_vectorLstd_vectorLcv_KeyPointGG_remove_size_t,
	std_vectorLstd_vectorLcv_KeyPointGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_KeyPointGG_clear,
	std_vectorLstd_vectorLcv_KeyPointGG_get_const_size_t, std_vectorLstd_vectorLcv_KeyPointGG_set_size_t_const_vectorLKeyPointG,
	std_vectorLstd_vectorLcv_KeyPointGG_push_const_vectorLKeyPointG, std_vectorLstd_vectorLcv_KeyPointGG_insert_size_t_const_vectorLKeyPointG,
}

vector_non_copy_or_bool! { clone core::Vector<core::KeyPoint> }


