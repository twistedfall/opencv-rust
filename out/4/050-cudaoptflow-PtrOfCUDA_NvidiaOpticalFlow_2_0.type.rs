ptr_extern! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0,
	cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_new_null_const, cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_delete, cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_getInnerPtr_const, cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_getInnerPtrMut
}

impl core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] pub fn as_raw_PtrOfCUDA_NvidiaOpticalFlow_2_0(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_NvidiaOpticalFlow_2_0(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0TraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_2_0(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0Trait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_2_0(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_to_PtrOfAlgorithm }

impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>, core::Ptr<crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow>, cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_to_PtrOfCUDA_NvidiaHWOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_NvidiaOpticalFlow_2_0")
			.finish()
	}
}

