ptr_extern! { crate::cudaoptflow::CUDA_BroxOpticalFlow,
	cv_PtrLcv_cuda_BroxOpticalFlowG_new_null_const, cv_PtrLcv_cuda_BroxOpticalFlowG_delete, cv_PtrLcv_cuda_BroxOpticalFlowG_getInnerPtr_const, cv_PtrLcv_cuda_BroxOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfCUDA_BroxOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_BroxOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaoptflow::CUDA_BroxOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_BroxOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_BroxOpticalFlowG_to_PtrOfAlgorithm }

impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow>, core::Ptr<crate::cudaoptflow::CUDA_DenseOpticalFlow>, cv_PtrLcv_cuda_BroxOpticalFlowG_to_PtrOfCUDA_DenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_BroxOpticalFlow")
			.finish()
	}
}

