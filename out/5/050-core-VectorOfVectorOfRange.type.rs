impl core::Vector<core::Vector<core::Range>> {
	pub fn as_raw_VectorOfVectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Range>,
	std_vectorLstd_vectorLcv_RangeGG_new_const, std_vectorLstd_vectorLcv_RangeGG_delete,
	std_vectorLstd_vectorLcv_RangeGG_len_const, std_vectorLstd_vectorLcv_RangeGG_isEmpty_const,
	std_vectorLstd_vectorLcv_RangeGG_capacity_const, std_vectorLstd_vectorLcv_RangeGG_shrinkToFit,
	std_vectorLstd_vectorLcv_RangeGG_reserve_size_t, std_vectorLstd_vectorLcv_RangeGG_remove_size_t,
	std_vectorLstd_vectorLcv_RangeGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_RangeGG_clear,
	std_vectorLstd_vectorLcv_RangeGG_get_const_size_t, std_vectorLstd_vectorLcv_RangeGG_set_size_t_const_vectorLRangeG,
	std_vectorLstd_vectorLcv_RangeGG_push_const_vectorLRangeG, std_vectorLstd_vectorLcv_RangeGG_insert_size_t_const_vectorLRangeG,
}

vector_non_copy_or_bool! { core::Vector<core::Range> }


