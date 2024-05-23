impl core::Vector<crate::gapi::Detail_OpaqueKind> {
	pub fn as_raw_VectorOfDetail_OpaqueKind(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_OpaqueKind(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::Detail_OpaqueKind,
	std_vectorLcv_detail_OpaqueKindG_new_const, std_vectorLcv_detail_OpaqueKindG_delete,
	std_vectorLcv_detail_OpaqueKindG_len_const, std_vectorLcv_detail_OpaqueKindG_isEmpty_const,
	std_vectorLcv_detail_OpaqueKindG_capacity_const, std_vectorLcv_detail_OpaqueKindG_shrinkToFit,
	std_vectorLcv_detail_OpaqueKindG_reserve_size_t, std_vectorLcv_detail_OpaqueKindG_remove_size_t,
	std_vectorLcv_detail_OpaqueKindG_swap_size_t_size_t, std_vectorLcv_detail_OpaqueKindG_clear,
	std_vectorLcv_detail_OpaqueKindG_get_const_size_t, std_vectorLcv_detail_OpaqueKindG_set_size_t_const_OpaqueKind,
	std_vectorLcv_detail_OpaqueKindG_push_const_OpaqueKind, std_vectorLcv_detail_OpaqueKindG_insert_size_t_const_OpaqueKind,
}

vector_copy_non_bool! { crate::gapi::Detail_OpaqueKind,
	std_vectorLcv_detail_OpaqueKindG_data_const, std_vectorLcv_detail_OpaqueKindG_dataMut, cv_fromSlice_const_const_OpaqueKindX_size_t,
	std_vectorLcv_detail_OpaqueKindG_clone_const,
}


