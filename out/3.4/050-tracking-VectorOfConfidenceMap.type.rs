impl core::Vector<crate::tracking::ConfidenceMap> {
	pub fn as_raw_VectorOfConfidenceMap(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfConfidenceMap(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::tracking::ConfidenceMap,
	std_vectorLcv_ConfidenceMapG_new_const, std_vectorLcv_ConfidenceMapG_delete,
	std_vectorLcv_ConfidenceMapG_len_const, std_vectorLcv_ConfidenceMapG_isEmpty_const,
	std_vectorLcv_ConfidenceMapG_capacity_const, std_vectorLcv_ConfidenceMapG_shrinkToFit,
	std_vectorLcv_ConfidenceMapG_reserve_size_t, std_vectorLcv_ConfidenceMapG_remove_size_t,
	std_vectorLcv_ConfidenceMapG_swap_size_t_size_t, std_vectorLcv_ConfidenceMapG_clear,
	std_vectorLcv_ConfidenceMapG_get_const_size_t, std_vectorLcv_ConfidenceMapG_set_size_t_const_ConfidenceMap,
	std_vectorLcv_ConfidenceMapG_push_const_ConfidenceMap, std_vectorLcv_ConfidenceMapG_insert_size_t_const_ConfidenceMap,
}

vector_non_copy_or_bool! { crate::tracking::ConfidenceMap }


