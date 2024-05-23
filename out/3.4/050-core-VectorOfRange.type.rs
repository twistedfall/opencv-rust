impl core::Vector<core::Range> {
	pub fn as_raw_VectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Range,
	std_vectorLcv_RangeG_new_const, std_vectorLcv_RangeG_delete,
	std_vectorLcv_RangeG_len_const, std_vectorLcv_RangeG_isEmpty_const,
	std_vectorLcv_RangeG_capacity_const, std_vectorLcv_RangeG_shrinkToFit,
	std_vectorLcv_RangeG_reserve_size_t, std_vectorLcv_RangeG_remove_size_t,
	std_vectorLcv_RangeG_swap_size_t_size_t, std_vectorLcv_RangeG_clear,
	std_vectorLcv_RangeG_get_const_size_t, std_vectorLcv_RangeG_set_size_t_const_Range,
	std_vectorLcv_RangeG_push_const_Range, std_vectorLcv_RangeG_insert_size_t_const_Range,
}

vector_non_copy_or_bool! { core::Range }

vector_boxed_ref! { core::Range }

vector_extern! { BoxedRef<'t, core::Range>,
	std_vectorLcv_RangeG_new_const, std_vectorLcv_RangeG_delete,
	std_vectorLcv_RangeG_len_const, std_vectorLcv_RangeG_isEmpty_const,
	std_vectorLcv_RangeG_capacity_const, std_vectorLcv_RangeG_shrinkToFit,
	std_vectorLcv_RangeG_reserve_size_t, std_vectorLcv_RangeG_remove_size_t,
	std_vectorLcv_RangeG_swap_size_t_size_t, std_vectorLcv_RangeG_clear,
	std_vectorLcv_RangeG_get_const_size_t, std_vectorLcv_RangeG_set_size_t_const_Range,
	std_vectorLcv_RangeG_push_const_Range, std_vectorLcv_RangeG_insert_size_t_const_Range,
}


