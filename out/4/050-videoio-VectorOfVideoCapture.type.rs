impl core::Vector<crate::videoio::VideoCapture> {
	pub fn as_raw_VectorOfVideoCapture(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVideoCapture(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::videoio::VideoCapture,
	std_vectorLcv_VideoCaptureG_new_const, std_vectorLcv_VideoCaptureG_delete,
	std_vectorLcv_VideoCaptureG_len_const, std_vectorLcv_VideoCaptureG_isEmpty_const,
	std_vectorLcv_VideoCaptureG_capacity_const, std_vectorLcv_VideoCaptureG_shrinkToFit,
	std_vectorLcv_VideoCaptureG_reserve_size_t, std_vectorLcv_VideoCaptureG_remove_size_t,
	std_vectorLcv_VideoCaptureG_swap_size_t_size_t, std_vectorLcv_VideoCaptureG_clear,
	std_vectorLcv_VideoCaptureG_get_const_size_t, std_vectorLcv_VideoCaptureG_set_size_t_const_VideoCapture,
	std_vectorLcv_VideoCaptureG_push_const_VideoCapture, std_vectorLcv_VideoCaptureG_insert_size_t_const_VideoCapture,
}

vector_non_copy_or_bool! { crate::videoio::VideoCapture }

vector_boxed_ref! { crate::videoio::VideoCapture }

vector_extern! { BoxedRef<'t, crate::videoio::VideoCapture>,
	std_vectorLcv_VideoCaptureG_new_const, std_vectorLcv_VideoCaptureG_delete,
	std_vectorLcv_VideoCaptureG_len_const, std_vectorLcv_VideoCaptureG_isEmpty_const,
	std_vectorLcv_VideoCaptureG_capacity_const, std_vectorLcv_VideoCaptureG_shrinkToFit,
	std_vectorLcv_VideoCaptureG_reserve_size_t, std_vectorLcv_VideoCaptureG_remove_size_t,
	std_vectorLcv_VideoCaptureG_swap_size_t_size_t, std_vectorLcv_VideoCaptureG_clear,
	std_vectorLcv_VideoCaptureG_get_const_size_t, std_vectorLcv_VideoCaptureG_set_size_t_const_VideoCapture,
	std_vectorLcv_VideoCaptureG_push_const_VideoCapture, std_vectorLcv_VideoCaptureG_insert_size_t_const_VideoCapture,
}


