impl core::Vector<crate::gapi::GTransform> {
	pub fn as_raw_VectorOfGTransform(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGTransform(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GTransform,
	std_vectorLcv_GTransformG_new_const, std_vectorLcv_GTransformG_delete,
	std_vectorLcv_GTransformG_len_const, std_vectorLcv_GTransformG_isEmpty_const,
	std_vectorLcv_GTransformG_capacity_const, std_vectorLcv_GTransformG_shrinkToFit,
	std_vectorLcv_GTransformG_reserve_size_t, std_vectorLcv_GTransformG_remove_size_t,
	std_vectorLcv_GTransformG_swap_size_t_size_t, std_vectorLcv_GTransformG_clear,
	std_vectorLcv_GTransformG_get_const_size_t, std_vectorLcv_GTransformG_set_size_t_const_GTransform,
	std_vectorLcv_GTransformG_push_const_GTransform, std_vectorLcv_GTransformG_insert_size_t_const_GTransform,
}

vector_non_copy_or_bool! { crate::gapi::GTransform }

vector_boxed_ref! { crate::gapi::GTransform }

vector_extern! { BoxedRef<'t, crate::gapi::GTransform>,
	std_vectorLcv_GTransformG_new_const, std_vectorLcv_GTransformG_delete,
	std_vectorLcv_GTransformG_len_const, std_vectorLcv_GTransformG_isEmpty_const,
	std_vectorLcv_GTransformG_capacity_const, std_vectorLcv_GTransformG_shrinkToFit,
	std_vectorLcv_GTransformG_reserve_size_t, std_vectorLcv_GTransformG_remove_size_t,
	std_vectorLcv_GTransformG_swap_size_t_size_t, std_vectorLcv_GTransformG_clear,
	std_vectorLcv_GTransformG_get_const_size_t, std_vectorLcv_GTransformG_set_size_t_const_GTransform,
	std_vectorLcv_GTransformG_push_const_GTransform, std_vectorLcv_GTransformG_insert_size_t_const_GTransform,
}


