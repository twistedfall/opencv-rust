ptr_extern! { crate::cudabgsegm::CUDA_BackgroundSubtractorMOG,
	cv_PtrLcv_cuda_BackgroundSubtractorMOGG_new_null_const, cv_PtrLcv_cuda_BackgroundSubtractorMOGG_delete, cv_PtrLcv_cuda_BackgroundSubtractorMOGG_getInnerPtr_const, cv_PtrLcv_cuda_BackgroundSubtractorMOGG_getInnerPtrMut
}

impl core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_BackgroundSubtractorMOGG_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_cuda_BackgroundSubtractorMOGG_to_PtrOfBackgroundSubtractor }

impl std::fmt::Debug for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_BackgroundSubtractorMOG")
			.finish()
	}
}

