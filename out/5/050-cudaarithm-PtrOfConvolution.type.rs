ptr_extern! { crate::cudaarithm::Convolution,
	cv_PtrLcv_cuda_ConvolutionG_new_null_const, cv_PtrLcv_cuda_ConvolutionG_delete, cv_PtrLcv_cuda_ConvolutionG_getInnerPtr_const, cv_PtrLcv_cuda_ConvolutionG_getInnerPtrMut
}

impl core::Ptr<crate::cudaarithm::Convolution> {
	#[inline] pub fn as_raw_PtrOfConvolution(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConvolution(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaarithm::ConvolutionTraitConst for core::Ptr<crate::cudaarithm::Convolution> {
	#[inline] fn as_raw_Convolution(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaarithm::ConvolutionTrait for core::Ptr<crate::cudaarithm::Convolution> {
	#[inline] fn as_raw_mut_Convolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaarithm::Convolution> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaarithm::Convolution> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaarithm::Convolution>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_ConvolutionG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::cudaarithm::Convolution> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConvolution")
			.finish()
	}
}

