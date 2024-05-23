impl core::Vector<crate::ml::DTrees_Split> {
	pub fn as_raw_VectorOfDTrees_Split(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDTrees_Split(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::ml::DTrees_Split,
	std_vectorLcv_ml_DTrees_SplitG_new_const, std_vectorLcv_ml_DTrees_SplitG_delete,
	std_vectorLcv_ml_DTrees_SplitG_len_const, std_vectorLcv_ml_DTrees_SplitG_isEmpty_const,
	std_vectorLcv_ml_DTrees_SplitG_capacity_const, std_vectorLcv_ml_DTrees_SplitG_shrinkToFit,
	std_vectorLcv_ml_DTrees_SplitG_reserve_size_t, std_vectorLcv_ml_DTrees_SplitG_remove_size_t,
	std_vectorLcv_ml_DTrees_SplitG_swap_size_t_size_t, std_vectorLcv_ml_DTrees_SplitG_clear,
	std_vectorLcv_ml_DTrees_SplitG_get_const_size_t, std_vectorLcv_ml_DTrees_SplitG_set_size_t_const_Split,
	std_vectorLcv_ml_DTrees_SplitG_push_const_Split, std_vectorLcv_ml_DTrees_SplitG_insert_size_t_const_Split,
}

vector_non_copy_or_bool! { crate::ml::DTrees_Split }

vector_boxed_ref! { crate::ml::DTrees_Split }

vector_extern! { BoxedRef<'t, crate::ml::DTrees_Split>,
	std_vectorLcv_ml_DTrees_SplitG_new_const, std_vectorLcv_ml_DTrees_SplitG_delete,
	std_vectorLcv_ml_DTrees_SplitG_len_const, std_vectorLcv_ml_DTrees_SplitG_isEmpty_const,
	std_vectorLcv_ml_DTrees_SplitG_capacity_const, std_vectorLcv_ml_DTrees_SplitG_shrinkToFit,
	std_vectorLcv_ml_DTrees_SplitG_reserve_size_t, std_vectorLcv_ml_DTrees_SplitG_remove_size_t,
	std_vectorLcv_ml_DTrees_SplitG_swap_size_t_size_t, std_vectorLcv_ml_DTrees_SplitG_clear,
	std_vectorLcv_ml_DTrees_SplitG_get_const_size_t, std_vectorLcv_ml_DTrees_SplitG_set_size_t_const_Split,
	std_vectorLcv_ml_DTrees_SplitG_push_const_Split, std_vectorLcv_ml_DTrees_SplitG_insert_size_t_const_Split,
}


