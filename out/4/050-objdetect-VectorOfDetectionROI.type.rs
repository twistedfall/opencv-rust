impl core::Vector<crate::objdetect::DetectionROI> {
	pub fn as_raw_VectorOfDetectionROI(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::objdetect::DetectionROI,
	std_vectorLcv_DetectionROIG_new_const, std_vectorLcv_DetectionROIG_delete,
	std_vectorLcv_DetectionROIG_len_const, std_vectorLcv_DetectionROIG_isEmpty_const,
	std_vectorLcv_DetectionROIG_capacity_const, std_vectorLcv_DetectionROIG_shrinkToFit,
	std_vectorLcv_DetectionROIG_reserve_size_t, std_vectorLcv_DetectionROIG_remove_size_t,
	std_vectorLcv_DetectionROIG_swap_size_t_size_t, std_vectorLcv_DetectionROIG_clear,
	std_vectorLcv_DetectionROIG_get_const_size_t, std_vectorLcv_DetectionROIG_set_size_t_const_DetectionROI,
	std_vectorLcv_DetectionROIG_push_const_DetectionROI, std_vectorLcv_DetectionROIG_insert_size_t_const_DetectionROI,
}

vector_non_copy_or_bool! { crate::objdetect::DetectionROI }

vector_boxed_ref! { crate::objdetect::DetectionROI }

vector_extern! { BoxedRef<'t, crate::objdetect::DetectionROI>,
	std_vectorLcv_DetectionROIG_new_const, std_vectorLcv_DetectionROIG_delete,
	std_vectorLcv_DetectionROIG_len_const, std_vectorLcv_DetectionROIG_isEmpty_const,
	std_vectorLcv_DetectionROIG_capacity_const, std_vectorLcv_DetectionROIG_shrinkToFit,
	std_vectorLcv_DetectionROIG_reserve_size_t, std_vectorLcv_DetectionROIG_remove_size_t,
	std_vectorLcv_DetectionROIG_swap_size_t_size_t, std_vectorLcv_DetectionROIG_clear,
	std_vectorLcv_DetectionROIG_get_const_size_t, std_vectorLcv_DetectionROIG_set_size_t_const_DetectionROI,
	std_vectorLcv_DetectionROIG_push_const_DetectionROI, std_vectorLcv_DetectionROIG_insert_size_t_const_DetectionROI,
}


