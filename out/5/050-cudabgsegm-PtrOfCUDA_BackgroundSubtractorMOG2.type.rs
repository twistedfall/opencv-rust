ptr_extern! { crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2,
	cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_new_null_const, cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_delete, cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_getInnerPtr_const, cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_getInnerPtrMut
}

impl core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG2(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2TraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2Trait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_to_PtrOfBackgroundSubtractor }

impl crate::video::BackgroundSubtractorMOG2TraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorMOG2Trait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>, core::Ptr<crate::video::BackgroundSubtractorMOG2>, cv_PtrLcv_cuda_BackgroundSubtractorMOG2G_to_PtrOfBackgroundSubtractorMOG2 }

impl std::fmt::Debug for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_BackgroundSubtractorMOG2")
			.finish()
	}
}

