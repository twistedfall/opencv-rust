impl core::Vector<crate::videoio::VideoCaptureAPIs> {
	pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::videoio::VideoCaptureAPIs,
	std_vectorLcv_VideoCaptureAPIsG_new_const, std_vectorLcv_VideoCaptureAPIsG_delete,
	std_vectorLcv_VideoCaptureAPIsG_len_const, std_vectorLcv_VideoCaptureAPIsG_isEmpty_const,
	std_vectorLcv_VideoCaptureAPIsG_capacity_const, std_vectorLcv_VideoCaptureAPIsG_shrinkToFit,
	std_vectorLcv_VideoCaptureAPIsG_reserve_size_t, std_vectorLcv_VideoCaptureAPIsG_remove_size_t,
	std_vectorLcv_VideoCaptureAPIsG_swap_size_t_size_t, std_vectorLcv_VideoCaptureAPIsG_clear,
	std_vectorLcv_VideoCaptureAPIsG_get_const_size_t, std_vectorLcv_VideoCaptureAPIsG_set_size_t_const_VideoCaptureAPIs,
	std_vectorLcv_VideoCaptureAPIsG_push_const_VideoCaptureAPIs, std_vectorLcv_VideoCaptureAPIsG_insert_size_t_const_VideoCaptureAPIs,
}

vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs,
	std_vectorLcv_VideoCaptureAPIsG_data_const, std_vectorLcv_VideoCaptureAPIsG_dataMut, cv_fromSlice_const_const_VideoCaptureAPIsX_size_t,
	std_vectorLcv_VideoCaptureAPIsG_clone_const,
}


