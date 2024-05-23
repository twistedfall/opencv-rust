impl core::Vector<crate::gapi::GArg> {
	pub fn as_raw_VectorOfGArg(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GArg,
	std_vectorLcv_GArgG_new_const, std_vectorLcv_GArgG_delete,
	std_vectorLcv_GArgG_len_const, std_vectorLcv_GArgG_isEmpty_const,
	std_vectorLcv_GArgG_capacity_const, std_vectorLcv_GArgG_shrinkToFit,
	std_vectorLcv_GArgG_reserve_size_t, std_vectorLcv_GArgG_remove_size_t,
	std_vectorLcv_GArgG_swap_size_t_size_t, std_vectorLcv_GArgG_clear,
	std_vectorLcv_GArgG_get_const_size_t, std_vectorLcv_GArgG_set_size_t_const_GArg,
	std_vectorLcv_GArgG_push_const_GArg, std_vectorLcv_GArgG_insert_size_t_const_GArg,
}

vector_non_copy_or_bool! { crate::gapi::GArg }

vector_boxed_ref! { crate::gapi::GArg }

vector_extern! { BoxedRef<'t, crate::gapi::GArg>,
	std_vectorLcv_GArgG_new_const, std_vectorLcv_GArgG_delete,
	std_vectorLcv_GArgG_len_const, std_vectorLcv_GArgG_isEmpty_const,
	std_vectorLcv_GArgG_capacity_const, std_vectorLcv_GArgG_shrinkToFit,
	std_vectorLcv_GArgG_reserve_size_t, std_vectorLcv_GArgG_remove_size_t,
	std_vectorLcv_GArgG_swap_size_t_size_t, std_vectorLcv_GArgG_clear,
	std_vectorLcv_GArgG_get_const_size_t, std_vectorLcv_GArgG_set_size_t_const_GArg,
	std_vectorLcv_GArgG_push_const_GArg, std_vectorLcv_GArgG_insert_size_t_const_GArg,
}


