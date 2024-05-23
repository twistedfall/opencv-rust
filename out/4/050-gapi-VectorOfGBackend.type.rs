impl core::Vector<crate::gapi::GBackend> {
	pub fn as_raw_VectorOfGBackend(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGBackend(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GBackend,
	std_vectorLcv_gapi_GBackendG_new_const, std_vectorLcv_gapi_GBackendG_delete,
	std_vectorLcv_gapi_GBackendG_len_const, std_vectorLcv_gapi_GBackendG_isEmpty_const,
	std_vectorLcv_gapi_GBackendG_capacity_const, std_vectorLcv_gapi_GBackendG_shrinkToFit,
	std_vectorLcv_gapi_GBackendG_reserve_size_t, std_vectorLcv_gapi_GBackendG_remove_size_t,
	std_vectorLcv_gapi_GBackendG_swap_size_t_size_t, std_vectorLcv_gapi_GBackendG_clear,
	std_vectorLcv_gapi_GBackendG_get_const_size_t, std_vectorLcv_gapi_GBackendG_set_size_t_const_GBackend,
	std_vectorLcv_gapi_GBackendG_push_const_GBackend, std_vectorLcv_gapi_GBackendG_insert_size_t_const_GBackend,
}

vector_non_copy_or_bool! { crate::gapi::GBackend }

vector_boxed_ref! { crate::gapi::GBackend }

vector_extern! { BoxedRef<'t, crate::gapi::GBackend>,
	std_vectorLcv_gapi_GBackendG_new_const, std_vectorLcv_gapi_GBackendG_delete,
	std_vectorLcv_gapi_GBackendG_len_const, std_vectorLcv_gapi_GBackendG_isEmpty_const,
	std_vectorLcv_gapi_GBackendG_capacity_const, std_vectorLcv_gapi_GBackendG_shrinkToFit,
	std_vectorLcv_gapi_GBackendG_reserve_size_t, std_vectorLcv_gapi_GBackendG_remove_size_t,
	std_vectorLcv_gapi_GBackendG_swap_size_t_size_t, std_vectorLcv_gapi_GBackendG_clear,
	std_vectorLcv_gapi_GBackendG_get_const_size_t, std_vectorLcv_gapi_GBackendG_set_size_t_const_GBackend,
	std_vectorLcv_gapi_GBackendG_push_const_GBackend, std_vectorLcv_gapi_GBackendG_insert_size_t_const_GBackend,
}


