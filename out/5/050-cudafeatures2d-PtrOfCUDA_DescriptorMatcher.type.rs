ptr_extern! { crate::cudafeatures2d::CUDA_DescriptorMatcher,
	cv_PtrLcv_cuda_DescriptorMatcherG_new_null_const, cv_PtrLcv_cuda_DescriptorMatcherG_delete, cv_PtrLcv_cuda_DescriptorMatcherG_getInnerPtr_const, cv_PtrLcv_cuda_DescriptorMatcherG_getInnerPtrMut
}

impl core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
	#[inline] pub fn as_raw_PtrOfCUDA_DescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_DescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudafeatures2d::CUDA_DescriptorMatcherTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
	#[inline] fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudafeatures2d::CUDA_DescriptorMatcherTrait for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
	#[inline] fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_DescriptorMatcherG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_DescriptorMatcher")
			.finish()
	}
}

