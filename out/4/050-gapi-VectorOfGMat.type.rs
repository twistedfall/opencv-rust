impl core::Vector<crate::gapi::GMat> {
	pub fn as_raw_VectorOfGMat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GMat,
	std_vectorLcv_GMatG_new_const, std_vectorLcv_GMatG_delete,
	std_vectorLcv_GMatG_len_const, std_vectorLcv_GMatG_isEmpty_const,
	std_vectorLcv_GMatG_capacity_const, std_vectorLcv_GMatG_shrinkToFit,
	std_vectorLcv_GMatG_reserve_size_t, std_vectorLcv_GMatG_remove_size_t,
	std_vectorLcv_GMatG_swap_size_t_size_t, std_vectorLcv_GMatG_clear,
	std_vectorLcv_GMatG_get_const_size_t, std_vectorLcv_GMatG_set_size_t_const_GMat,
	std_vectorLcv_GMatG_push_const_GMat, std_vectorLcv_GMatG_insert_size_t_const_GMat,
}

vector_non_copy_or_bool! { clone crate::gapi::GMat }

vector_boxed_ref! { crate::gapi::GMat }

vector_extern! { BoxedRef<'t, crate::gapi::GMat>,
	std_vectorLcv_GMatG_new_const, std_vectorLcv_GMatG_delete,
	std_vectorLcv_GMatG_len_const, std_vectorLcv_GMatG_isEmpty_const,
	std_vectorLcv_GMatG_capacity_const, std_vectorLcv_GMatG_shrinkToFit,
	std_vectorLcv_GMatG_reserve_size_t, std_vectorLcv_GMatG_remove_size_t,
	std_vectorLcv_GMatG_swap_size_t_size_t, std_vectorLcv_GMatG_clear,
	std_vectorLcv_GMatG_get_const_size_t, std_vectorLcv_GMatG_set_size_t_const_GMat,
	std_vectorLcv_GMatG_push_const_GMat, std_vectorLcv_GMatG_insert_size_t_const_GMat,
}


