ptr_extern! { crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow,
	cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_new_null_const, cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_delete, cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_getInnerPtr_const, cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfCUDA_NvidiaHWOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_NvidiaHWOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow> {
	#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow> {
	#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_NvidiaHWOpticalFlow")
			.finish()
	}
}

