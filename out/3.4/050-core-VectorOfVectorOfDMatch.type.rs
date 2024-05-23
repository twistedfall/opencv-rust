impl core::Vector<core::Vector<core::DMatch>> {
	pub fn as_raw_VectorOfVectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::DMatch>,
	std_vectorLstd_vectorLcv_DMatchGG_new_const, std_vectorLstd_vectorLcv_DMatchGG_delete,
	std_vectorLstd_vectorLcv_DMatchGG_len_const, std_vectorLstd_vectorLcv_DMatchGG_isEmpty_const,
	std_vectorLstd_vectorLcv_DMatchGG_capacity_const, std_vectorLstd_vectorLcv_DMatchGG_shrinkToFit,
	std_vectorLstd_vectorLcv_DMatchGG_reserve_size_t, std_vectorLstd_vectorLcv_DMatchGG_remove_size_t,
	std_vectorLstd_vectorLcv_DMatchGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_DMatchGG_clear,
	std_vectorLstd_vectorLcv_DMatchGG_get_const_size_t, std_vectorLstd_vectorLcv_DMatchGG_set_size_t_const_vectorLDMatchG,
	std_vectorLstd_vectorLcv_DMatchGG_push_const_vectorLDMatchG, std_vectorLstd_vectorLcv_DMatchGG_insert_size_t_const_vectorLDMatchG,
}

vector_non_copy_or_bool! { clone core::Vector<core::DMatch> }


