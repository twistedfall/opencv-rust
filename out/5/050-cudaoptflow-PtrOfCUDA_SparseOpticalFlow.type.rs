ptr_extern! { crate::cudaoptflow::CUDA_SparseOpticalFlow,
	cv_PtrLcv_cuda_SparseOpticalFlowG_new_null_const, cv_PtrLcv_cuda_SparseOpticalFlowG_delete, cv_PtrLcv_cuda_SparseOpticalFlowG_getInnerPtr_const, cv_PtrLcv_cuda_SparseOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfCUDA_SparseOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_SparseOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow> {
	#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_SparseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow> {
	#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_SparseOpticalFlowG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_SparseOpticalFlow")
			.finish()
	}
}

