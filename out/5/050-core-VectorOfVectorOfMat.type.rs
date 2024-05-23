impl core::Vector<core::Vector<core::Mat>> {
	pub fn as_raw_VectorOfVectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Mat>,
	std_vectorLstd_vectorLcv_MatGG_new_const, std_vectorLstd_vectorLcv_MatGG_delete,
	std_vectorLstd_vectorLcv_MatGG_len_const, std_vectorLstd_vectorLcv_MatGG_isEmpty_const,
	std_vectorLstd_vectorLcv_MatGG_capacity_const, std_vectorLstd_vectorLcv_MatGG_shrinkToFit,
	std_vectorLstd_vectorLcv_MatGG_reserve_size_t, std_vectorLstd_vectorLcv_MatGG_remove_size_t,
	std_vectorLstd_vectorLcv_MatGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_MatGG_clear,
	std_vectorLstd_vectorLcv_MatGG_get_const_size_t, std_vectorLstd_vectorLcv_MatGG_set_size_t_const_vectorLMatG,
	std_vectorLstd_vectorLcv_MatGG_push_const_vectorLMatG, std_vectorLstd_vectorLcv_MatGG_insert_size_t_const_vectorLMatG,
}

vector_non_copy_or_bool! { clone core::Vector<core::Mat> }


