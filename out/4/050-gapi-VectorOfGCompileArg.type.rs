impl core::Vector<crate::gapi::GCompileArg> {
	pub fn as_raw_VectorOfGCompileArg(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGCompileArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GCompileArg,
	std_vectorLcv_GCompileArgG_new_const, std_vectorLcv_GCompileArgG_delete,
	std_vectorLcv_GCompileArgG_len_const, std_vectorLcv_GCompileArgG_isEmpty_const,
	std_vectorLcv_GCompileArgG_capacity_const, std_vectorLcv_GCompileArgG_shrinkToFit,
	std_vectorLcv_GCompileArgG_reserve_size_t, std_vectorLcv_GCompileArgG_remove_size_t,
	std_vectorLcv_GCompileArgG_swap_size_t_size_t, std_vectorLcv_GCompileArgG_clear,
	std_vectorLcv_GCompileArgG_get_const_size_t, std_vectorLcv_GCompileArgG_set_size_t_const_GCompileArg,
	std_vectorLcv_GCompileArgG_push_const_GCompileArg, std_vectorLcv_GCompileArgG_insert_size_t_const_GCompileArg,
}

vector_non_copy_or_bool! { crate::gapi::GCompileArg }

vector_boxed_ref! { crate::gapi::GCompileArg }

vector_extern! { BoxedRef<'t, crate::gapi::GCompileArg>,
	std_vectorLcv_GCompileArgG_new_const, std_vectorLcv_GCompileArgG_delete,
	std_vectorLcv_GCompileArgG_len_const, std_vectorLcv_GCompileArgG_isEmpty_const,
	std_vectorLcv_GCompileArgG_capacity_const, std_vectorLcv_GCompileArgG_shrinkToFit,
	std_vectorLcv_GCompileArgG_reserve_size_t, std_vectorLcv_GCompileArgG_remove_size_t,
	std_vectorLcv_GCompileArgG_swap_size_t_size_t, std_vectorLcv_GCompileArgG_clear,
	std_vectorLcv_GCompileArgG_get_const_size_t, std_vectorLcv_GCompileArgG_set_size_t_const_GCompileArg,
	std_vectorLcv_GCompileArgG_push_const_GCompileArg, std_vectorLcv_GCompileArgG_insert_size_t_const_GCompileArg,
}


