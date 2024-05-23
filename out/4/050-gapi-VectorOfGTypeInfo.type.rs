impl core::Vector<crate::gapi::GTypeInfo> {
	pub fn as_raw_VectorOfGTypeInfo(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGTypeInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GTypeInfo,
	std_vectorLcv_GTypeInfoG_new_const, std_vectorLcv_GTypeInfoG_delete,
	std_vectorLcv_GTypeInfoG_len_const, std_vectorLcv_GTypeInfoG_isEmpty_const,
	std_vectorLcv_GTypeInfoG_capacity_const, std_vectorLcv_GTypeInfoG_shrinkToFit,
	std_vectorLcv_GTypeInfoG_reserve_size_t, std_vectorLcv_GTypeInfoG_remove_size_t,
	std_vectorLcv_GTypeInfoG_swap_size_t_size_t, std_vectorLcv_GTypeInfoG_clear,
	std_vectorLcv_GTypeInfoG_get_const_size_t, std_vectorLcv_GTypeInfoG_set_size_t_const_GTypeInfo,
	std_vectorLcv_GTypeInfoG_push_const_GTypeInfo, std_vectorLcv_GTypeInfoG_insert_size_t_const_GTypeInfo,
}

vector_non_copy_or_bool! { clone crate::gapi::GTypeInfo }

vector_boxed_ref! { crate::gapi::GTypeInfo }

vector_extern! { BoxedRef<'t, crate::gapi::GTypeInfo>,
	std_vectorLcv_GTypeInfoG_new_const, std_vectorLcv_GTypeInfoG_delete,
	std_vectorLcv_GTypeInfoG_len_const, std_vectorLcv_GTypeInfoG_isEmpty_const,
	std_vectorLcv_GTypeInfoG_capacity_const, std_vectorLcv_GTypeInfoG_shrinkToFit,
	std_vectorLcv_GTypeInfoG_reserve_size_t, std_vectorLcv_GTypeInfoG_remove_size_t,
	std_vectorLcv_GTypeInfoG_swap_size_t_size_t, std_vectorLcv_GTypeInfoG_clear,
	std_vectorLcv_GTypeInfoG_get_const_size_t, std_vectorLcv_GTypeInfoG_set_size_t_const_GTypeInfo,
	std_vectorLcv_GTypeInfoG_push_const_GTypeInfo, std_vectorLcv_GTypeInfoG_insert_size_t_const_GTypeInfo,
}


