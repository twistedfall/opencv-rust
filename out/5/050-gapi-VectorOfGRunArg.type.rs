impl core::Vector<crate::gapi::GRunArg> {
	pub fn as_raw_VectorOfGRunArg(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGRunArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GRunArg,
	std_vectorLcv_GRunArgG_new_const, std_vectorLcv_GRunArgG_delete,
	std_vectorLcv_GRunArgG_len_const, std_vectorLcv_GRunArgG_isEmpty_const,
	std_vectorLcv_GRunArgG_capacity_const, std_vectorLcv_GRunArgG_shrinkToFit,
	std_vectorLcv_GRunArgG_reserve_size_t, std_vectorLcv_GRunArgG_remove_size_t,
	std_vectorLcv_GRunArgG_swap_size_t_size_t, std_vectorLcv_GRunArgG_clear,
	std_vectorLcv_GRunArgG_get_const_size_t, std_vectorLcv_GRunArgG_set_size_t_const_GRunArg,
	std_vectorLcv_GRunArgG_push_const_GRunArg, std_vectorLcv_GRunArgG_insert_size_t_const_GRunArg,
}

vector_non_copy_or_bool! { crate::gapi::GRunArg }

vector_boxed_ref! { crate::gapi::GRunArg }

vector_extern! { BoxedRef<'t, crate::gapi::GRunArg>,
	std_vectorLcv_GRunArgG_new_const, std_vectorLcv_GRunArgG_delete,
	std_vectorLcv_GRunArgG_len_const, std_vectorLcv_GRunArgG_isEmpty_const,
	std_vectorLcv_GRunArgG_capacity_const, std_vectorLcv_GRunArgG_shrinkToFit,
	std_vectorLcv_GRunArgG_reserve_size_t, std_vectorLcv_GRunArgG_remove_size_t,
	std_vectorLcv_GRunArgG_swap_size_t_size_t, std_vectorLcv_GRunArgG_clear,
	std_vectorLcv_GRunArgG_get_const_size_t, std_vectorLcv_GRunArgG_set_size_t_const_GRunArg,
	std_vectorLcv_GRunArgG_push_const_GRunArg, std_vectorLcv_GRunArgG_insert_size_t_const_GRunArg,
}


