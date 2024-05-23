impl core::Vector<core::GpuMat> {
	pub fn as_raw_VectorOfGpuMat(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGpuMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::GpuMat,
	std_vectorLcv_cuda_GpuMatG_new_const, std_vectorLcv_cuda_GpuMatG_delete,
	std_vectorLcv_cuda_GpuMatG_len_const, std_vectorLcv_cuda_GpuMatG_isEmpty_const,
	std_vectorLcv_cuda_GpuMatG_capacity_const, std_vectorLcv_cuda_GpuMatG_shrinkToFit,
	std_vectorLcv_cuda_GpuMatG_reserve_size_t, std_vectorLcv_cuda_GpuMatG_remove_size_t,
	std_vectorLcv_cuda_GpuMatG_swap_size_t_size_t, std_vectorLcv_cuda_GpuMatG_clear,
	std_vectorLcv_cuda_GpuMatG_get_const_size_t, std_vectorLcv_cuda_GpuMatG_set_size_t_const_GpuMat,
	std_vectorLcv_cuda_GpuMatG_push_const_GpuMat, std_vectorLcv_cuda_GpuMatG_insert_size_t_const_GpuMat,
}

vector_non_copy_or_bool! { clone core::GpuMat }

vector_boxed_ref! { core::GpuMat }

vector_extern! { BoxedRef<'t, core::GpuMat>,
	std_vectorLcv_cuda_GpuMatG_new_const, std_vectorLcv_cuda_GpuMatG_delete,
	std_vectorLcv_cuda_GpuMatG_len_const, std_vectorLcv_cuda_GpuMatG_isEmpty_const,
	std_vectorLcv_cuda_GpuMatG_capacity_const, std_vectorLcv_cuda_GpuMatG_shrinkToFit,
	std_vectorLcv_cuda_GpuMatG_reserve_size_t, std_vectorLcv_cuda_GpuMatG_remove_size_t,
	std_vectorLcv_cuda_GpuMatG_swap_size_t_size_t, std_vectorLcv_cuda_GpuMatG_clear,
	std_vectorLcv_cuda_GpuMatG_get_const_size_t, std_vectorLcv_cuda_GpuMatG_set_size_t_const_GpuMat,
	std_vectorLcv_cuda_GpuMatG_push_const_GpuMat, std_vectorLcv_cuda_GpuMatG_insert_size_t_const_GpuMat,
}


