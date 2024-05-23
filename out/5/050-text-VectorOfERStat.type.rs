impl core::Vector<crate::text::ERStat> {
	pub fn as_raw_VectorOfERStat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfERStat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::text::ERStat,
	std_vectorLcv_text_ERStatG_new_const, std_vectorLcv_text_ERStatG_delete,
	std_vectorLcv_text_ERStatG_len_const, std_vectorLcv_text_ERStatG_isEmpty_const,
	std_vectorLcv_text_ERStatG_capacity_const, std_vectorLcv_text_ERStatG_shrinkToFit,
	std_vectorLcv_text_ERStatG_reserve_size_t, std_vectorLcv_text_ERStatG_remove_size_t,
	std_vectorLcv_text_ERStatG_swap_size_t_size_t, std_vectorLcv_text_ERStatG_clear,
	std_vectorLcv_text_ERStatG_get_const_size_t, std_vectorLcv_text_ERStatG_set_size_t_const_ERStat,
	std_vectorLcv_text_ERStatG_push_const_ERStat, std_vectorLcv_text_ERStatG_insert_size_t_const_ERStat,
}

vector_non_copy_or_bool! { crate::text::ERStat }

vector_boxed_ref! { crate::text::ERStat }

vector_extern! { BoxedRef<'t, crate::text::ERStat>,
	std_vectorLcv_text_ERStatG_new_const, std_vectorLcv_text_ERStatG_delete,
	std_vectorLcv_text_ERStatG_len_const, std_vectorLcv_text_ERStatG_isEmpty_const,
	std_vectorLcv_text_ERStatG_capacity_const, std_vectorLcv_text_ERStatG_shrinkToFit,
	std_vectorLcv_text_ERStatG_reserve_size_t, std_vectorLcv_text_ERStatG_remove_size_t,
	std_vectorLcv_text_ERStatG_swap_size_t_size_t, std_vectorLcv_text_ERStatG_clear,
	std_vectorLcv_text_ERStatG_get_const_size_t, std_vectorLcv_text_ERStatG_set_size_t_const_ERStat,
	std_vectorLcv_text_ERStatG_push_const_ERStat, std_vectorLcv_text_ERStatG_insert_size_t_const_ERStat,
}


