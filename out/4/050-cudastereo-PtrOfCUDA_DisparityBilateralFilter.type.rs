ptr_extern! { crate::cudastereo::CUDA_DisparityBilateralFilter,
	cv_PtrLcv_cuda_DisparityBilateralFilterG_new_null_const, cv_PtrLcv_cuda_DisparityBilateralFilterG_delete, cv_PtrLcv_cuda_DisparityBilateralFilterG_getInnerPtr_const, cv_PtrLcv_cuda_DisparityBilateralFilterG_getInnerPtrMut
}

impl core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
	#[inline] pub fn as_raw_PtrOfCUDA_DisparityBilateralFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_DisparityBilateralFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudastereo::CUDA_DisparityBilateralFilterTraitConst for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
	#[inline] fn as_raw_CUDA_DisparityBilateralFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudastereo::CUDA_DisparityBilateralFilterTrait for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
	#[inline] fn as_raw_mut_CUDA_DisparityBilateralFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_DisparityBilateralFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_DisparityBilateralFilter")
			.finish()
	}
}

