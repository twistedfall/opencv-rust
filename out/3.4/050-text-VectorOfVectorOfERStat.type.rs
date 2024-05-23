impl core::Vector<core::Vector<crate::text::ERStat>> {
	pub fn as_raw_VectorOfVectorOfERStat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfERStat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<crate::text::ERStat>,
	std_vectorLstd_vectorLcv_text_ERStatGG_new_const, std_vectorLstd_vectorLcv_text_ERStatGG_delete,
	std_vectorLstd_vectorLcv_text_ERStatGG_len_const, std_vectorLstd_vectorLcv_text_ERStatGG_isEmpty_const,
	std_vectorLstd_vectorLcv_text_ERStatGG_capacity_const, std_vectorLstd_vectorLcv_text_ERStatGG_shrinkToFit,
	std_vectorLstd_vectorLcv_text_ERStatGG_reserve_size_t, std_vectorLstd_vectorLcv_text_ERStatGG_remove_size_t,
	std_vectorLstd_vectorLcv_text_ERStatGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_text_ERStatGG_clear,
	std_vectorLstd_vectorLcv_text_ERStatGG_get_const_size_t, std_vectorLstd_vectorLcv_text_ERStatGG_set_size_t_const_vectorLERStatG,
	std_vectorLstd_vectorLcv_text_ERStatGG_push_const_vectorLERStatG, std_vectorLstd_vectorLcv_text_ERStatGG_insert_size_t_const_vectorLERStatG,
}

vector_non_copy_or_bool! { core::Vector<crate::text::ERStat> }


